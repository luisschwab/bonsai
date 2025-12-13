use iced::widget::{button, column, container, row, text};
use iced::window::{self, icon};
use iced::{Element, Font, Length, Subscription, Task, Theme};
use std::sync::Arc;
use tokio::sync::RwLock;

mod common;
mod node;
mod wallet;

use common::interface::colors::{BACKGROUND, FOREGROUND, GREEN, ORANGE, BLUE, RED};

fn main() -> iced::Result {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_name("bonsai-runtime")
        .build()
        .unwrap();

    let rt_handle = rt.handle().clone();
    node::node::set_runtime_handle(rt_handle);

    let _guard = rt.enter();
    std::mem::forget(rt);

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
                    background: BACKGROUND,
                    text: FOREGROUND,
                    primary: ORANGE,
                    success: GREEN,
                    danger: RED,
                },
            )
        })
        .subscription(App::subscription)
        .run_with(|| {
            (
                App::default(),
                Task::perform(node::node::start_node(), Message::NodeStarted),
            )
        })
}

#[derive(Clone)]
pub enum Message {
    TabSelected(Tab),
    Node(node::node::Message),
    Wallet(wallet::wallet::Message),
    NodeStarted(Result<Arc<RwLock<bdk_floresta::FlorestaNode>>, String>),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::TabSelected(tab) => f.debug_tuple("TabSelected").field(tab).finish(),
            Message::Node(msg) => f.debug_tuple("Node").field(msg).finish(),
            Message::Wallet(msg) => f.debug_tuple("Wallet").field(msg).finish(),
            Message::NodeStarted(Ok(_)) => write!(f, "NodeStarted(Ok(<handle>))"),
            Message::NodeStarted(Err(e)) => f
                .debug_tuple("NodeStarted")
                .field(&Err::<(), _>(e.clone()))
                .finish(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    #[default]
    Node,
    Wallet,
}

#[derive(Default)]
pub struct App {
    active_tab: Tab,
    node: node::node::Node,
    wallet: wallet::wallet::Wallet,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            Message::Node(msg) => {
                self.node.update(msg);
                Task::none()
            }
            Message::Wallet(msg) => {
                self.wallet.update(msg);
                Task::none()
            }
            Message::NodeStarted(Ok(handle)) => {
                self.node.update(node::node::Message::NodeReady(handle));
                Task::none()
            }
            Message::NodeStarted(Err(e)) => {
                self.node.update(node::node::Message::ErrorOccurred(e));
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let tabs = row![
            button(text("Node").font(Font::MONOSPACE))
                .on_press(Message::TabSelected(Tab::Node))
                .style(if self.active_tab == Tab::Node {
                    button::primary
                } else {
                    button::secondary
                }),
            button(text("Wallet").font(Font::MONOSPACE))
                .on_press(Message::TabSelected(Tab::Wallet))
                .style(if self.active_tab == Tab::Wallet {
                    button::primary
                } else {
                    button::secondary
                }),
        ]
        .spacing(10);

        let content = match self.active_tab {
            Tab::Node => self.node.view().map(Message::Node),
            Tab::Wallet => self.wallet.view().map(Message::Wallet),
        };

        container(column![tabs, content].spacing(20).padding(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.active_tab {
            Tab::Node => self.node.subscription().map(Message::Node),
            Tab::Wallet => Subscription::none(),
        }
    }
}
