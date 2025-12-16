use core::fmt::Debug;

use iced::widget::{button, column, container, row, text};
use iced::window;
use iced::window::icon;
use iced::window::settings::PlatformSpecific;
use iced::window::{Icon, Level, Position, Settings};
use iced::{Element, Length, Size, Subscription, Task, Theme};

use common::interface::color::{BACKGROUND, FOREGROUND, GREEN, ORANGE, RED};
use common::interface::font::BERKELEY_MONO_REGULAR;
use common::logger::setup_logger;
use node::control::Node;
use node::control::{set_runtime_handle, start_node, stop_node};
use node::error::BonsaiNodeError;
use node::message::NodeMessage;
use wallet::placeholder::{Wallet, WalletMessage};

mod common;
mod node;
mod wallet;

const START_NODE_AUTO: bool = false;

#[derive(Debug, Clone)]
pub(crate) enum BonsaiMessage {
    SelectTab(Tab),
    Node(NodeMessage),
    Wallet(WalletMessage),
    CloseRequested,
    CloseWindow,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) enum Tab {
    #[default]
    Node,
    Wallet,
}

#[derive(Default)]
pub(crate) struct Bonsai {
    active_tab: Tab,
    node: Node,
    wallet: Wallet,
}

impl Bonsai {
    fn update(&mut self, message: BonsaiMessage) -> Task<BonsaiMessage> {
        match message {
            BonsaiMessage::SelectTab(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            BonsaiMessage::Node(msg) => self.node.update(msg).map(BonsaiMessage::Node),
            BonsaiMessage::Wallet(msg) => {
                self.wallet.update(msg);
                Task::none()
            }
            BonsaiMessage::CloseRequested => {
                if self.node.handle.is_some() {
                    let stopping_task = Task::done(BonsaiMessage::Node(NodeMessage::ShuttingDown));
                    self.node.unsubscribe();

                    // Take the handle for shutdown
                    let node_handle = self.node.handle.take().unwrap();
                    let rt_handle = node::control::get_runtime_handle().clone();

                    let shutdown_task = Task::future(async move {
                        let _ = rt_handle
                            .spawn(async move { stop_node(node_handle).await })
                            .await;

                        BonsaiMessage::CloseWindow
                    });

                    Task::batch([stopping_task, shutdown_task])
                } else {
                    Task::done(BonsaiMessage::CloseWindow)
                }
            }
            BonsaiMessage::CloseWindow => window::oldest()
                .and_then(window::close::<BonsaiMessage>)
                .discard(),
        }
    }

    fn view(&self) -> Element<'_, BonsaiMessage> {
        let tabs = row![
            button(text("Node"))
                .on_press(BonsaiMessage::SelectTab(Tab::Node))
                .style(if self.active_tab == Tab::Node {
                    button::primary
                } else {
                    button::secondary
                }),
            button(text("Wallet"))
                .on_press(BonsaiMessage::SelectTab(Tab::Wallet))
                .style(if self.active_tab == Tab::Wallet {
                    button::primary
                } else {
                    button::secondary
                }),
        ]
        .spacing(10);

        let content = match self.active_tab {
            Tab::Node => self.node.view().map(BonsaiMessage::Node),
            Tab::Wallet => self.wallet.view().map(BonsaiMessage::Wallet),
        };

        container(column![tabs, content].spacing(20).padding(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<BonsaiMessage> {
        use iced::event::{self, Event};

        let window_events = event::listen_with(|event, _status, _id| {
            if let Event::Window(window::Event::CloseRequested) = event {
                Some(BonsaiMessage::CloseRequested)
            } else {
                None
            }
        });

        let tab_subscription = match self.active_tab {
            Tab::Node => self.node.subscribe().map(BonsaiMessage::Node),
            Tab::Wallet => Subscription::none(),
        };

        Subscription::batch([window_events, tab_subscription])
    }
}

fn main() -> iced::Result {
    // Setup the logger.
    setup_logger();

    // Create a Tokio runtime for the underlying node to run on.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_name("bonsai-rt")
        .build()
        .unwrap();
    // Get Tokio's runtime handle and set it
    let rt_handle = rt.handle().clone();
    set_runtime_handle(rt_handle);
    // Get a guard to the runtime so it keeps running.
    let _guard = rt.enter();
    std::mem::forget(rt);

    // Create an [`Icon`] from a PNG.
    let icon: Icon = icon::from_file("./assets/icon/bonsai.png").unwrap();

    // Define some window [`Settings`].
    let window_settings: Settings = Settings {
        size: Size::new(1000.0, 800.0),
        position: Position::Default,
        min_size: Some(Size::new(800.0, 600.0)),
        max_size: None,
        visible: true,
        resizable: true,
        decorations: true,
        transparent: true,
        level: Level::Normal,
        icon: Some(icon),
        platform_specific: PlatformSpecific::default(),
        exit_on_close_request: false,
        maximized: false,
        fullscreen: false,
        closeable: true,
        minimizable: true,
        blur: false,
    };

    iced::application(
        || {
            let tasks = if START_NODE_AUTO {
                Task::batch([
                    Task::done(BonsaiMessage::Node(NodeMessage::Starting)),
                    Task::perform(start_node(), |result| match result {
                        Ok(handle) => BonsaiMessage::Node(NodeMessage::Running(handle)),
                        Err(e) => BonsaiMessage::Node(NodeMessage::Error(BonsaiNodeError::from(e))),
                    }),
                ])
            } else {
                Task::none()
            };

            (Bonsai::default(), tasks)
        },
        Bonsai::update,
        Bonsai::view,
    )
    .window(window_settings)
    .theme(|_: &Bonsai| {
        Theme::custom(
            "GruvboxDarkHard".to_string(),
            iced::theme::Palette {
                background: BACKGROUND,
                text: FOREGROUND,
                primary: ORANGE,
                success: GREEN,
                danger: RED,
                warning: RED,
            },
        )
    })
    .font(include_bytes!("../assets/font/BerkeleyMono-Bold.ttf").as_slice())
    .font(include_bytes!("../assets/font/BerkeleyMono-Regular.ttf").as_slice())
    .default_font(BERKELEY_MONO_REGULAR)
    .subscription(Bonsai::subscription)
    .run()
}
