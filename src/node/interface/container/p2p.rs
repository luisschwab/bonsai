use iced::border::Radius;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Border, Theme};
use iced::{Element, Length, Padding};

use crate::common::interface::color::{BLUE, GREEN, OFF_WHITE, ORANGE, RED};
use crate::common::interface::container::common::{BORDER_RADIUS, BORDER_WIDTH};
use crate::node::control::{NETWORK, NodeStatus};
use crate::node::interface::container::common::{TITLE_PADDING, table_cell, title_container};
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;

pub fn view_p2p(
    status: &NodeStatus,
    statistics: &Option<NodeStatistics>,
) -> Element<'static, NodeMessage> {
    unimplemented!()
}
