use iced::widget::{Container, button, column, container, row, text, text_input};
use iced::{Element, Length};

use crate::node::control::NodeStatus;
use crate::node::interface::common::{TITLE_PADDING, input_field, table_cell, title_container};
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;

const CELL_HEIGHT: Length = Length::Fixed(35.0);
const TABLE_HEADER_FONT_SIZE: u32 = 16;
const TABLE_CELL_FONT_SIZE: u32 = 12;

pub fn view_p2p<'a>(
    _status: &'a NodeStatus,
    statistics: &'a Option<NodeStatistics>,
    peer_input: &'a str,
) -> Element<'a, NodeMessage> {
    let title: Container<'_, NodeMessage> = container(text("NODE P2P").size(25))
        .style(title_container())
        .padding(TITLE_PADDING);

    let add_peer_field = container(
        text_input("69.69.69.69:38333", peer_input)
            .on_input(NodeMessage::AddPeerInputChanged)
            .style(input_field())
            .padding(10),
    )
    .height(40);

    let add_button = button(text("CONNECT"))
        .on_press(NodeMessage::AddPeer)
        .padding(10)
        .height(40);

    let peer_row = row![add_peer_field, add_button].spacing(10);

    let left_content_box = container(column![])
        .style(table_cell())
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill);

    // Left section
    let left = column![title, text("").size(10), peer_row, left_content_box,]
        .spacing(10)
        .width(Length::FillPortion(7));

    // Build peer table for right section
    let mut peer_table = column![].spacing(0);

    // Always add header row with all fields
    peer_table = peer_table.push(row![
        container(text("ADDRESS").size(TABLE_HEADER_FONT_SIZE))
            .padding(10)
            .width(Length::FillPortion(2))
            .style(table_cell()),
        container(text("USER AGENT").size(TABLE_HEADER_FONT_SIZE))
            .padding(10)
            .width(Length::FillPortion(2))
            .style(table_cell()),
        container(text("IMPL").size(TABLE_HEADER_FONT_SIZE))
            .padding(10)
            .width(Length::FillPortion(1))
            .style(table_cell()),
        container(text("HEIGHT").size(TABLE_HEADER_FONT_SIZE))
            .padding(10)
            .width(Length::FillPortion(1))
            .style(table_cell()),
    ]);

    // Get peer list
    let peers = statistics
        .as_ref()
        .map(|s| s.peer_informations.as_slice())
        .unwrap_or(&[]);

    // Always render 10 rows
    const NUM_ROWS: usize = 14;
    for i in 0..NUM_ROWS {
        if let Some(peer) = peers.get(i) {
            // Render peer data
            peer_table = peer_table.push(row![
                container(text(&peer.address).size(TABLE_CELL_FONT_SIZE))
                    .padding(10)
                    .height(CELL_HEIGHT)
                    .width(Length::FillPortion(2))
                    .style(table_cell()),
                container(text(&peer.user_agent).size(TABLE_CELL_FONT_SIZE))
                    .padding(10)
                    .height(CELL_HEIGHT)
                    .width(Length::FillPortion(2))
                    .style(table_cell()),
                container(text(format!("{:?}", peer.implementation)).size(TABLE_CELL_FONT_SIZE))
                    .padding(10)
                    .height(CELL_HEIGHT)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(peer.initial_height).size(TABLE_CELL_FONT_SIZE))
                    .padding(10)
                    .height(CELL_HEIGHT)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ]);
        } else {
            // Render empty row
            peer_table = peer_table.push(row![
                container(text("").size(12))
                    .padding(10)
                    .width(Length::FillPortion(2))
                    .style(table_cell()),
                container(text("").size(12))
                    .padding(10)
                    .width(Length::FillPortion(2))
                    .style(table_cell()),
                container(text("").size(12))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text("").size(12))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ]);
        }
    }

    // Right section with peer table
    let right = column![peer_table]
        .spacing(10)
        .width(Length::FillPortion(13));

    row![left, right].spacing(20).into()
}
