use std::net::SocketAddr;

use bdk_floresta::TransportProtocol;
use bitcoin::p2p::ServiceFlags;
use iced::Alignment::Center;
use iced::Element;
use iced::Length;
use iced::alignment::Horizontal::Left;
use iced::widget::Container;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::image;
use iced::widget::row;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::tooltip;

use crate::common::interface::constants::CELL_HEIGHT;
use crate::common::interface::constants::TABLE_CELL_FONT_SIZE;
use crate::common::interface::constants::TABLE_CELL_ICON_SIZE;
use crate::common::interface::constants::TABLE_HEADER_FONT_SIZE;
use crate::node::control::NodeStatus;
use crate::node::geoip::GeoIpReader;
use crate::node::message::NodeMessage;
use crate::node::network::style::network_button;
use crate::node::network::style::peer_info_table_container;
use crate::node::stats_fetcher::NodeImpl;
use crate::node::stats_fetcher::NodeStatistics;
use crate::node::style::input_field;
use crate::node::style::table_cell;
use crate::node::style::title_container;

fn get_address_with_asn_tooltip<'a>(
    socket: &'a SocketAddr,
    geoip_reader: &'a Option<GeoIpReader>,
) -> Element<'a, NodeMessage> {
    let not_found = String::from("NO INFORMATION\nAVAILABLE FOR THIS ADDRESS");
    let tooltip_text = if let Some(reader) = geoip_reader {
        let geo_info = reader.lookup_all(socket.ip());
        let mut lines = Vec::new();

        if let Some(asn) = &geo_info.asn {
            lines.push(format!("AS{}", asn.number));
            lines.push(asn.organization.clone());
        }

        if let Some(city) = &geo_info.city {
            lines.push(city.to_string());
        }

        if lines.is_empty() {
            not_found
        } else {
            lines.join("\n")
        }
    } else {
        not_found
    };

    tooltip(
        text(socket.to_string()).size(TABLE_CELL_FONT_SIZE),
        text(tooltip_text),
        tooltip::Position::FollowCursor,
    )
    .style(container::rounded_box)
    .into()
}

fn get_impl_icon<'a>(node_impl: &'a NodeImpl, user_agent: &'a str) -> Element<'a, NodeMessage> {
    let icon_path = match node_impl {
        NodeImpl::Core => "assets/icon/implementations/core.png",
        NodeImpl::Knots => "assets/icon/implementations/knots.png",
        NodeImpl::Btcd => "assets/icon/implementations/btcd.png",
        NodeImpl::Utreexod => "assets/icon/implementations/utreexod.jpeg",
        NodeImpl::Floresta => "assets/icon/implementations/floresta.png",
        NodeImpl::Unknown => "assets/icon/implementations/unknown.png",
    };

    let content = row![
        image(icon_path)
            .height(TABLE_CELL_ICON_SIZE)
            .width(TABLE_CELL_ICON_SIZE),
        text(node_impl.to_string()).size(TABLE_CELL_FONT_SIZE)
    ]
    .spacing(5)
    .align_y(Center);

    tooltip(content, user_agent, tooltip::Position::FollowCursor)
        .style(container::rounded_box)
        .into()
}

fn get_transport_with_tooltip<'a>(transport: &'a TransportProtocol) -> Element<'a, NodeMessage> {
    let (display_text, tooltip_text) = match transport {
        TransportProtocol::V1 => (
            "P2PV1",
            "Network messages between you and\nthis peer are not encrypted",
        ),
        TransportProtocol::V2 => (
            "P2PV2",
            "Network messages between you and this\npeer are encrypted using ChaCha20Poly1305",
        ),
    };

    tooltip(
        text(display_text).size(TABLE_CELL_FONT_SIZE),
        text(tooltip_text),
        tooltip::Position::FollowCursor,
    )
    .style(container::rounded_box)
    .into()
}

