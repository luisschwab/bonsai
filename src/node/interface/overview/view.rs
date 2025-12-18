use std::time::Duration;

use iced::Length::Fill;
use iced::widget::{Container, button, column, container, row, scrollable, text};
use iced::{Element, Length, Padding};

use crate::common::interface::color::{BLUE, GREEN, OFF_WHITE, ORANGE, RED};
use crate::node::control::{NETWORK, NodeStatus};
use crate::node::interface::common::{TITLE_PADDING, table_cell, title_container};
use crate::node::interface::overview::style::{ActionButton, action_button, log_container};
use crate::node::logger::LogCapture;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;

/// Format a [`Duration`] to HH:MM:SS.
fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{:02}h {:02}m {:02}s", hours, minutes, seconds)
}

/// Calculate IBD progress from
fn calculate_progress(headers: u32, blocks: u32) -> f64 {
    if headers > 0 {
        (blocks as f64 / headers as f64) * 100.0
    } else {
        0.0
    }
}

/// View renderer for the `NODE OVERVIEW` tab.
pub(crate) fn view_overview<'a>(
    node_status: &'a NodeStatus,
    statistics: &'a Option<NodeStatistics>,
    log_capture: &'a LogCapture,
) -> Element<'a, NodeMessage> {
    // Tab Title.
    let title: Container<'_, NodeMessage> = container(text("NODE OVERVIEW").size(25))
        .style(title_container())
        .padding(TITLE_PADDING);

    // Action Button Section.
    let action_button_title: Container<'_, NodeMessage> = container(text("ACTIONS").size(24));
    let action_button_container: Container<'_, NodeMessage> = container(
        row![
            button(text("START"))
                .on_press(NodeMessage::Start)
                .style(action_button(node_status, ActionButton::Start))
                .width(Fill),
            button(text("RESTART"))
                .on_press(NodeMessage::Restart)
                .style(action_button(node_status, ActionButton::Restart))
                .width(Fill),
            button(text("SHUTDOWN"))
                .on_press(NodeMessage::Shutdown)
                .style(action_button(node_status, ActionButton::Shutdown))
                .width(Fill),
        ]
        .spacing(10),
    )
    .style(title_container())
    .padding(10);
    let action_button = column![action_button_title, action_button_container];

    // Metrics Section.
    let in_ibd = statistics.as_ref().map(|s| s.in_ibd).unwrap_or(true);
    let headers = statistics.as_ref().map(|s| s.headers).unwrap_or(0);
    let blocks = statistics.as_ref().map(|s| s.blocks).unwrap_or(0);
    let progress = calculate_progress(headers, blocks);
    let peer_count = statistics
        .as_ref()
        .map(|s| s.peer_informations.len())
        .unwrap_or(0);
    let uptime = statistics
        .as_ref()
        .map(|stats| format_duration(stats.uptime))
        .unwrap_or("00h 00m 00s".to_string());

    let metrics_title = container(text("METRICS").size(24));
    let metrics_table = container(
        column![
            row![
                container(text("STATUS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(node_status.to_string()).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("NETWORK").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(NETWORK.to_string().to_uppercase()).size(14))
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
                container(text(headers).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("BLOCKS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(blocks).size(14))
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
                    .style(table_cell()),
                container(text("TODO").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
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
        log_column = log_column.push(text("No logs yet...").size(12));
    } else {
        for log in logs.into_iter().take(100) {
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

    let left = column![title, action_button, metrics]
        .spacing(20)
        .width(Length::FillPortion(4));
    let right = column![log_title, logs_container];

    row![left, right].spacing(20).into()
}
