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
use iced::Padding;
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
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::toggler;

use crate::common::interface::color::BLACK;
use crate::common::interface::color::BLUE;
use crate::common::interface::color::GREEN;
use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::color::ORANGE;
use crate::common::interface::color::PURPLE;
use crate::common::interface::color::RED;
use crate::common::interface::color::YELLOW;
use crate::common::interface::container::common::BORDER_RADIUS;
use crate::common::interface::container::common::BORDER_WIDTH;
use crate::common::interface::container::common::SHADOW;
use crate::common::interface::container::content::button_container;
use crate::node::interface::common::table_cell;
use crate::node::interface::common::title_container;
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
    let user_agent = node_config.user_agent.clone().unwrap_or_default();
    let fixed_peer = node_config.fixed_peer.clone().unwrap_or_default();
    let proxy = node_config.proxy;
    let max_banscore = node_config.max_banscore.unwrap_or_default();
    let max_inflight = node_config.max_inflight.unwrap_or_default();
    let max_outbound = node_config.max_outbound.unwrap_or_default();

    let network_title: Container<'_, BonsaiSettingsMessage> = container(text("NETWORK").size(24));
    let network_buttons: Container<'_, BonsaiSettingsMessage> = container(
        row![
            network_button_with_disable_logic("BITCOIN", Network::Bitcoin, active_network, ORANGE),
            network_button_with_disable_logic("SIGNET", Network::Signet, active_network, PURPLE),
            network_button_with_disable_logic("TESTNET4", Network::Testnet4, active_network, BLUE),
            network_button_with_disable_logic("REGTEST", Network::Regtest, active_network, YELLOW),
        ]
        .height(Length::Fixed(SECTION_BOX_HEIGHT))
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let network_section = column![network_title, network_buttons];

    let auto_start_title: Container<'_, BonsaiSettingsMessage> =
        container(text("AUTO START NODE").size(24));
    let auto_start_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                auto_start,
                GREEN,
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

    let powfps_title: Container<'_, BonsaiSettingsMessage> =
        container(text("PROOF-OF-WORK FRAUD PROOFS").size(24));
    let powfps_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                use_powfps,
                GREEN,
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

    let backfill_title: Container<'_, BonsaiSettingsMessage> = container(text("BACKFILL").size(24));
    let backfill_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                backfill,
                GREEN,
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
        container(text("ALLOW V1 FALLBACK").size(24));
    let v1_fallback_buttons = container(
        row![
            boolean_button_with_disable_logic(
                "TRUE",
                true,
                allow_v1_fallback,
                GREEN,
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

    let left = column![
        network_section,
        auto_start_section,
        powfps_section,
        backfill_section,
        v1_fallback_section,
    ]
    .spacing(15)
    .width(FillPortion(1));

    let user_agent_title: Container<'_, BonsaiSettingsMessage> =
        container(text("USER AGENT").size(24));
    let user_agent_input = container(
        text_input("USER AGENT", &settings.user_agent_input)
            .on_input(BonsaiSettingsMessage::UserAgentInputChanged)
            .padding(10)
            .width(Fill),
    )
    .style(title_container())
    .padding(1);
    let user_agent_section = column![user_agent_title, user_agent_input];

    let fixed_peer_title: Container<'_, BonsaiSettingsMessage> =
        container(text("FIXED PEER").size(24));
    let fixed_peer_input = container(
        text_input("123.123.123.123:38333", &settings.fixed_peer_input)
            .on_input(BonsaiSettingsMessage::FixedPeerInputChanged)
            .padding(10)
            .width(Fill),
    )
    .style(title_container())
    .padding(1);
    let fixed_peer_section = column![fixed_peer_title, fixed_peer_input];

    let proxy_title: Container<'_, BonsaiSettingsMessage> = container(text("PROXY").size(24));
    let proxy_input = container(
        text_input("123.123.123.123:9050", &settings.proxy_input)
            .on_input(BonsaiSettingsMessage::ProxyInputChanged)
            .padding(10)
            .width(Fill),
    )
    .style(title_container())
    .padding(1);
    let proxy_section = column![proxy_title, proxy_input];

    let max_banscore_title: Container<'_, BonsaiSettingsMessage> =
        container(text("MAX BAN SCORE").size(24));
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
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
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
        container(text("MAX OUTBOUND PEERS").size(24));
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
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
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
        container(text("MAX INFLIGHT REQUESTS").size(24));
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
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
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
            GREEN
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
    .align_y(iced::alignment::Vertical::Center);

    let restart_button_row = row![
        text(if settings.node_restart_required {
            "NODE RESTART REQUIRED\nTO APPLY SETTINGS"
        } else {
            ""
        })
        .size(12)
        .color(if settings.node_restart_required {
            ORANGE
        } else {
            GREEN
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
    .align_y(iced::alignment::Vertical::Center);

    let actions_container = container(column![save_button_row, restart_button_row,].spacing(20))
        .padding(10)
        .width(Fill);

    let right = column![
        user_agent_section,
        fixed_peer_section,
        proxy_section,
        max_banscore_section,
        max_outbound_section,
        max_inflight_section,
        Space::new().height(Fill),
        actions_container
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
            Pair {
                color: color.scale_alpha(0.8),
                text: BLACK,
            }
        } else {
            match button_status {
                ButtonStatus::Active => Pair {
                    color: color.scale_alpha(0.8),
                    text: BLACK,
                },
                ButtonStatus::Hovered => Pair {
                    color: color.scale_alpha(0.6),
                    text: BLACK,
                },
                ButtonStatus::Pressed => Pair {
                    color: color.scale_alpha(0.5),
                    text: BLACK,
                },
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
    let is_network_active = (button_network == active_network);

    let button = button(text(label).size(16).align_x(Center).align_y(Center))
        .width(Fill)
        .style(network_button_style(button_network, active_network, color));

    if !is_network_active && button_network == Network::Signet {
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
            Pair {
                color: color.scale_alpha(0.8),
                text: BLACK,
            }
        } else {
            match button_status {
                ButtonStatus::Active => Pair { color, text: BLACK },
                ButtonStatus::Hovered => Pair {
                    color: color.scale_alpha(0.8),
                    text: BLACK,
                },
                ButtonStatus::Pressed => Pair {
                    color: color.scale_alpha(0.6),
                    text: BLACK,
                },
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
        shadow: SHADOW,
        ..Default::default()
    }
}

// Helper function for integer controls with +/- buttons
fn integer_control<'a, F>(
    value: u32,
    on_change: F,
    step: u32,
    min: u32,
    max: u32,
) -> iced::widget::Row<'a, BonsaiSettingsMessage>
where
    F: Fn(u32) -> BonsaiSettingsMessage + 'a + Copy,
{
    row![
        button(text("-").size(14))
            .on_press_maybe(if value > min {
                Some(on_change(value.saturating_sub(step)))
            } else {
                None
            })
            .padding(5)
            .style(button_container())
            .width(Length::Fixed(30.0)),
        container(text(value.to_string()).size(14))
            .padding(5)
            .width(Fill)
            .center_x(Fill)
            .align_x(iced::alignment::Horizontal::Center),
        button(text("+").size(14))
            .on_press_maybe(if value < max {
                Some(on_change(value.saturating_add(step)))
            } else {
                None
            })
            .padding(5)
            .style(button_container())
            .width(Length::Fixed(30.0)),
    ]
    .spacing(5)
    .align_y(iced::alignment::Vertical::Center)
}