fn get_services_with_tooltip<'a>(services: &'a ServiceFlags) -> Element<'a, NodeMessage> {
    let services_hex = format!("0x{:016x}", services.to_u64());

    const ALL_FLAGS: &[(ServiceFlags, &str, u8)] = &[
        (ServiceFlags::NETWORK, "NETWORK", 0),
        (ServiceFlags::GETUTXO, "GETUTXO", 1),
        (ServiceFlags::BLOOM, "BLOOM", 2),
        (ServiceFlags::WITNESS, "WITNESS", 3),
        (ServiceFlags::COMPACT_FILTERS, "COMPACT_FILTERS", 6),
        (ServiceFlags::NETWORK_LIMITED, "NETWORK_LIMITED", 10),
        (ServiceFlags::P2P_V2, "P2P_V2", 11),
    ];

    let mut service_strings: Vec<String> = ALL_FLAGS
        .iter()
        .filter(|(flag, _, _)| services.has(*flag))
        .map(|(_, name, _bit)| name.to_string())
        .collect();

    // Check for non-spec flags manually
    const NODE_UTREEXO: u64 = 1 << 24;
    const NODE_UTREEXO_ARCHIVE: u64 = 1 << 25;

    if services.to_u64() & NODE_UTREEXO != 0 {
        service_strings.push("NODE_UTREEXO".to_string());
    }
    if services.to_u64() & NODE_UTREEXO_ARCHIVE != 0 {
        service_strings.push("NODE_UTREEXO_ARCHIVE".to_string());
    }

    let services_text = if service_strings.is_empty() {
        "NONE".to_string()
    } else {
        service_strings.join("\n")
    };

    container(
        tooltip(
            text(services_hex).size(TABLE_CELL_FONT_SIZE - 4),
            text(services_text),
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box),
    )
    .align_x(Left)
    .align_y(Center)
    .into()
}

