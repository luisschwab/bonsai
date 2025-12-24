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
use crate::common::interface::container::content::button_container;
use crate::common::util::format_thousands;
use crate::node::interface::common::TITLE_PADDING;
use crate::node::interface::common::table_cell;
use crate::node::interface::common::title_container;
use crate::node::interface::p2p::style::peer_info_table_container;
use crate::node::interface::utreexo::style::ROOT_CELL_HEIGHT;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;

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
    let size_roots = format!("{} BYTES", 32 * num_roots);

    // Left: Statistics Table
    let accumulator_title = container(text("ACCUMULATOR STATS").size(24));
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
        .align_y(iced::alignment::Vertical::Center)
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

    let left = column![title, accumulator, qr]
        .spacing(20)
        .width(Length::Fixed(420.0));

    let roots_title: Container<'_, NodeMessage> = container(text("ROOTS").size(24));
    let mut roots_table = column![row![
        container(text("IDX").size(14))
            .align_y(iced::alignment::Vertical::Center)
            .align_x(iced::alignment::Horizontal::Center)
            .padding(10)
            .width(Length::FillPortion(1))
            .style(table_cell()),
        container(text("ROOT").size(14))
            .padding(10)
            .width(Length::FillPortion(4))
            .style(table_cell()),
        container(text("IDX").size(14))
            .align_y(iced::alignment::Vertical::Center)
            .align_x(iced::alignment::Horizontal::Center)
            .padding(10)
            .width(Length::FillPortion(1))
            .style(table_cell()),
        container(text("ROOT").size(14))
            .padding(10)
            .width(Length::FillPortion(4))
            .style(table_cell()),
    ]]
    .spacing(0);

    let roots = roots.as_deref().unwrap_or(&[]);
    for i in 0..16 {
        let left_idx = i;
        let right_idx = i + 16;

        let left_idx_cell = container(text(format!("{:02}", left_idx)).size(14))
            .padding(10)
            .align_y(iced::alignment::Vertical::Center)
            .align_x(iced::alignment::Horizontal::Center)
            .height(ROOT_CELL_HEIGHT)
            .width(Length::FillPortion(1))
            .style(table_cell());

        let left_root_cell = if let Some(root) = roots.get(left_idx) {
            let root_hex = hex::encode(**root);
            let root_hex_split = format!("{}\n{}", &root_hex[..32], &root_hex[32..]);

            container(text(root_hex_split).size(10))
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
                .height(ROOT_CELL_HEIGHT)
                .width(Length::FillPortion(4))
                .style(table_cell())
        } else {
            container(text("NULL").size(12).color(OFF_WHITE.scale_alpha(0.5)))
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
                .height(ROOT_CELL_HEIGHT)
                .width(Length::FillPortion(4))
                .style(table_cell())
        };

        let right_idx_cell = container(text(format!("{:02}", right_idx)).size(14))
            .padding(10)
            .align_y(iced::alignment::Vertical::Center)
            .align_x(iced::alignment::Horizontal::Center)
            .height(ROOT_CELL_HEIGHT)
            .width(Length::FillPortion(1))
            .style(table_cell());

        let right_root_cell = if let Some(root) = roots.get(right_idx) {
            let root_hex = hex::encode(**root);
            let root_hex_split = format!("{}\n{}", &root_hex[..32], &root_hex[32..]);

            container(text(root_hex_split).size(10))
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
                .height(ROOT_CELL_HEIGHT)
                .width(Length::FillPortion(4))
                .style(table_cell())
        } else {
            container(text("NULL").size(12).color(OFF_WHITE.scale_alpha(0.5)))
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
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
