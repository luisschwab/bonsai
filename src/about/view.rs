use iced::Alignment::Center;
use iced::Element;
use iced::Fill;
use iced::Length;
use iced::Padding;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::image;
use iced::widget::row;
use iced::widget::text;

use crate::BDK_ICON_PATH;
use crate::BONSAI_ICON_DARK_PATH;
use crate::BonsaiMessage;
use crate::FLORESTA_ICON_PATH;
use crate::common::interface::button::image_button;
use crate::common::interface::font::BERKELEY_MONO_BOLD;
use crate::node::style::table_cell;

pub(crate) fn view_about<'a>() -> Element<'a, BonsaiMessage> {
    let content = column![
        container(
            image(BONSAI_ICON_DARK_PATH)
                .width(Length::Fixed(170.0))
                .height(Length::Fixed(170.0))
        )
        .style(table_cell())
        .padding(1),
        text("BONSAI").font(BERKELEY_MONO_BOLD).size(36),
        Space::new().height(10.0),
        text("Bonsai is a desktop wallet with an embbeded node, built as a showcase of `bdk_floresta`, a novel chain-source crate for BDK that leverages Floresta and Utreexo to keep a compact full node running within the application, enabling completely trustless and on-device validation and blockchain data fetching. Zero API calls means better user privacy and sovereignty.").align_x(Center),
        Space::new().height(Fill),
        text("POWERED BY").font(BERKELEY_MONO_BOLD).size(24),
        Space::new().height(15.0),
        row![
            column![
                button(
                    container(
                        image(BDK_ICON_PATH)
                            .width(Length::Fixed(170.0))
                            .height(Length::Fixed(170.0))
                    )
                )
                .on_press(BonsaiMessage::OpenLink("https://bitcoindevkit.org".to_string()))
                .padding(0)
                .style(image_button()),
                text("Bitcoin Dev Kit")
            ].align_x(Center).spacing(4),
            column![
                button(
                    container(
                        image(FLORESTA_ICON_PATH)
                            .width(Length::Fixed(170.0))
                            .height(Length::Fixed(170.0))
                    )
                )
                .on_press(BonsaiMessage::OpenLink("https://getfloresta.org".to_string()))
                .padding(0)
                .style(image_button()),
                text("Floresta")
            ].align_x(Center).spacing(4),
        ].align_y(Center).spacing(20)
    ]
    .align_x(Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Center)
        .align_y(Center)
        .padding(Padding::from([40.0, 150.0]))
        .into()
}
