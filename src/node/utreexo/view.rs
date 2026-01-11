use bdk_floresta::rustreexo::accumulator::node_hash::BitcoinNodeHash;
use iced::Alignment::Center;
use iced::Element;
use iced::Length;
use iced::widget::Container;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::qr_code;
use iced::widget::row;
use iced::widget::text;
use iced::widget::tooltip;

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::container::button_container;
use crate::common::util::format_thousands;
use crate::node::message::NodeMessage;
use crate::node::network::style::peer_info_table_container;
use crate::node::stats_fetcher::NodeStatistics;
use crate::node::style::table_cell;
use crate::node::style::title_container;
use crate::node::utreexo::style::ROOT_CELL_HEIGHT;

pub fn view_utreexo<'a>(
    statistics: &'a Option<NodeStatistics>,
    qr_data: &'a Option<qr_code::Data>,
) -> Element<'a, NodeMessage> {
    let num_leaves = statistics
        .as_ref()
        .map(|s| s.accumulator.leaves as u32)
        .unwrap_or(0);
    let roots = statistics.as_ref().map(|s| s.accumulator.roots.clone());
    let num_roots = roots.as_ref().map(|r| r.len()).unwrap_or(0);
    let size_roots = format!("{} BYTES", format_thousands(32 * num_roots));

    // Place the roots in their correct positions
    // according to the binary representation of `num_leaves`.
    let mut roots_in_position: Vec<Option<BitcoinNodeHash>> = vec![None; 32];
    if let Some(roots_vec) = roots {
        let mut root_iter = roots_vec.into_iter();

        // If `num_leaves >> 1` is `1`, place the next root there.
        for (idx, root_slot) in roots_in_position.iter_mut().enumerate() {
            if (num_leaves >> idx) & 1 == 1 {
                *root_slot = root_iter.next();
            }
        }
    }

    // Left: Statistics Table
    let accumulator_title = container(text("ACCUMULATOR STATS").size(24));
    let accumulator_table = container(
        column![
            row![
                container(text("ROOT COUNT").size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
                container(text(num_roots).size(14))
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(table_cell()),
            ],
            row![
                container(text("LEAF COUNT").size(14))
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
            row![
                container(text("PROOF CACHE SIZE").size(14))
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
    let accumulator = container(column![accumulator_title, accumulator_table]);

    let qr_title = container(
        row![
            tooltip(
                text("ACCUMULATOR EXPORT").size(24),
                text("You can use this QR code to export the validation done\non this device to trustlessly skip IBD on another device").size(12),
                tooltip::Position::FollowCursor
            )
            .style(container::rounded_box),
            Space::new().width(Length::Fill),
            button(text("COPY").size(16))
                .on_press(NodeMessage::CopyAccumulatorData)
                .style(button_container())
                .padding(2)
        ]
        .spacing(10)
        .align_y(Center)
    );

    let qr_code = if let Some(data) = qr_data {
        container(qr_code(data).cell_size(4).total_size(350))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(title_container())
    } else {
        container(text("accumulator data unavailable").size(16))
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(title_container())
    };
    let qr = container(column![qr_title, qr_code].spacing(5));

    let left = column![accumulator, qr]
        .spacing(20)
        .width(Length::Fixed(420.0));

    let roots_title: Container<'_, NodeMessage> = container(text("ROOTS").size(24));
    let mut roots_table = column![row![
        container(text("IDX").size(14))
            .align_y(Center)
            .align_x(Center)
            .padding(10)
            .width(Length::FillPortion(1))
            .style(table_cell()),
        container(text("ROOT").size(14))
            .padding(10)
            .width(Length::FillPortion(4))
            .style(table_cell()),
        container(text("IDX").size(14))
            .align_y(Center)
            .align_x(Center)
            .padding(10)
            .width(Length::FillPortion(1))
            .style(table_cell()),
        container(text("ROOT").size(14))
            .padding(10)
            .width(Length::FillPortion(4))
            .style(table_cell()),
    ]]
    .spacing(0);

    for idx in 0..16 {
        let left_idx = idx;
        let right_idx = idx + 16;

        let left_idx_cell = container(text(format!("{:02}", left_idx)).size(14))
            .padding(10)
            .align_y(Center)
            .align_x(Center)
            .height(ROOT_CELL_HEIGHT)
            .width(Length::FillPortion(1))
            .style(table_cell());

        let left_root_cell = if let Some(Some(root)) = roots_in_position.get(left_idx) {
            let root_hex = hex::encode(**root);
            let root_hex_split = format!("{}\n{}", &root_hex[..32], &root_hex[32..]);

            container(text(root_hex_split).size(10))
                .align_y(Center)
                .align_x(Center)
                .height(ROOT_CELL_HEIGHT)
                .width(Length::FillPortion(4))
                .style(table_cell())
        } else {
            container(text("NULL").size(12).color(OFF_WHITE.scale_alpha(0.5)))
                .align_y(Center)
                .align_x(Center)
                .height(ROOT_CELL_HEIGHT)
                .width(Length::FillPortion(4))
                .style(table_cell())
        };

        let right_idx_cell = container(text(format!("{:02}", right_idx)).size(14))
            .padding(10)
            .align_y(Center)
            .align_x(Center)
            .height(ROOT_CELL_HEIGHT)
            .width(Length::FillPortion(1))
            .style(table_cell());

        let right_root_cell = if let Some(Some(root)) = roots_in_position.get(right_idx) {
            let root_hex = hex::encode(**root);
            let root_hex_split = format!("{}\n{}", &root_hex[..32], &root_hex[32..]);

            container(text(root_hex_split).size(10))
                .align_y(Center)
                .align_x(Center)
                .height(ROOT_CELL_HEIGHT)
                .width(Length::FillPortion(4))
                .style(table_cell())
        } else {
            container(text("NULL").size(12).color(OFF_WHITE.scale_alpha(0.5)))
                .align_y(Center)
                .align_x(Center)
                .height(ROOT_CELL_HEIGHT)
                .width(Length::FillPortion(4))
                .style(table_cell())
        };

        roots_table = roots_table.push(row![
            left_idx_cell,
            left_root_cell,
            right_idx_cell,
            right_root_cell,
        ]);
    }

    let roots = container(column![
        roots_title,
        container(roots_table).style(peer_info_table_container())
    ])
    .width(Length::Fill);

    let right = column![roots].spacing(20).width(Length::Fill);

    row![left, right].spacing(20).into()
}
