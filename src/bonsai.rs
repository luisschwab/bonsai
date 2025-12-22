// TODO: remove me after node stuff is done
#![allow(unused)]

use core::fmt::Debug;

use bdk_wallet::bitcoin::Network;
use iced::Element;
use iced::Length;
use iced::Padding;
use iced::Size;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::theme::Palette;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::image;
use iced::widget::row;
use iced::widget::text;
use iced::window;
use iced::window::Icon;
use iced::window::Level;
use iced::window::Position;
use iced::window::Settings;
use iced::window::icon;
use iced::window::settings::PlatformSpecific;
use tokio::runtime::Handle;

use crate::common::interface::color::BLUE;
use crate::common::interface::color::DARK_GREY;
use crate::common::interface::color::GREEN;
use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::color::ORANGE;
use crate::common::interface::color::PURPLE;
use crate::common::interface::color::RED;
use crate::common::interface::color::WHITE;
use crate::common::interface::color::YELLOW;
use crate::common::interface::container::content::CONTENT_PADDING;
use crate::common::interface::container::content::CONTENT_SPACING;
use crate::common::interface::container::content::content_container;
use crate::common::interface::container::header::HEADER_HEIGHT;
use crate::common::interface::container::header::HEADER_PADDING;
use crate::common::interface::container::header::header_container;
use crate::common::interface::container::sidebar::SIDEBAR_BUTTON_HEIGHT;
use crate::common::interface::container::sidebar::SIDEBAR_BUTTON_SPACING;
use crate::common::interface::container::sidebar::SIDEBAR_PADDING;
use crate::common::interface::container::sidebar::SIDEBAR_WIDTH;
use crate::common::interface::container::sidebar::sidebar_button;
use crate::common::interface::container::sidebar::sidebar_container;
use crate::common::interface::container::window::WINDOW_PADDING;
use crate::common::interface::font::BERKELEY_MONO_BOLD;
use crate::common::interface::font::BERKELEY_MONO_REGULAR;
use crate::common::logger::setup_logger;
use crate::common::util::format_thousands;
use crate::node::control::NETWORK;
use crate::node::control::Node;
use crate::node::control::NodeStatus;
use crate::node::control::start_node;
use crate::node::control::stop_node;
use crate::node::error::BonsaiNodeError;
use crate::node::geoip::GeoIpReader;
use crate::node::interface::common::table_cell;
use crate::node::message::NodeMessage;
use crate::wallet::ark::placeholder::ArkWallet;
use crate::wallet::ark::placeholder::ArkWalletMessage;
use crate::wallet::bdk::placeholder::BDKWallet;
use crate::wallet::bdk::placeholder::BDKWalletMessage;
use crate::wallet::phoenixd::placeholder::Phoenixd;
use crate::wallet::phoenixd::placeholder::PhoenixdMessage;

pub(crate) mod common;
pub(crate) mod node;
pub(crate) mod wallet;

const START_NODE_AUTO: bool = true;
const APP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
const GEOIP_ASN_DB: &str = "./assets/geoip/GeoLite2-ASN.mmdb";
const GEOIP_CITY_DB: &str = "./assets/geoip/GeoLite2-City.mmdb";

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) enum Tab {
    BDKWallet,
    Phoenixd,
    Ark,
    #[default]
    NodeOverview,
    NodeP2P,
    NodeBlocks,
    NodeUtreexo,
    NodeMempool,
    NodeSettings,
    About,
}

#[derive(Debug, Clone)]
pub(crate) enum BonsaiMessage {
    SelectTab(Tab),
    CloseRequested,
    CloseWindow,
    BdkWallet(BDKWalletMessage),
    Phoenixd(PhoenixdMessage),
    ArkWallet(ArkWalletMessage),
    Node(NodeMessage),
}

#[derive(Default)]
pub(crate) struct Bonsai {
    pub(crate) active_tab: Tab,
    pub(crate) node: Node,
    pub(crate) onchain_wallet: BDKWallet,
    pub(crate) lightning_wallet: Phoenixd,
    pub(crate) ark_wallet: ArkWallet,
}

impl Bonsai {
    fn view(&self) -> Element<'_, BonsaiMessage> {
        let node_status = &self.node.status;
        let status_color = match node_status {
            NodeStatus::Starting | NodeStatus::Running => GREEN,
            NodeStatus::Inactive => OFF_WHITE,
            NodeStatus::ShuttingDown | NodeStatus::Failed(_) => RED,
        };
        let blocks = self.node.statistics.as_ref().map(|s| s.blocks).unwrap_or(0);
        let network_color = match NETWORK {
            Network::Bitcoin => ORANGE,
            Network::Signet => PURPLE,
            Network::Testnet | Network::Testnet4 => BLUE,
            Network::Regtest => OFF_WHITE,
        };

