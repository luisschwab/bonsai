use std::net::SocketAddr;

use bitcoin::Network;
use iced::Alignment::Center;
use iced::Background::Color as BackgroundColor;
use iced::Border;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Length::Fill;
use iced::Length::FillPortion;
use iced::Theme;
use iced::border::Radius;
use iced::theme::palette::Pair;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::column;
use iced::widget::container;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::row;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::tooltip;

use crate::common::interface::color::BLACK;
use crate::common::interface::color::BLUE;
use crate::common::interface::color::GREEN_SHAMROCK;
use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::color::ORANGE;
use crate::common::interface::color::PURPLE;
use crate::common::interface::color::RED;
use crate::common::interface::color::YELLOW;
use crate::common::interface::constants::BORDER_RADIUS;
use crate::common::interface::constants::BORDER_WIDTH;
use crate::common::interface::constants::TABLE_CELL_FONT_SIZE;
use crate::common::interface::container::button_container;
use crate::common::interface::shadow::SHADOW_GRAY;
use crate::common::interface::shadow::SHADOW_RED;
use crate::node::style::title_container;
use crate::node::style::title_container_red;
use crate::settings::bonsai_settings::AUTO_START_NODE;
use crate::settings::bonsai_settings::BonsaiSettings;
use crate::settings::bonsai_settings::BonsaiSettingsMessage;

const SECTION_BOX_HEIGHT: f32 = 30.0;

fn section_title<'a>(label: &'static str, size: u32) -> Container<'a, BonsaiSettingsMessage> {
    container(text(label).size(size))
}

fn boolean_section<'a>(
    title: &'static str,
    active_value: bool,
    true_message: BonsaiSettingsMessage,
    false_message: BonsaiSettingsMessage,
) -> Column<'a, BonsaiSettingsMessage> {
    let title = section_title(title, 21);
    let buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                active_value,
                GREEN_SHAMROCK,
                true_message
            ),
            boolean_button_with_disable_logic("FALSE", false, active_value, RED, false_message),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);

    column![title, buttons]
}

fn network_section<'a>(active_network: Network) -> Column<'a, BonsaiSettingsMessage> {
    let title = section_title("NETWORK", 22);
    let buttons = container(
        row![
            network_button_with_disable_logic("BITCOIN", Network::Bitcoin, active_network, ORANGE),
            network_button_with_disable_logic("SIGNET", Network::Signet, active_network, PURPLE),
            tooltip(
                network_button_with_disable_logic(
                    "TESTNET4",
                    Network::Testnet4,
                    active_network,
                    BLUE
                ),
                text("No `Network::Testnet4` bridges are available yet").size(TABLE_CELL_FONT_SIZE),
                tooltip::Position::FollowCursor
            )
            .style(container::rounded_box),
            network_button_with_disable_logic("REGTEST", Network::Regtest, active_network, YELLOW),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);

    column![title, buttons]
}

fn text_input_section<'a>(
    title: &'static str,
    placeholder: String,
    value: &'a str,
    on_input: fn(String) -> BonsaiSettingsMessage,
    error: Option<&'a str>,
) -> Column<'a, BonsaiSettingsMessage> {
    let input = container(
        text_input(&placeholder, value)
            .on_input(on_input)
            .padding(10)
            .width(Fill),
    )
    .style(title_container())
    .padding(1);

    column![
        section_title(title, 21),
        input,
        text(error.unwrap_or("")).size(12).color(RED)
    ]
}

