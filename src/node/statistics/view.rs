use bitcoin::Network;
use iced::Alignment::Center;
use iced::Element;
use iced::Length;
use iced::Length::Fill;
use iced::Padding;
use iced::widget::Container;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text::Wrapping;

use crate::common::interface::color::BLUE;
use crate::common::interface::color::GREEN_SHAMROCK;
use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::color::ORANGE;
use crate::common::interface::color::RED;
use crate::common::interface::color::network_color;
use crate::common::interface::container::button_container;
use crate::common::util::format_duration;
use crate::common::util::format_thousands;
use crate::node::control::NodeStatus;
use crate::node::log_capture::LogCapture;
use crate::node::message::NodeMessage;
use crate::node::statistics::style::ControlButton;
use crate::node::statistics::style::action_button;
use crate::node::statistics::style::log_container;
use crate::node::stats_fetcher::NodeStatistics;
use crate::node::style::TITLE_PADDING;
use crate::node::style::table_cell;
use crate::node::style::title_container;
use crate::pulse_color;

/// Calculate IBD progress from blocks and headers.
fn calculate_progress(blocks: u32, headers: u32) -> f64 {
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
    let button = button(text(label).align_x(Center).align_y(Center))
        .style(action_button(node_status, action_type))
        .width(Fill);

    // Determine whether the button should be enabled.
    #[allow(clippy::match_like_matches_macro)]
    let should_enable = match (node_status, action_type) {
        (NodeStatus::Running, ControlButton::Restart) => true,
        (NodeStatus::Running, ControlButton::Shutdown) => true,
        (NodeStatus::Inactive, ControlButton::Start) => true,
        (NodeStatus::Failed(_), ControlButton::Start) => true,
        _ => false,
    };

    if should_enable {
        button.on_press(message)
    } else {
        button
    }
}

pub(crate) fn view_statistics<'a>(
    network: Network,
    node_status: &'a NodeStatus,
    statistics: &'a Option<NodeStatistics>,
    log_capture: &'a LogCapture,
    app_clock: usize,
) -> Element<'a, NodeMessage> {
    // Control Button Section.
    let control_button_title: Container<'_, NodeMessage> = container(text("NODE CONTROL").size(24));
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
    let ibd_status = statistics.as_ref().map(|s| s.in_ibd).unwrap_or(true);
    let headers = statistics.as_ref().map(|s| s.headers).unwrap_or(0);
    let blocks = statistics.as_ref().map(|s| s.blocks).unwrap_or(0);
    let ibd_progress = calculate_progress(blocks, headers);
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

    let network_color = network_color(&network);
    let node_status_color = match node_status {
        NodeStatus::Starting => pulse_color(GREEN_SHAMROCK, app_clock),
        NodeStatus::Running => GREEN_SHAMROCK,
        NodeStatus::Inactive => OFF_WHITE,
        NodeStatus::Failed(_) => RED,
        NodeStatus::ShuttingDown => pulse_color(RED, app_clock),
    };

    let metrics_title = container(text("NODE STATISTICS").size(24));
    let metrics_table = container(
        column![
            row![
                container(text("STATUS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(
                    text(node_status.to_string())
                        .size(14)
                        .color(node_status_color)
                )
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
                    text(network.to_string().to_uppercase())
                        .size(14)
                        .color(network_color)
                )
                .padding(10)
                .width(Length::FillPortion(1))
                .style(table_cell()),
            ],
            row![
                container(text("TOR CIRCUIT").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text("TODO").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("IBD STATUS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(ibd_status.to_string().to_uppercase()).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("IBD PROGRESS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(format!("{:.2}%", ibd_progress)).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("BACKFILL").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text("TODO").size(14)) // TODO: add backfill status getter to NodeInterface
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
                container(text("COMPACT\nBLOCK FILTERS").size(14)) // TODO: add CBF info getter to NodeInterface
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(60.0))
                    .align_y(Center)
                    .style(table_cell()),
                container(text("TODO").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(60.0))
                    .align_y(Center)
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
                    .align_y(Center)
                    .style(table_cell()),
                container(text(user_agent).size(12))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .height(Length::Fixed(60.0))
                    .align_y(Center)
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

    let left = column![control, metrics]
        .spacing(20)
        .width(Length::FillPortion(4));

    // Logs Section.
    let log_title = container(
        row![
            text("LOGS").size(24),
            Space::new().width(Length::Fill),
            button(text("CLEAR").size(14))
                .on_press(NodeMessage::ClearLogs)
                .style(button_container())
                .padding(2)
        ]
        .spacing(10)
        .align_y(Center),
    )
    .width(Length::Fill);

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
        // Keep the last 5000 logs rendered.
        let skip_count = logs.len().saturating_sub(5000);

        for log in logs.into_iter().skip(skip_count) {
            let color = if log.contains("ERROR") {
                RED
            } else if log.contains("WARN") {
                ORANGE
            } else if log.contains("INFO") {
                GREEN_SHAMROCK
            } else if log.contains("DEBUG") {
                BLUE
            } else {
                OFF_WHITE
            };

            log_column = log_column.push(text(log).size(12).color(color).wrapping(Wrapping::Glyph));
        }
    }

    let logs_scrollable = scrollable(log_column).height(Length::Fill).anchor_bottom();
    let logs_container = container(logs_scrollable)
        .style(log_container())
        .padding(TITLE_PADDING)
        .height(Length::Fill)
        .width(Length::FillPortion(6));

    let right = column![log_title, logs_container]
        .width(Length::FillPortion(6))
        .spacing(5);

    row![left, right].spacing(20).into()
}
