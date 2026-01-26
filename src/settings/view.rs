use bdk_floresta::UtreexoNodeConfig;
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
use crate::settings::bonsai_settings::AUTO_START_NODE;
use crate::settings::bonsai_settings::BonsaiSettings;
use crate::settings::bonsai_settings::BonsaiSettingsMessage;

const SECTION_BOX_HEIGHT: f32 = 30.0;

pub(crate) fn view_settings(settings: &BonsaiSettings) -> Element<'_, BonsaiSettingsMessage> {
    let utreexo_node_config_default = UtreexoNodeConfig::default();

    let auto_start = settings.node.auto_start.unwrap_or(AUTO_START_NODE);
    let active_network = settings
        .bonsai
        .network
        .unwrap_or(utreexo_node_config_default.network);

    let node_config = settings.node.get_network_config(active_network);

    let use_assume_utreexo = node_config.use_assume_utreexo.unwrap_or(true);
    let use_powfps = node_config.pow_fraud_proofs.unwrap_or(true);
    let backfill = node_config.backfill.unwrap_or(true);
    let allow_v1_fallback = node_config.allow_v1_fallback.unwrap_or(true);
    let disable_dns_seeds = node_config.disable_dns_seeds.unwrap_or(false);
    let user_agent = node_config.user_agent.clone();
    let fixed_peer = node_config.fixed_peer.clone();
    let proxy = node_config.proxy;
    let max_banscore = node_config.max_banscore.unwrap_or_default();
    let max_inflight = node_config.max_inflight.unwrap_or_default();
    let max_outbound = node_config.max_outbound.unwrap_or_default();

    let network_title: Container<'_, BonsaiSettingsMessage> = container(text("NETWORK").size(22));
    // TODO(@luisschwab): remove once BIP-0183 is final and `utreexod` supports other networks.
    let network_buttons: Container<'_, BonsaiSettingsMessage> = container(
        row![
            tooltip(
                network_button_with_disable_logic("BITCOIN", Network::Bitcoin, active_network, ORANGE),
                text("Support for `Network::Bitcoin` will become available once\nBIP-0183 is final and implemented on `Floresta` and `utreexod`").size(TABLE_CELL_FONT_SIZE),
                tooltip::Position::FollowCursor
            ).style(container::rounded_box),
            network_button_with_disable_logic("SIGNET", Network::Signet, active_network, PURPLE),
            tooltip(
                network_button_with_disable_logic("TESTNET4", Network::Testnet4, active_network, BLUE),
                text("Support for `Network::Testnet4` will become available once\nBIP-0183 is final and implemented on `Floresta` and `utreexod`").size(TABLE_CELL_FONT_SIZE),
                tooltip::Position::FollowCursor
            ).style(container::rounded_box),
            network_button_with_disable_logic("REGTEST", Network::Regtest, active_network, YELLOW),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let network_section = column![network_title, network_buttons];

    let auto_start_title: Container<'_, BonsaiSettingsMessage> =
        container(text("AUTO START NODE").size(21));
    let auto_start_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                auto_start,
                GREEN_SHAMROCK,
                BonsaiSettingsMessage::AutoStartChanged(true)
            ),
            boolean_button_with_disable_logic(
                "FALSE",
                false,
                auto_start,
                RED,
                BonsaiSettingsMessage::AutoStartChanged(false)
            ),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let auto_start_section = column![auto_start_title, auto_start_buttons];

    let assume_utreexo_title: Container<'_, BonsaiSettingsMessage> =
        container(text("ASSUME UTREEXO").size(21));
    let assume_utreexo_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                use_assume_utreexo,
                GREEN_SHAMROCK,
                BonsaiSettingsMessage::UseAssumeUtreexoChanged(true)
            ),
            boolean_button_with_disable_logic(
                "FALSE",
                false,
                use_assume_utreexo,
                RED,
                BonsaiSettingsMessage::UseAssumeUtreexoChanged(false)
            ),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let assume_utreexo_section = column![assume_utreexo_title, assume_utreexo_buttons];

    let powfps_title: Container<'_, BonsaiSettingsMessage> =
        container(text("PROOF-OF-WORK FRAUD PROOFS").size(21));
    let powfps_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                use_powfps,
                GREEN_SHAMROCK,
                BonsaiSettingsMessage::PowFraudProofsChanged(true)
            ),
            boolean_button_with_disable_logic(
                "FALSE",
                false,
                use_powfps,
                RED,
                BonsaiSettingsMessage::PowFraudProofsChanged(false)
            ),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let powfps_section = column![powfps_title, powfps_buttons];

    let backfill_title: Container<'_, BonsaiSettingsMessage> = container(text("BACKFILL").size(21));
    let backfill_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                backfill,
                GREEN_SHAMROCK,
                BonsaiSettingsMessage::BackfillChanged(true)
            ),
            boolean_button_with_disable_logic(
                "FALSE",
                false,
                backfill,
                RED,
                BonsaiSettingsMessage::BackfillChanged(false)
            ),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let backfill_section = column![backfill_title, backfill_buttons];

    let v1_fallback_title: Container<'_, BonsaiSettingsMessage> =
        container(text("ALLOW V1 FALLBACK").size(21));
    let v1_fallback_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                allow_v1_fallback,
                GREEN_SHAMROCK,
                BonsaiSettingsMessage::AllowV1FallbackChanged(true)
            ),
            boolean_button_with_disable_logic(
                "FALSE",
                false,
                allow_v1_fallback,
                RED,
                BonsaiSettingsMessage::AllowV1FallbackChanged(false)
            ),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let v1_fallback_section = column![v1_fallback_title, v1_fallback_buttons];

    let disable_dns_seeds_title: Container<'_, BonsaiSettingsMessage> =
        container(text("DISABLE DNS SEEDS").size(21));
    let disable_dns_seeds_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                disable_dns_seeds,
                GREEN_SHAMROCK,
                BonsaiSettingsMessage::DisableDnsSeedsChanged(true)
            ),
            boolean_button_with_disable_logic(
                "FALSE",
                false,
                disable_dns_seeds,
                RED,
                BonsaiSettingsMessage::DisableDnsSeedsChanged(false)
            ),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let disable_dns_seeds_section = column![disable_dns_seeds_title, disable_dns_seeds_buttons];

    let user_agent_title: Container<'_, BonsaiSettingsMessage> =
        container(text("USER AGENT").size(21));
    let user_agent_input = container(
        text_input(
            user_agent.as_deref().unwrap_or("NULL"),
            &settings.user_agent_input,
        )
        .on_input(BonsaiSettingsMessage::UserAgentInputChanged)
        .padding(10)
        .width(Fill),
    )
    .style(title_container())
    .padding(1);
    let user_agent_section = column![user_agent_title, user_agent_input];

    let left = column![
        network_section,
        Space::new().height(Length::Fill),
        auto_start_section,
        Space::new().height(Length::Fill),
        assume_utreexo_section,
        Space::new().height(Length::Fill),
        powfps_section,
        Space::new().height(Length::Fill),
        backfill_section,
        Space::new().height(Length::Fill),
        v1_fallback_section,
        Space::new().height(Length::Fill),
        disable_dns_seeds_section,
        Space::new().height(Length::Fill),
        user_agent_section,
    ]
    .width(FillPortion(1));

    let proxy_title: Container<'_, BonsaiSettingsMessage> = container(text("PROXY").size(21));
    let proxy_input = container(
        text_input(
            &proxy
                .map(|p| p.to_string())
                .unwrap_or_else(|| "NULL".to_string()),
            &settings.proxy_input,
        )
        .on_input(BonsaiSettingsMessage::ProxyInputChanged)
        .padding(10)
        .width(Fill),
    )
    .style(title_container())
    .padding(1);
    let proxy_section = column![proxy_title, proxy_input];

    let fixed_peer_title: Container<'_, BonsaiSettingsMessage> =
        container(text("FIXED PEER").size(21));
    let fixed_peer_input = container(
        text_input(
            fixed_peer.as_deref().unwrap_or("NULL"),
            &settings.fixed_peer_input,
        )
        .on_input(BonsaiSettingsMessage::FixedPeerInputChanged)
        .padding(10)
        .width(Fill),
    )
    .style(title_container())
    .padding(1);
    let fixed_peer_section = column![fixed_peer_title, fixed_peer_input];

    let max_banscore_title: Container<'_, BonsaiSettingsMessage> =
        container(text("MAX BAN SCORE").size(21));
    let max_banscore_controls = container(
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
    let max_banscore_section = column![max_banscore_title, max_banscore_controls];

    let max_outbound_title: Container<'_, BonsaiSettingsMessage> =
        container(text("MAX OUTBOUND PEERS").size(21));
    let max_outbound_controls = container(
        row![
            container(
                text(max_outbound.to_string())
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
                .on_press_maybe(if max_outbound > 1 {
                    Some(BonsaiSettingsMessage::MaxOutboundChanged(
                        (max_outbound - 1).to_string(),
                    ))
                } else {
                    None
                })
                .width(FillPortion(1))
                .style(button_container()),
            button(text("+").size(16).align_x(Center).align_y(Center))
                .on_press_maybe(if max_outbound < 100 {
                    Some(BonsaiSettingsMessage::MaxOutboundChanged(
                        (max_outbound + 1).to_string(),
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
    let max_outbound_section = column![max_outbound_title, max_outbound_controls];

    let max_inflight_title: Container<'_, BonsaiSettingsMessage> =
        container(text("MAX INFLIGHT REQUESTS").size(21));
    let max_inflight_controls = container(
        row![
            container(
                text(max_inflight.to_string())
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
                .on_press_maybe(if max_inflight > 1 {
                    Some(BonsaiSettingsMessage::MaxInflightChanged(
                        (max_inflight - 1).to_string(),
                    ))
                } else {
                    None
                })
                .width(FillPortion(1))
                .style(button_container()),
            button(text("+").size(16).align_x(Center).align_y(Center))
                .on_press_maybe(if max_inflight < 100 {
                    Some(BonsaiSettingsMessage::MaxInflightChanged(
                        (max_inflight + 1).to_string(),
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
    let max_inflight_section = column![max_inflight_title, max_inflight_controls];

    let save_button_row = row![
        text(if settings.unsaved_changes {
            "UNSAVED CHANGES"
        } else {
            ""
        })
        .size(12)
        .color(if settings.unsaved_changes {
            ORANGE
        } else {
            GREEN_SHAMROCK
        }),
        Space::new().width(Fill),
        button(
            text("SAVE SETTINGS")
                .size(20)
                .align_x(Center)
                .align_y(Center)
        )
        .on_press_maybe(if settings.unsaved_changes {
            Some(BonsaiSettingsMessage::SaveSettings)
        } else {
            None
        })
        .style(button_container())
        .width(Length::Fixed(220.0))
        .height(Length::Fixed(50.0))
    ]
    .spacing(10)
    .align_y(Center);

    let restart_button_row = row![
        text(if settings.node_restart_required {
            "CHANGED SETTINGS\nREQUIRE A NODE RESTART"
        } else {
            ""
        })
        .size(12)
        .color(if settings.node_restart_required {
            ORANGE
        } else {
            GREEN_SHAMROCK
        }),
        Space::new().width(Fill),
        button(
            text("RESTART NODE")
                .size(20)
                .align_x(Center)
                .align_y(Center)
        )
        .on_press_maybe(if settings.node_restart_required {
            Some(BonsaiSettingsMessage::RestartNode)
        } else {
            None
        })
        .style(button_container())
        .width(Length::Fixed(220.0))
        .height(Length::Fixed(50.0))
    ]
    .spacing(10)
    .align_y(Center);

    let actions_container = container(column![save_button_row, restart_button_row].spacing(20))
        .padding(15)
        .style(title_container())
        .width(Fill);

    // TODO(@luisschwab): implement data deletion
    let delete_data_row = row![
        text("THIS ACTION IS DESTRUCTIVE!\nALL VALIDATION WORK AND\nCOMPACT FILTERS WILL BE LOST")
            .size(12)
            .color(RED),
        Space::new().width(Fill),
        button(
            text("DELETE NODE DATA")
                .color(RED)
                .size(20)
                .align_x(Center)
                .align_y(Center)
        )
        .style(delete_button_container())
        .width(Length::Fixed(220.0))
        .height(Length::Fixed(50.0))
    ];

    let danger_container = container(column![delete_data_row])
        .padding(15)
        .style(title_container())
        .width(Fill);

    let right = column![
        proxy_section,
        fixed_peer_section,
        max_banscore_section,
        max_outbound_section,
        max_inflight_section,
        Space::new().height(Fill),
        actions_container,
        danger_container
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

// TODO(@luisschwab): enable other networks once we have bridges.
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
        && (button_network == Network::Signet || button_network == Network::Regtest)
    // TODO(@luisschwab): remove once BIP-0183 is final and `utreexod` supports other networks.
    // && (button_network == Network::Bitcoin || button_network == Network::Signet)
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