fn max_banscore_section<'a>(max_banscore: u32) -> Column<'a, BonsaiSettingsMessage> {
    let controls = container(
        row![
            container(
                text(max_banscore.to_string())
                    .align_x(Center)
                    .align_y(Center)
                    .size(16)
            )
            .padding(10)
            .width(FillPortion(2))
            .align_x(Center)
            .align_y(Center)
            .style(table_cell_with_shadow()),
            button(text("-").size(16).align_x(Center).align_y(Center))
                .on_press_maybe(if max_banscore > 0 {
                    Some(BonsaiSettingsMessage::MaxBanscoreChanged(
                        (max_banscore - 1).to_string(),
                    ))
                } else {
                    None
                })
                .width(FillPortion(1))
                .style(button_container()),
            button(text("+").size(16).align_x(Center).align_y(Center))
                .on_press_maybe(if max_banscore < 1000 {
                    Some(BonsaiSettingsMessage::MaxBanscoreChanged(
                        (max_banscore + 1).to_string(),
                    ))
                } else {
                    None
                })
                .width(FillPortion(1))
                .style(button_container()),
        ]
        .spacing(10)
        .height(Length::Fixed(SECTION_BOX_HEIGHT)),
    )
    .style(title_container())
    .padding(10);

    column![section_title("MAX BAN SCORE", 21), controls]
}

fn action_button_row<'a>(
    status_text: &'static str,
    is_active: bool,
    label: &'static str,
    message: BonsaiSettingsMessage,
) -> iced::widget::Row<'a, BonsaiSettingsMessage> {
    row![
        text(if is_active { status_text } else { "" })
            .size(12)
            .color(if is_active { ORANGE } else { GREEN_SHAMROCK }),
        Space::new().width(Fill),
        button(text(label).size(20).align_x(Center).align_y(Center))
            .on_press_maybe(if is_active { Some(message) } else { None })
            .style(button_container())
            .width(Length::Fixed(220.0))
            .height(Length::Fixed(50.0))
    ]
    .spacing(10)
    .align_y(Center)
}

fn actions_section(settings: &BonsaiSettings) -> Column<'_, BonsaiSettingsMessage> {
    let save_button_row = action_button_row(
        "UNSAVED CHANGES",
        settings.unsaved_changes,
        "SAVE SETTINGS",
        BonsaiSettingsMessage::SaveSettings,
    );
    let restart_button_row = action_button_row(
        "CHANGED SETTINGS\nREQUIRE A NODE RESTART",
        settings.node_restart_required,
        "RESTART NODE",
        BonsaiSettingsMessage::RestartNode,
    );

    let container = container(column![save_button_row, restart_button_row].spacing(20))
        .padding(15)
        .style(title_container())
        .width(Fill);

    column![section_title("SAVE CHANGES & RESTART", 21), container]
}

fn danger_section(settings: &BonsaiSettings) -> Column<'_, BonsaiSettingsMessage> {
    let confirm_delete = settings.delete_node_data_confirm;
    let button_label = if confirm_delete {
        "CONFIRM DELETE"
    } else {
        "DELETE NODE DATA"
    };
    let delete_message = if confirm_delete {
        BonsaiSettingsMessage::ConfirmDeleteNodeData
    } else {
        BonsaiSettingsMessage::RequestDeleteNodeData
    };
    let delete_data_row = row![
        text(if confirm_delete {
            "CLICK CONFIRM TO DELETE\nACTIVE NETWORK NODE DATA\nTHIS CANNOT BE UNDONE"
        } else {
            "THIS ACTION IS DESTRUCTIVE\nALL VALIDATION WORK FOR\nTHIS NETWORK WILL BE LOST"
        })
        .size(12)
        .color(RED),
        Space::new().width(Fill),
        button(
            text(button_label)
                .color(RED)
                .size(20)
                .align_x(Center)
                .align_y(Center)
        )
        .on_press(delete_message)
        .style(delete_button_container())
        .width(Length::Fixed(220.0))
        .height(Length::Fixed(50.0))
    ];
    let cancel_row = row![
        Space::new().width(Fill),
        button(text("CANCEL").size(16).align_x(Center).align_y(Center))
            .on_press(BonsaiSettingsMessage::CancelDeleteNodeData)
            .style(button_container())
            .width(Length::Fixed(220.0))
            .height(Length::Fixed(36.0))
    ];
    let status_text = text(
        settings
            .delete_node_data_error
            .as_deref()
            .or(settings.delete_node_data_status.as_deref())
            .unwrap_or(""),
    )
    .size(12)
    .color(if settings.delete_node_data_error.is_some() {
        RED
    } else {
        GREEN_SHAMROCK
    });
    let danger_content = if confirm_delete {
        column![delete_data_row, cancel_row, status_text].spacing(10)
    } else {
        column![delete_data_row, status_text].spacing(10)
    };
    let danger_container = container(danger_content)
        .padding(15)
        .style(title_container_red())
        .width(Fill);
    let danger_title: Container<'_, BonsaiSettingsMessage> =
        container(text("DANGER ZONE").size(21).color(RED));

    column![danger_title, danger_container]
}

