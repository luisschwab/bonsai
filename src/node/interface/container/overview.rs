use iced::border::Radius;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Border, Theme};
use iced::{Element, Length, Padding};

use crate::common::interface::color::{BLUE, GREEN, OFF_WHITE, ORANGE, RED};
use crate::common::interface::container::common::{BORDER_RADIUS, BORDER_WIDTH};
use crate::node::control::{NETWORK, NodeStatus};
use crate::node::interface::container::common::{TITLE_PADDING, table_cell, title_container};
use crate::node::logger::LogCapture;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;

pub(crate) fn log_container() -> impl Fn(&Theme) -> ContainerStyle {
    |_theme| ContainerStyle {
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        },
        ..Default::default()
    }
}

pub fn view_overview(
    status: &NodeStatus,
    statistics: &Option<NodeStatistics>,
    log_capture: &LogCapture,
) -> Element<'static, NodeMessage> {
    let status_text = status.to_string();

    let control_button = match status {
        NodeStatus::Running => {
            row![
                button(text("SHUTDOWN"))
                    .on_press(NodeMessage::Shutdown)
                    .style(button::danger)
            ]
        }
        NodeStatus::Inactive | NodeStatus::Failed(_) => {
            row![
                button(text("START"))
                    .on_press(NodeMessage::Start)
                    .style(button::success)
            ]
        }
        NodeStatus::ShuttingDown => {
            row![button(text("SHUTTING DOWN")).style(button::secondary)]
        }
        NodeStatus::Starting => {
            row![button(text("STARTING")).style(button::secondary)]
        }
    };

    let in_ibd = statistics.as_ref().map(|s| s.in_ibd).unwrap_or(true);
    let chain_height = statistics.as_ref().map(|s| s.chain_height).unwrap_or(0);
    let validated_height = statistics.as_ref().map(|s| s.validated_height).unwrap_or(0);
    let progress = if chain_height > 0 {
        (validated_height as f64 / chain_height as f64) * 100.0
    } else {
        0.0
    };
    let peer_count = statistics.as_ref().map(|s| s.peer_info.len()).unwrap_or(0);
    let uptime = if let Some(stats) = statistics {
        let total_secs = stats.uptime.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        format!("{:02}h {:02}m {:02}s", hours, minutes, seconds)
    } else {
        "00h 00m 00s".to_string()
    };

    let title = container(text("NODE OVERVIEW").size(25))
        .style(title_container())
        .padding(TITLE_PADDING);

    let left = column![
        title,
        text("").size(10),
        control_button,
        text("").size(10),
        row![
            container(text("STATUS").size(14))
                .padding(10)
                .width(Length::FillPortion(1))
                .style(table_cell()),
            container(text(status_text).size(14))
                .padding(10)
                .width(Length::FillPortion(1))
                .style(table_cell()),
        ],
        row![
            container(text("NETWORK").size(14))
                .padding(10)
                .width(Length::FillPortion(1))
                .style(table_cell()),
            container(text(format!("{}", NETWORK.to_string().to_uppercase())).size(14))
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
            container(text(chain_height).size(14))
                .padding(10)
                .width(Length::FillPortion(1))
                .style(table_cell()),
        ],
        row![
            container(text("BLOCKS").size(14))
                .padding(10)
                .width(Length::FillPortion(1))
                .style(table_cell()),
            container(text(validated_height).size(14))
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
            container(text("UPTIME").size(14))
                .padding(10)
                .width(Length::FillPortion(1))
                .style(table_cell()),
            container(text(uptime).size(14))
                .padding(10)
                .width(Length::FillPortion(1))
                .style(table_cell()),
        ],
    ]
    .spacing(0)
    .width(Length::FillPortion(4));

    let log_title = container(text("LOGS").size(24));

    let logs = log_capture.get_logs();
    let mut log_column = column![].spacing(2).padding(Padding {
        top: 0.0,
        right: 12.0,
        bottom: 0.0,
        left: 0.0,
    });

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

    let right = column![log_title, logs_container];

    row![left, right].spacing(20).into()
}
