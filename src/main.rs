mod node;
mod wallet;

use iced::widget::{button, column, container, row, text};
use iced::{Color, Element, Font, Length, Theme, window};
use iced::window::icon;

// Gruvbox Dark Hard colors
pub const BG: Color = Color::from_rgb(0x1d as f32 / 255.0, 0x20 as f32 / 255.0, 0x21 as f32 / 255.0);
pub const FG: Color = Color::from_rgb(0xeb as f32 / 255.0, 0xdb as f32 / 255.0, 0xb2 as f32 / 255.0);
pub const ORANGE: Color = Color::from_rgb(0xfe as f32 / 255.0, 0x80 as f32 / 255.0, 0x19 as f32 / 255.0);
pub const GREEN: Color = Color::from_rgb(0xb8 as f32 / 255.0, 0xbb as f32 / 255.0, 0x26 as f32 / 255.0);

fn main() -> iced::Result {
    let icon = icon::from_file("./assets/icon/bonsai.png").unwrap();
    iced::application("bonsai", App::update, App::view)
        .window(window::Settings {
            icon: Some(icon),
            ..Default::default()
        })
        .theme(|_| {
            Theme::custom(
                "gruvbox".to_string(),
                iced::theme::Palette {
                    background: BG,
                    text: FG,
                    primary: ORANGE,
                    success: GREEN,
                    danger: Color::from_rgb(0xfb as f32 / 255.0, 0x49 as f32 / 255.0, 0x34 as f32 / 255.0),
                },
            )
        })
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
    Node(node::Message),
    Wallet(wallet::Message),
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    #[default]
    NodeManagement,
    Wallet,
}

#[derive(Default)]
pub struct App {
    active_tab: Tab,
    node: node::Node,
    wallet: wallet::Wallet,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
            }
            Message::Node(msg) => {
                self.node.update(msg);
            }
            Message::Wallet(msg) => {
                self.wallet.update(msg);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let tabs = row![
            button(text("BDK Wallet").font(Font::MONOSPACE))
                .on_press(Message::TabSelected(Tab::Wallet))
                .style(if self.active_tab == Tab::Wallet {
                    button::primary
                } else {
                    button::secondary
                }),
            button(text("Node Management").font(Font::MONOSPACE))
                .on_press(Message::TabSelected(Tab::NodeManagement))
                .style(if self.active_tab == Tab::NodeManagement {
                    button::primary
                } else {
                    button::secondary
                }),
        ]
        .spacing(10);

        let content = match self.active_tab {
            Tab::NodeManagement => self.node.view().map(Message::Node),
            Tab::Wallet => self.wallet.view().map(Message::Wallet),
        };

        container(column![tabs, content].spacing(20).padding(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