fn socket_placeholder(socket: Option<SocketAddr>) -> String {
    socket.map_or("NULL".to_string(), |addr| addr.to_string())
}

pub(crate) fn view_settings(settings: &BonsaiSettings) -> Element<'_, BonsaiSettingsMessage> {
    let auto_start = settings.node.auto_start.unwrap_or(AUTO_START_NODE);
    let active_network = settings.active_network();

    let node_config = settings.node.get_network_config(active_network);

    let use_assume_utreexo = node_config.use_assume_utreexo.unwrap_or(true);
    let use_powfps = node_config.enable_powfps.unwrap_or(true);
    let backfill = node_config.perform_backfill.unwrap_or(true);
    let allow_v1_fallback = node_config.allow_p2pv1_fallback.unwrap_or(true);
    let disable_dns_seeds = node_config.disable_dns_seeds.unwrap_or(false);
    let user_agent = node_config.user_agent.clone();
    let max_banscore = node_config.max_banscore.unwrap_or_default();

    let left = column![
        network_section(active_network),
        Space::new().height(Length::Fill),
        boolean_section(
            "AUTO START NODE",
            auto_start,
            BonsaiSettingsMessage::AutoStartChanged(true),
            BonsaiSettingsMessage::AutoStartChanged(false),
        ),
        Space::new().height(Length::Fill),
        boolean_section(
            "ASSUME UTREEXO",
            use_assume_utreexo,
            BonsaiSettingsMessage::UseAssumeUtreexoChanged(true),
            BonsaiSettingsMessage::UseAssumeUtreexoChanged(false),
        ),
        Space::new().height(Length::Fill),
        boolean_section(
            "PROOF-OF-WORK FRAUD PROOFS",
            use_powfps,
            BonsaiSettingsMessage::PowFraudProofsChanged(true),
            BonsaiSettingsMessage::PowFraudProofsChanged(false),
        ),
        Space::new().height(Length::Fill),
        boolean_section(
            "BACKFILL",
            backfill,
            BonsaiSettingsMessage::BackfillChanged(true),
            BonsaiSettingsMessage::BackfillChanged(false),
        ),
        Space::new().height(Length::Fill),
        boolean_section(
            "ALLOW V1 FALLBACK",
            allow_v1_fallback,
            BonsaiSettingsMessage::AllowV1FallbackChanged(true),
            BonsaiSettingsMessage::AllowV1FallbackChanged(false),
        ),
        Space::new().height(Length::Fill),
        boolean_section(
            "DISABLE DNS SEEDS",
            disable_dns_seeds,
            BonsaiSettingsMessage::DisableDnsSeedsChanged(true),
            BonsaiSettingsMessage::DisableDnsSeedsChanged(false),
        ),
    ]
    .width(FillPortion(1));

    let right = column![
        text_input_section(
            "USER AGENT",
            user_agent.unwrap_or_else(|| "NULL".to_string()),
            &settings.user_agent_input,
            BonsaiSettingsMessage::UserAgentInputChanged,
            None,
        ),
        text_input_section(
            "PROXY",
            socket_placeholder(node_config.socks5_proxy),
            &settings.proxy_input,
            BonsaiSettingsMessage::ProxyInputChanged,
            settings.proxy_error.as_deref(),
        ),
        text_input_section(
            "FIXED PEER",
            socket_placeholder(node_config.fixed_peer),
            &settings.fixed_peer_input,
            BonsaiSettingsMessage::FixedPeerInputChanged,
            settings.fixed_peer_error.as_deref(),
        ),
        max_banscore_section(max_banscore),
        Space::new().height(Fill),
        actions_section(settings),
        danger_section(settings)
    ]
    .spacing(15)
    .width(FillPortion(1));

    row![left, right].spacing(20).into()
}

