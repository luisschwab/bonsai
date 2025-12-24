use iced::Element;
use iced::Length;
use iced::alignment::Horizontal::Left;
use iced::alignment::Horizontal::Right;
use iced::widget::Container;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::qr_code;
use iced::widget::row;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::tooltip;

use crate::common::interface::container::common::CELL_HEIGHT;
use crate::common::interface::container::common::SHADOW;
use crate::common::interface::container::common::shadow_container;
use crate::common::interface::container::content::button_container;
use crate::common::interface::font::BERKELEY_MONO_BOLD;
use crate::node::interface::common::TITLE_PADDING;
use crate::node::interface::common::input_field;
use crate::node::interface::common::title_container;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;

pub fn view_blocks<'a>(
    statistics: &'a Option<NodeStatistics>,
    block_height: &'a str,
) -> Element<'a, NodeMessage> {
    // Tab Title.
    let title: Container<'_, NodeMessage> = container(text("NODE BLOCKS").size(25))
        .style(title_container())
        .padding(TITLE_PADDING);

    let latest_title: Container<'_, NodeMessage> = container(text("LATEST BLOCKS").size(24));
    let latest_canvas: Container<'_, NodeMessage> = container(text("TODO"))
        .padding(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(title_container());
    let latest = container(column![latest_title, latest_canvas].spacing(5));

    let left = column![title, latest]
        .spacing(20)
        .width(Length::FillPortion(1));

    let explorer_title = container(row![
        text("BLOCK EXPLORER").size(24),
        Space::new().width(Length::Fill),
        row![
            button(
                text("<")
                    .size(16)
                    .font(BERKELEY_MONO_BOLD)
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
            )
            //.on_press(NodeMessage::CopyAccumulatorData)
            .style(button_container())
            .padding(10)
            .height(CELL_HEIGHT),
            container(
                text_input("TODO", block_height)
                    //.on_input(NodeMessage::AddPeerInputChanged)
                    .style(input_field())
                    .size(16)
                    .align_x(iced::alignment::Horizontal::Center)
                    .width(Length::Fixed(110.0))
            )
            .padding(0)
            .style(shadow_container())
            .height(CELL_HEIGHT),
            button(
                text(">")
                    .size(16)
                    .font(BERKELEY_MONO_BOLD)
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
            )
            //.on_press(NodeMessage::CopyAccumulatorData)
            .style(button_container())
            .padding(10)
            .height(CELL_HEIGHT)
        ]
        .spacing(10)
    ]);

    let explorer_canvas: Container<'_, NodeMessage> = container(text("TODO"))
        .padding(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(title_container());
    let explorer = container(column![explorer_title, explorer_canvas].spacing(5));

    let right = column![explorer].spacing(20).width(Length::FillPortion(1));

    row![left, right].spacing(20).into()
}
