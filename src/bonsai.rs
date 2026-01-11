//#![allow(unused)]

use core::fmt::Debug;

use bitcoin::Network;
use iced::Alignment::Center;
use iced::Element;
use iced::Event;
use iced::Length;
use iced::Padding;
use iced::Size;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::event;
use iced::theme::Palette;
use iced::time;
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
use tracing::error;
use tracing::info;

use crate::common::interface::button::sidebar_button;
use crate::common::interface::color::DARK_GREY;
use crate::common::interface::color::GREEN_SHAMROCK;
use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::color::ORANGE;
use crate::common::interface::color::PURPLE;
use crate::common::interface::color::RED;
use crate::common::interface::color::WHITE;
use crate::common::interface::color::network_color;
use crate::common::interface::color::pulse_color;
use crate::common::interface::constants::CONTENT_PADDING;
use crate::common::interface::constants::CONTENT_SPACING;
use crate::common::interface::constants::HEADER_HEIGHT;
use crate::common::interface::constants::HEADER_PADDING;
use crate::common::interface::constants::SIDEBAR_BUTTON_HEIGHT;
use crate::common::interface::constants::SIDEBAR_BUTTON_SPACING;
use crate::common::interface::constants::SIDEBAR_PADDING;
use crate::common::interface::constants::SIDEBAR_WIDTH;
use crate::common::interface::constants::WINDOW_PADDING;
use crate::common::interface::container::content_container;
use crate::common::interface::container::header_container;
use crate::common::interface::container::sidebar_container;
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
use crate::node::message::NodeMessage;
use crate::node::style::table_cell;
use crate::settings::bonsai_settings::AUTO_START_NODE;
use crate::settings::bonsai_settings::BonsaiSettings;
use crate::settings::bonsai_settings::BonsaiSettingsMessage;
use crate::settings::bonsai_settings::SETTINGS_FILE;
use crate::wallet::placeholder::Wallet;
use crate::wallet::placeholder::WalletMessage;

pub(crate) mod common;
pub(crate) mod node;
pub(crate) mod settings;
pub(crate) mod wallet;

pub(crate) const APP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
pub(crate) const GEOIP_ASN_DB_PATH: &str = "./assets/geoip/GeoLite2-ASN.mmdb";
pub(crate) const GEOIP_CITY_DB_PATH: &str = "./assets/geoip/GeoLite2-City.mmdb";
pub(crate) const BONSAI_ICON_DARK_PATH: &str = "./assets/icon/bonsai-dark.png";
//pub(crate) const BONSAI_ICON_LIGHT_PATH: &str = "./assets/icon/bonsai-light.png";

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) enum Tab {
    Wallet,
    #[default]
    NodeStatistics,
    NodeNetwork,
    NodeBlockchain,
    NodeUtreexo,
    Settings,
    About,
}

#[derive(Debug, Clone)]
pub(crate) enum BonsaiMessage {
    AnimationTick,
    SelectTab(Tab),
    CloseRequested,
    CloseWindow,
    Settings(BonsaiSettingsMessage),
    Node(NodeMessage),
    BdkWallet(WalletMessage),
}

#[derive(Default)]
pub(crate) struct Bonsai {
    pub(crate) app_clock: usize,
    pub(crate) active_tab: Tab,
    pub(crate) node: Node,
    pub(crate) wallet: Wallet,
    pub(crate) settings: BonsaiSettings,
}

impl Bonsai {
    fn view(&self) -> Element<'_, BonsaiMessage> {
        let node_status = &self.node.status;
        let status_color = match node_status {
            NodeStatus::Starting => pulse_color(GREEN_SHAMROCK, self.app_clock),
            NodeStatus::Running => GREEN_SHAMROCK,
            NodeStatus::Inactive => OFF_WHITE,
            NodeStatus::ShuttingDown => pulse_color(RED, self.app_clock),
            NodeStatus::Failed(_) => RED,
        };
        let blocks = self.node.statistics.as_ref().map(|s| s.blocks).unwrap_or(0);
        let network_color = network_color(NETWORK);