        let header = container(
            container(
                row![
                    // Left.
                    container(
                        row![
                            container(image("assets/icon/bonsai.png").height(Length::Fill))
                                .padding(5)
                                .style(table_cell()),
                            column![
                                row![
                                    text("BONSAI").size(36).font(BERKELEY_MONO_BOLD),
                                    text("盆栽").size(32).font(BERKELEY_MONO_REGULAR),
                                ]
                                .spacing(10)
                                .align_y(iced::Alignment::Center),
                                Space::new().height(Length::Fill),
                                text("UTREEXO-AWARE BITCOIN\nWALLET WITH AN EMBEDDED NODE")
                                    .size(12)
                            ]
                            .spacing(-1.5)
                            .height(Length::Fill),
                        ]
                        .spacing(10)
                        .height(Length::Fill),
                    )
                    .padding(Padding::from([-4.0, 5.0]))
                    .height(Length::Fill),
                    Space::new().width(Length::Fill),
                    // Right.
                    row![
                        column![
                            text(node_status.to_string())
                                .size(12)
                                .font(BERKELEY_MONO_BOLD)
                                .color(status_color),
                            text(NETWORK.to_string().to_uppercase())
                                .size(12)
                                .font(BERKELEY_MONO_BOLD)
                                .color(network_color),
                            text(format_thousands(blocks))
                                .size(12)
                                .font(BERKELEY_MONO_BOLD),
                            text(APP_VERSION).size(12).font(BERKELEY_MONO_BOLD),
                        ]
                        .spacing(2)
                        .align_x(iced::Alignment::End),
                        column![
                            text("NODE").size(12),
                            text("NETWORK").size(12),
                            text("HEIGHT").size(12),
                            text("VERSION").size(12),
                        ]
                        .spacing(2)
                        .align_x(iced::Alignment::Start),
                    ]
                    .spacing(4)
                    .padding(0)
                    .align_y(iced::Alignment::Center),
                ]
                .align_y(iced::Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .height(Length::Fixed(HEADER_HEIGHT))
        .width(Length::Fill)
        .padding(HEADER_PADDING)
        .style(header_container());

        let tabs = column![
            button(text("ONCHAIN WALLET"))
                .on_press(BonsaiMessage::SelectTab(Tab::BDKWallet))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::BDKWallet, ORANGE)),
            //button(text("LIGHTNING WALLET"))
            //    .on_press(BonsaiMessage::SelectTab(Tab::Phoenixd))
            //    .height(SIDEBAR_BUTTON_HEIGHT)
            //    .width(Length::Fill)
            //    .style(sidebar_button(self.active_tab == Tab::Phoenixd, YELLOW)),
            //button(text("ARK WALLET"))
            //    .on_press(BonsaiMessage::SelectTab(Tab::Ark))
            //    .height(SIDEBAR_BUTTON_HEIGHT)
            //    .width(Length::Fill)
            //    .style(sidebar_button(self.active_tab == Tab::Ark, PURPLE)),
            button(text("NODE OVERVIEW"))
                .on_press(BonsaiMessage::SelectTab(Tab::NodeOverview))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::NodeOverview, GREEN)),
            button(text("NODE P2P"))
                .on_press(BonsaiMessage::SelectTab(Tab::NodeP2P))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::NodeP2P, GREEN)),
            button(text("NODE UTREEXO"))
                .on_press(BonsaiMessage::SelectTab(Tab::NodeUtreexo))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::NodeUtreexo, GREEN)),
            button(text("NODE BLOCKS"))
                //.on_press(BonsaiMessage::SelectTab(Tab::NodeBlocks))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::NodeBlocks, GREEN)),
            button(text("NODE MEMPOOL"))
                //.on_press(BonsaiMessage::SelectTab(Tab::NodeMempool))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::NodeMempool, GREEN)),
            button(text("NODE SETTINGS"))
                //.on_press(BonsaiMessage::SelectTab(Tab::NodeSettings))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::NodeSettings, GREEN)),
            button(text("ABOUT BONSAI"))
                //.on_press(BonsaiMessage::SelectTab(Tab::About))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::About, WHITE)),
        ]
        .spacing(SIDEBAR_BUTTON_SPACING);

        let sidebar = container(tabs)
            .padding(SIDEBAR_PADDING)
            .width(Length::Fixed(SIDEBAR_WIDTH))
            .height(Length::Fill)
            .style(sidebar_container());

        let content = match self.active_tab {
            Tab::BDKWallet => self.onchain_wallet.view().map(BonsaiMessage::BdkWallet),
            Tab::Phoenixd => self.lightning_wallet.view().map(BonsaiMessage::Phoenixd),
            Tab::Ark => self.ark_wallet.view().map(BonsaiMessage::ArkWallet),
            Tab::NodeOverview
            | Tab::NodeP2P
            | Tab::NodeBlocks
            | Tab::NodeUtreexo
            | Tab::NodeMempool
            | Tab::NodeSettings => self.node.view_tab(self.active_tab).map(BonsaiMessage::Node),
            Tab::About => unimplemented!(),
        };

        let content_area = container(content)
            .padding(CONTENT_PADDING)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(content_container());

        let main_layout = row![sidebar, content_area].spacing(CONTENT_SPACING);

        let body = column![header, main_layout].spacing(CONTENT_SPACING);

        let inner = container(body)
            .padding(WINDOW_PADDING)
            .width(Length::Fill)
            .height(Length::Fill);

        container(inner)
            .padding(WINDOW_PADDING)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: BonsaiMessage) -> Task<BonsaiMessage> {
        match message {
            BonsaiMessage::SelectTab(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            BonsaiMessage::BdkWallet(msg) => {
                self.onchain_wallet.update(msg);
                Task::none()
            }
            BonsaiMessage::CloseRequested => {
                if self.node.handle.is_some() {
                    let stopping_task = Task::done(BonsaiMessage::Node(NodeMessage::ShuttingDown));
                    self.node.unsubscribe();

                    // Take the handle for shutdown
                    let node_handle = self.node.handle.take().unwrap();
                    let rt_handle = Handle::current();

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
            BonsaiMessage::Node(msg) => self.node.update(msg).map(BonsaiMessage::Node),
        }
    }

    fn subscription(&self) -> Subscription<BonsaiMessage> {
        use iced::event::Event;
        use iced::event::{self};

        let window_events = event::listen_with(|event, _status, _id| {
            if let Event::Window(window::Event::CloseRequested) = event {
                Some(BonsaiMessage::CloseRequested)
            } else {
                None
            }
        });

        let tab_subscription = match self.active_tab {
            Tab::BDKWallet | Tab::Phoenixd | Tab::Ark => Subscription::none(),
            Tab::NodeOverview
            | Tab::NodeP2P
            | Tab::NodeBlocks
            | Tab::NodeUtreexo
            | Tab::NodeMempool
            | Tab::NodeSettings => self.node.subscribe().map(BonsaiMessage::Node),
            Tab::About => unimplemented!(),
        };

        Subscription::batch([window_events, tab_subscription])
    }
}

fn main() -> iced::Result {
    // Setup the logger.
    let log_capture = setup_logger();

    // Create a Tokio runtime for the underlying node to run on.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_name("bonsai-rt")
        .build()
        .unwrap();
    // Get a guard to the runtime so it keeps running.
    let _guard = rt.enter();
    std::mem::forget(rt);

    // Create an [`Icon`] from a PNG.
    let icon: Icon = icon::from_file("./assets/icon/bonsai.png").unwrap();

    // Define some window [`Settings`].
    let window_settings: Settings = Settings {
        size: Size::new(1200.0, 850.0),
        position: Position::Default,
        min_size: Some(Size::new(1200.0, 850.0)),
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
        move || {
            let bonsai = Bonsai {
                active_tab: Tab::default(),
                node: Node {
                    log_capture: log_capture.clone(),
                    geoip_reader: GeoIpReader::new(GEOIP_ASN_DB, GEOIP_CITY_DB).ok(),
                    ..Node::default()
                },
                onchain_wallet: BDKWallet::default(),
                lightning_wallet: Phoenixd::default(),
                ark_wallet: ArkWallet::default(),
            };

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

            (bonsai, tasks)
        },
        Bonsai::update,
        Bonsai::view,
    )
    .window(window_settings)
    .theme(|_: &Bonsai| {
        Theme::custom(
            "Bonsai".to_string(),
            Palette {
                background: DARK_GREY,
                text: OFF_WHITE,
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