/// [`Button`] style for the [`Network`] toggle section.
pub(crate) fn network_button_style(
    button_network: Network,
    active_network: Network,
    color: Color,
) -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    move |_theme, button_status| {
        let is_active = button_network == active_network;

        let pair = if is_active {
            Pair { color, text: BLACK }
        } else {
            match button_status {
                ButtonStatus::Active => Pair {
                    color: color.scale_alpha(0.5),
                    text: BLACK,
                },
                ButtonStatus::Hovered => Pair {
                    color: color.scale_alpha(0.8),
                    text: BLACK,
                },
                ButtonStatus::Pressed => Pair { color, text: BLACK },
                ButtonStatus::Disabled => Pair {
                    color: color.scale_alpha(0.5),
                    text: BLACK.scale_alpha(0.5),
                },
            }
        };

        ButtonStyle {
            background: Some(BackgroundColor(pair.color)),
            text_color: pair.text,
            border: Border {
                color: BLACK,
                width: 2.0,
                radius: Radius::new(0.0),
            },
            ..ButtonStyle::default()
        }
    }
}

fn network_button_with_disable_logic<'a>(
    label: &'static str,
    button_network: Network,
    active_network: Network,
    color: Color,
) -> Button<'a, BonsaiSettingsMessage> {
    let is_network_active = button_network == active_network;

    let button = button(text(label).size(16).align_x(Center).align_y(Center))
        .width(Fill)
        .style(network_button_style(button_network, active_network, color));

    if !is_network_active
        && (button_network == Network::Bitcoin
            || button_network == Network::Signet
            || button_network == Network::Regtest)
    {
        button.on_press(BonsaiSettingsMessage::NetworkChanged(button_network))
    } else {
        button
    }
}

pub(crate) fn boolean_button_style(
    button_value: bool,
    active_value: bool,
    color: iced::Color,
) -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    move |_theme, button_status| {
        let is_active = button_value == active_value;

        let pair = if is_active {
            Pair { color, text: BLACK }
        } else {
            match button_status {
                ButtonStatus::Active => Pair {
                    color: color.scale_alpha(0.5),
                    text: BLACK,
                },
                ButtonStatus::Hovered => Pair {
                    color: color.scale_alpha(0.8),
                    text: BLACK,
                },
                ButtonStatus::Pressed => Pair { color, text: BLACK },
                ButtonStatus::Disabled => Pair {
                    color: color.scale_alpha(0.5),
                    text: BLACK.scale_alpha(0.5),
                },
            }
        };

        ButtonStyle {
            background: Some(BackgroundColor(pair.color)),
            text_color: pair.text,
            border: Border {
                color: BLACK,
                width: 2.0,
                radius: Radius::new(0.0),
            },
            ..ButtonStyle::default()
        }
    }
}

fn boolean_button_with_disable_logic<'a>(
    label: &'static str,
    button_value: bool,
    active_value: bool,
    color: iced::Color,
    message: BonsaiSettingsMessage,
) -> iced::widget::Button<'a, BonsaiSettingsMessage> {
    let is_active = button_value == active_value;

    let button = button(text(label).size(16).align_x(Center).align_y(Center))
        .width(Fill)
        .style(boolean_button_style(button_value, active_value, color));

    if !is_active {
        button.on_press(message)
    } else {
        button
    }
}

pub(crate) fn table_cell_with_shadow() -> impl Fn(&Theme) -> ContainerStyle {
    |_theme| ContainerStyle {
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        },
        shadow: SHADOW_GRAY,
        ..Default::default()
    }
}

pub(crate) fn delete_button_container() -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    |_theme, status| {
        let text_color = match status {
            ButtonStatus::Hovered => RED.scale_alpha(0.7),
            ButtonStatus::Pressed => RED.scale_alpha(0.5),
            _ => OFF_WHITE,
        };

        ButtonStyle {
            border: Border {
                color: RED,
                width: BORDER_WIDTH,
                radius: Radius::new(BORDER_RADIUS),
            },
            text_color,
            shadow: SHADOW_RED,
            ..Default::default()
        }
    }
}