        let header = container(
            container(
                row![
                    // Left.
                    container(
                        row![
                            container(image(BONSAI_ICON_DARK_PATH).height(Length::Fill))
                                .padding(1)
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
            button(text("WALLET").size(20).align_y(Center).align_x(Center))
                .on_press(BonsaiMessage::SelectTab(Tab::Wallet))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::Wallet, ORANGE)),
            button(text("STATISTICS").size(20).align_y(Center).align_x(Center))
                .on_press(BonsaiMessage::SelectTab(Tab::NodeStatistics))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(
                    self.active_tab == Tab::NodeStatistics,
                    GREEN_SHAMROCK
                )),
            button(text("NETWORK").size(20).align_y(Center).align_x(Center))
                .on_press(BonsaiMessage::SelectTab(Tab::NodeNetwork))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(
                    self.active_tab == Tab::NodeNetwork,
                    GREEN_SHAMROCK
                )),
            button(text("UTREEXO").size(20).align_y(Center).align_x(Center))
                .on_press(BonsaiMessage::SelectTab(Tab::NodeUtreexo))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(
                    self.active_tab == Tab::NodeUtreexo,
                    GREEN_SHAMROCK
                )),
            button(text("BLOCKCHAIN").size(20).align_y(Center).align_x(Center))
                .on_press(BonsaiMessage::SelectTab(Tab::NodeBlockchain))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(
                    self.active_tab == Tab::NodeBlockchain,
                    GREEN_SHAMROCK
                )),
            button(text("SETTINGS").size(20).align_y(Center).align_x(Center))
                .on_press(BonsaiMessage::SelectTab(Tab::Settings))
                .height(SIDEBAR_BUTTON_HEIGHT)
                .width(Length::Fill)
                .style(sidebar_button(self.active_tab == Tab::Settings, PURPLE)),
            button(text("ABOUT").size(20).align_y(Center).align_x(Center))
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
            Tab::Wallet => self.wallet.view().map(BonsaiMessage::BdkWallet),
            Tab::NodeStatistics => self
                .node
                .view_tab(self.active_tab, self.app_clock)
                .map(BonsaiMessage::Node),
            Tab::NodeNetwork => self
                .node
                .view_tab(self.active_tab, self.app_clock)
                .map(BonsaiMessage::Node),
            Tab::NodeBlockchain => self
                .node
                .view_tab(self.active_tab, self.app_clock)
                .map(BonsaiMessage::Node),
            Tab::NodeUtreexo => self
                .node
                .view_tab(self.active_tab, self.app_clock)
                .map(BonsaiMessage::Node),
            Tab::Settings => self.settings.view().map(BonsaiMessage::Settings),
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
            BonsaiMessage::AnimationTick => {
                self.app_clock = self.app_clock.wrapping_add(1);
                Task::none()
            }
            BonsaiMessage::BdkWallet(msg) => {
                self.wallet.update(msg);
                Task::none()
            }
            BonsaiMessage::CloseRequested => {
                if let Err(e) = self.settings.save() {
                    eprintln!("Failed to save settings on close: {}", e);
                }

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
            BonsaiMessage::Node(msg) => {
                // Save settings when node shuts down or restarts
                match &msg {
                    NodeMessage::Shutdown | NodeMessage::Restart => {
                        if let Err(e) = self.settings.save() {
                            eprintln!("Failed to save settings: {}", e);
                        }
                    }
                    NodeMessage::ConfigUsed(config) => {
                        // Update settings with the actual config used by the node
                        self.settings.update_from_config(config);
                        if let Err(e) = self.settings.save() {
                            eprintln!("Failed to save actual node config: {}", e);
                        }
                    }
                    _ => {}
                }

                self.node.update(msg).map(BonsaiMessage::Node)
            }
            BonsaiMessage::Settings(msg) => {
                // Check if it's a restart request before updating
                let should_restart = matches!(msg, BonsaiSettingsMessage::RestartNode);

                let task = self.settings.update(msg).map(BonsaiMessage::Settings);

                if should_restart {
                    // Update the node config before restarting
                    let network = self.settings.bonsai.network.unwrap_or(NETWORK);
                    let node_config = self
                        .settings
                        .get_node_config(network, &BonsaiSettings::base_dir());
                    self.node.config = Some(node_config);

                    // Trigger node restart
                    let restart_task = Task::done(BonsaiMessage::Node(NodeMessage::Restart));
                    Task::batch([task, restart_task])
                } else {
                    task
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<BonsaiMessage> {
        let animation_timer =
            time::every(std::time::Duration::from_millis(32)).map(|_| BonsaiMessage::AnimationTick);

        let window_events = event::listen_with(|event, _status, _id| {
            if let Event::Window(window::Event::CloseRequested) = event {
                Some(BonsaiMessage::CloseRequested)
            } else {
                None
            }
        });

        let tab_subscription = match self.active_tab {
            Tab::Wallet => Subscription::none(),
            Tab::NodeStatistics => self.node.subscribe().map(BonsaiMessage::Node),
            Tab::NodeNetwork => self.node.subscribe().map(BonsaiMessage::Node),
            Tab::NodeBlockchain => self.node.subscribe().map(BonsaiMessage::Node),
            Tab::NodeUtreexo => self.node.subscribe().map(BonsaiMessage::Node),
            Tab::Settings => Subscription::none(),
            Tab::About => unimplemented!(),
        };

        Subscription::batch([animation_timer, window_events, tab_subscription])
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
    let icon: Icon = icon::from_file(BONSAI_ICON_DARK_PATH).unwrap();

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

    // Load [`BonsaiSettings`] from disk.
    let mut settings = BonsaiSettings::load();

    // Check if this is the first run by seeing if the file exists
    let settings_file = BonsaiSettings::base_dir().join(SETTINGS_FILE);
    let is_first_run = !settings_file.exists();

    // On first run, populate settings with the actual config that will be used
    // and save it to disk
    if is_first_run {
        let network = settings.bonsai.network.unwrap_or(Network::Signet);

        let node_config = settings.get_node_config(network, &BonsaiSettings::base_dir());
        settings.update_from_config(&node_config);

        match settings.save() {
            Ok(_) => {
                info!(
                    "Successfully saved default settings to {}",
                    settings_file.to_string_lossy()
                );
            }
            Err(e) => {
                error!(
                    "Failed to save default settings to {}: {}",
                    settings_file.to_string_lossy(),
                    e
                );
            }
        }
    }

    let auto_start_node = settings.node.auto_start.unwrap_or(AUTO_START_NODE);
    let network = settings.bonsai.network.unwrap_or(Network::Signet);
    let node_config = settings.get_node_config(network, &BonsaiSettings::base_dir());

    iced::application(
        move || {
            let bonsai = Bonsai {
                active_tab: Tab::default(),
                app_clock: usize::default(),
                settings: settings.clone(),
                node: Node {
                    config: Some(node_config.clone()),
                    log_capture: log_capture.clone(),
                    geoip_reader: GeoIpReader::new(GEOIP_ASN_DB_PATH, GEOIP_CITY_DB_PATH).ok(),
                    block_explorer_height_str: String::from("0"),
                    ..Node::default()
                },
                wallet: Wallet::default(),
            };

            let tasks = if auto_start_node {
                let network = settings.bonsai.network.unwrap_or(Network::Signet);
                let node_config = settings.get_node_config(network, &BonsaiSettings::base_dir());

                Task::batch([
                    Task::done(BonsaiMessage::Node(NodeMessage::Starting)),
                    Task::perform(start_node(node_config), |result| match result {
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
                success: GREEN_SHAMROCK,
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