pub fn view_p2p<'a>(
    _status: &'a NodeStatus,
    statistics: &'a Option<NodeStatistics>,
    peer_input: &'a str,
    geoip_reader: &'a Option<GeoIpReader>,
) -> Element<'a, NodeMessage> {
    // Add Peer.
    let add_peer_title: Container<'_, NodeMessage> = container(text("ADD PEER").size(21));
    let add_peer_input = container(
        text_input("123.123.123.123:38333", peer_input)
            .on_input(NodeMessage::AddPeerInputChanged)
            .style(input_field())
            .size(12)
            .padding(10),
    )
    .align_y(Center)
    .height(Length::Fill);
    let add_peer_button = container(
        button(text("CONNECT").size(12))
            .on_press(NodeMessage::AddPeer)
            .style(network_button())
            .padding(10),
    )
    .align_y(Center)
    .height(Length::Fill);
    let add_peer_container = container(
        row![add_peer_input, add_peer_button]
            .spacing(10)
            .height(50)
            .align_y(Center),
    )
    .style(title_container())
    .padding(10);
    let add_peer =
        container(column![add_peer_title, add_peer_container]).height(Length::Fixed(100.0));

    // P2P Messages (TODO: requires a node hook for P2P messages)
    let p2p_messages_title: Container<'_, NodeMessage> = container(text("P2P MESSAGES").size(21));
    let p2p_messages_container = container(row![text(
        "WIP: Requires a node hook for P2P messages on Floresta"
    )])
    .style(title_container())
    .padding(10)
    .height(Length::Fill)
    .width(Length::Fill);
    let p2p_messages = column![p2p_messages_title, p2p_messages_container];

    // Left Section.
    let left = column![add_peer, p2p_messages]
        .spacing(25)
        .width(Length::FillPortion(3));

    // Peer Info.
    let peer_info_title = container(text("PEERS").size(21));

    let mut peer_info_table = column![].spacing(0);
    peer_info_table = peer_info_table.push(row![
        container(text("SOCKET").size(TABLE_HEADER_FONT_SIZE))
            .width(Length::FillPortion(16))
            .height(CELL_HEIGHT)
            .align_x(Center)
            .align_y(Center)
            .style(table_cell()),
        container(text("IMPLEMENTATION").size(TABLE_HEADER_FONT_SIZE))
            .width(Length::FillPortion(16))
            .height(CELL_HEIGHT)
            .align_x(Center)
            .align_y(Center)
            .style(table_cell()),
        container(text("SERVICES").size(TABLE_HEADER_FONT_SIZE))
            .width(Length::FillPortion(10))
            .height(CELL_HEIGHT)
            .align_x(Center)
            .align_y(Center)
            .style(table_cell()),
        container(text("TRANSPORT").size(TABLE_HEADER_FONT_SIZE - 4))
            .width(Length::FillPortion(8))
            .height(CELL_HEIGHT)
            .align_x(Center)
            .align_y(Center)
            .style(table_cell()),
        container(text("ACTION").size(TABLE_HEADER_FONT_SIZE))
            .width(Length::FillPortion(10))
            .height(CELL_HEIGHT)
            .align_x(Center)
            .align_y(Center)
            .style(table_cell()),
    ]);

    // Get peer list.
    let peers = statistics
        .as_ref()
        .map(|s| s.peer_informations.as_slice())
        .unwrap_or(&[]);

    const NUM_ROWS: usize = 16;
    for i in 0..NUM_ROWS {
        if let Some(peer) = peers.get(i) {
            let disconnect_button = button(text("DISCONNECT").size(10))
                .on_press(NodeMessage::DisconnectPeer(peer.socket))
                .style(network_button())
                .padding(5);

            // TODO: add peer banning logic on `floresta-wire`.
            let ban_button = button(text("BAN").size(10))
                .on_press(NodeMessage::Tick)
                .style(network_button())
                .padding(5);

            peer_info_table = peer_info_table.push(row![
                container(get_address_with_asn_tooltip(&peer.socket, geoip_reader))
                    .padding(10)
                    .height(CELL_HEIGHT)
                    .width(Length::FillPortion(16))
                    .style(table_cell()),
                container(get_impl_icon(&peer.node_impl, &peer.user_agent))
                    .padding(10)
                    .height(CELL_HEIGHT)
                    .width(Length::FillPortion(16))
                    .style(table_cell())
                    .align_y(Center),
                container(get_services_with_tooltip(&peer.services))
                    .padding(10)
                    .height(CELL_HEIGHT)
                    .width(Length::FillPortion(10))
                    .align_y(Center)
                    .style(table_cell()),
                container(get_transport_with_tooltip(&peer.transport_protocol))
                    .padding(10)
                    .height(CELL_HEIGHT)
                    .width(Length::FillPortion(8))
                    .style(table_cell())
                    .align_x(Center)
                    .align_y(Center),
                container(row![ban_button, disconnect_button].spacing(4))
                    .padding(2)
                    .height(CELL_HEIGHT)
                    .width(Length::FillPortion(10))
                    .style(table_cell())
                    .align_x(Center)
                    .align_y(Center),
            ]);
        } else {
            peer_info_table = peer_info_table.push(row![
                container(text("").size(12))
                    .padding(10)
                    .width(Length::FillPortion(16))
                    .style(table_cell()),
                container(text("").size(12))
                    .padding(10)
                    .width(Length::FillPortion(16))
                    .style(table_cell()),
                container(text("").size(12))
                    .padding(10)
                    .width(Length::FillPortion(10))
                    .style(table_cell()),
                container(text("").size(12))
                    .padding(10)
                    .width(Length::FillPortion(8))
                    .style(table_cell()),
                container(text("").size(12))
                    .padding(10)
                    .width(Length::FillPortion(10))
                    .style(table_cell()),
            ]);
        }
    }
    let peer_info_table = container(peer_info_table).style(peer_info_table_container());

    let right = column![peer_info_title, peer_info_table]
        .spacing(5)
        .width(Length::FillPortion(7));

    row![left, right].spacing(20).into()
}
