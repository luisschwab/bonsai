use bdk_floresta::rustreexo::accumulator::stump::Stump;
use iced::widget::{Container, column, container, qr_code, row, text};
use iced::{Alignment, Element, Length, Padding};

use crate::common::interface::container::common::CELL_HEIGHT;
use crate::common::util::format_thousands;
use crate::node::interface::common::{TITLE_PADDING, table_cell, title_container};
use crate::node::interface::p2p::style::peer_info_table_container;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;

fn encode_stump(stump: &Stump) -> String {
    // TODO: Implement proper stump encoding
    format!("{:?}", stump)
}

pub fn view_utreexo<'a>(
    statistics: &'a Option<NodeStatistics>,
    qr_data: &'a Option<qr_code::Data>,
) -> Element<'a, NodeMessage> {
    // Tab Title
    let title: Container<'_, NodeMessage> = container(text("NODE UTREEXO").size(25))
        .style(title_container())
        .padding(TITLE_PADDING);

    let num_leaves = statistics
        .as_ref()
        .map(|s| s.accumulator.leaves)
        .unwrap_or(0);

    let roots = statistics.as_ref().map(|s| s.accumulator.roots.clone());
    let num_roots = roots.clone().unwrap_or_default().len();
    // 32B per root
    let size_roots = format!("{} BYTES", 32 * num_roots);

    // Left: Statistics Table
    let accumulator_title = container(text("ACCUMULATOR").size(24));
    let accumulator_table = container(
        column![
            row![
                container(text("NUM ROOTS").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(num_roots).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("NUM LEAVES").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(format_thousands(num_leaves)).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("ACCUMULATOR SIZE").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(size_roots).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
        ]
        .spacing(0),
    )
    .style(title_container());
    let accumulator = container(column![accumulator_title, accumulator_table]);

    let qr_title = container(text("QR CODE").size(24));
    let qr_code = if let Some(data) = qr_data {
        container(qr_code(data).cell_size(4).total_size(350))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(title_container())
    } else {
        container(text("No QR data available").size(14))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(title_container())
    };
    let qr = container(column![qr_title, qr_code]);

    let left = column![title, accumulator, qr]
        .spacing(20)
        .width(Length::Fixed(440.0));

    let roots_title: Container<'_, NodeMessage> = container(text("ROOTS").size(24));
    let mut roots_table = column![].spacing(0);
    if let Some(roots) = roots.as_deref() {
        for root in roots {
            roots_table = roots_table.push(row![
                container(text(hex::encode(**root)).size(12))
                    .padding(11)
                    .height(CELL_HEIGHT)
                    .style(table_cell())
            ]);
        }
    }
    let roots = container(column![
        roots_title,
        container(roots_table).style(peer_info_table_container())
    ])
    .width(Length::Fill);

    let right = column![roots]
        .padding(Padding::from([30, 0]))
        .spacing(20)
        .width(Length::Fill);

    row![left, right].spacing(20).into()
}
