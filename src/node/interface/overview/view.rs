use std::time::Duration;

use bdk_wallet::bitcoin::Network;
use iced::Length::Fill;
use iced::widget::{Container, button, column, container, row, scrollable, text};
use iced::{Element, Length, Padding};

use crate::common::interface::color::{BLUE, GREEN, OFF_WHITE, ORANGE, PURPLE, RED};
use crate::common::util::{format_duration, format_thousands};
use crate::node::control::{NETWORK, NodeStatus};
use crate::node::interface::common::{TITLE_PADDING, table_cell, title_container};
use crate::node::interface::overview::style::{ControlButton, action_button, log_container};
use crate::node::log_capture::LogCapture;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;

/// Calculate IBD progress from
fn calculate_progress(headers: u32, blocks: u32) -> f64 {
    if headers > 0 {
        (blocks as f64 / headers as f64) * 100.0
    } else {
        0.0
    }
}

/// Disable control buttons conditionally depending on [`ControlButton`] and [`NodeStatus`].
fn control_button_with_disable_logic<'a>(
    label: &'static str,
    node_status: &'a NodeStatus,
    action_type: ControlButton,
    message: NodeMessage,
) -> iced::widget::Button<'a, NodeMessage> {
    let button = button(text(label))
        .style(action_button(node_status, action_type))
        .width(Fill);

    // Determine whether the button should be enabled.
    #[allow(clippy::match_like_matches_macro)]
    let should_enable = match (node_status, action_type) {
        (NodeStatus::Inactive | NodeStatus::Failed(_), ControlButton::Start) => true,
        (NodeStatus::Running, ControlButton::Restart) => true,
        (NodeStatus::Running, ControlButton::Shutdown) => true,
        _ => false,
    };

    if should_enable {
        button.on_press(message)
    } else {
        button
    }
}

/// View renderer for the `NODE OVERVIEW` tab.
pub(crate) fn view_overview<'a>(
    node_status: &'a NodeStatus,
    statistics: &'a Option<NodeStatistics>,
    log_capture: &'a LogCapture,
    animation_tick: usize,
) -> Element<'a, NodeMessage> {
    // Tab Title.
    let title: Container<'_, NodeMessage> = container(text("NODE OVERVIEW").size(25))
        .style(title_container())
        .padding(TITLE_PADDING);

    // Control Button Section.
    let control_button_title: Container<'_, NodeMessage> = container(text("CONTROL").size(24));
    let control_button_container: Container<'_, NodeMessage> = container(
        row![
            control_button_with_disable_logic(
                "START",
                node_status,
                ControlButton::Start,
                NodeMessage::Start
            ),
            control_button_with_disable_logic(
                "RESTART",
                node_status,
                ControlButton::Restart,
                NodeMessage::Restart
            ),
            control_button_with_disable_logic(
                "SHUTDOWN",
                node_status,
                ControlButton::Shutdown,
                NodeMessage::Shutdown
            ),
        ]
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let control = column![control_button_title, control_button_container];

    // Metrics Section.
    let in_ibd = statistics.as_ref().map(|s| s.in_ibd).unwrap_or(true);
    let headers = statistics.as_ref().map(|s| s.headers).unwrap_or(0);
    let blocks = statistics.as_ref().map(|s| s.blocks).unwrap_or(0);
    let progress = calculate_progress(headers, blocks);
    let user_agent = statistics
        .as_ref()
        .map(|s| s.user_agent.clone())
        .unwrap_or("NULL".to_string());
    let peer_count = statistics
        .as_ref()
        .map(|s| s.peer_informations.len())
        .unwrap_or(0);
    let uptime = statistics
        .as_ref()
        .map(|stats| format_duration(stats.uptime))
        .unwrap_or("00h 00m 00s".to_string());

    let status_color = match node_status {
        NodeStatus::Starting | NodeStatus::Running => GREEN,
        NodeStatus::Inactive => OFF_WHITE,
        NodeStatus::Failed(_) => RED,
        NodeStatus::ShuttingDown => {
            // Pulse with sine wave
            let time = (animation_tick as f32) * 32.0;

            // Pulse with 1 second period (1000ms)
            let pulse = ((time / 1000.0) * std::f32::consts::PI * 2.0).sin();

            // Map sine wave (-1 to 1) to alpha range (0.7 to 1.0)
            let alpha = 0.7 + ((pulse + 1.0) / 2.0) * 0.7;

            RED.scale_alpha(alpha)
        }
    };

    let network_color = match NETWORK {
        Network::Bitcoin => ORANGE,
        Network::Signet => PURPLE,
        Network::Testnet | Network::Testnet4 => BLUE,
        Network::Regtest => OFF_WHITE,
    };

    let metrics_title = container(text("METRICS").size(24));
    let metrics_table = container(
        column![
            row![
                container(text("STATUS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(node_status.to_string()).size(14).color(status_color))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("NETWORK").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(
                    text(NETWORK.to_string().to_uppercase())
                        .size(14)
                        .color(network_color)
                )
                .padding(10)
                .width(Length::FillPortion(1))
                .style(table_cell()),
            ],
            row![
                container(text("IBD").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(in_ibd.to_string().to_uppercase()).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("PROGRESS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(format!("{:.2}%", progress)).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("HEADERS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(format_thousands(headers)).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("BLOCKS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(format_thousands(blocks)).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("PEERS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(peer_count).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("USER AGENT").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(60.0))
                    .align_y(iced::alignment::Vertical::Center)
                    .style(table_cell()),
                container(text(user_agent).size(12))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(60.0))
                    .align_y(iced::alignment::Vertical::Center)
                    .style(table_cell()),
            ],
            row![
                container(text("UPTIME").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(uptime).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("MEMORY [USED]").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text("TODO").size(14).wrapping(text::Wrapping::None))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("MEMORY [ALLOCATED]").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text("TODO").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
        ]
        .spacing(0),
    )
    .style(title_container());
    let metrics = column![metrics_title, metrics_table].spacing(0);

    let left = column![title, control, metrics]
        .spacing(20)
        .width(Length::FillPortion(4));

    // Logs Section.
    let log_title = container(text("LOGS").size(24));

    let mut log_column = column![].spacing(2).padding(Padding {
        top: 0.0,
        right: 10.0,
        bottom: 0.0,
        left: 0.0,
    });

    let logs = log_capture.get_logs();

    if logs.is_empty() {
        log_column = log_column.push(text("").size(12));
    } else {
        // Keep the last 500 logs rendered.
        let skip_count = logs.len().saturating_sub(500);

        for log in logs.into_iter().skip(skip_count) {
            let color = if log.contains("ERROR") {
                RED
            } else if log.contains("WARN") {
                ORANGE
            } else if log.contains("INFO") {
                GREEN
            } else if log.contains("DEBUG") {
                BLUE
            } else {
                OFF_WHITE
            };

            log_column = log_column.push(
                text(log)
                    .size(12)
                    .color(color)
                    .wrapping(text::Wrapping::Glyph),
            );
        }
    }

    let logs_scrollable = scrollable(log_column).height(Length::Fill).anchor_bottom();
    let logs_container = container(logs_scrollable)
        .style(log_container())
        .padding(TITLE_PADDING)
        .height(Length::Fill)
        .width(Length::FillPortion(6));

    let right = column![log_title, logs_container];

    row![left, right].spacing(20).into()
}
