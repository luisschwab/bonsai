use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::path::PathBuf;

use bdk_floresta::ChainParams;
use bdk_floresta::UtreexoNodeConfig;
use bitcoin::Network;
use iced::Element;
use iced::Task;
use serde::Deserialize;
use serde::Serialize;
use tracing::error;

pub(crate) const AUTO_START_NODE: bool = false;
pub(crate) const SETTINGS_FILE: &str = "bonsai.toml";

#[derive(Debug, Clone, Default)]
pub(crate) enum BonsaiSettingsMessage {
    // Application-wide settings.
    NetworkChanged(Network),

    // Node specific settings.
    AutoStartChanged(bool),

    // Network Specific Node Settings.
    UseAssumeUtreexoChanged(bool),
    PowFraudProofsChanged(bool),
    BackfillChanged(bool),
    UserAgentInputChanged(String),
    AllowV1FallbackChanged(bool),
    FixedPeerInputChanged(String),
    ProxyInputChanged(String),
    MaxBanscoreChanged(String),
    MaxOutboundChanged(String),
    MaxInflightChanged(String),
    DisableDnsSeedsChanged(bool),

    // Actions
    SaveSettings,
    RestartNode,
    #[default]
    ClearRestartFlag,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct BonsaiSettings {
    #[serde(default)]
    pub(crate) bonsai: BonsaiAppSettings,
    #[serde(default)]
    pub(crate) node: NodeSettings,

    /// Whether the node needs a restart to apply configuration changes.
    #[serde(skip)]
    pub(crate) node_restart_required: bool,

    /// Whether we have configuration changes that need to be saved.
    #[serde(skip)]
    pub(crate) unsaved_changes: bool,

    #[serde(skip)]
    pub(crate) user_agent_input: String,
    #[serde(skip)]
    pub(crate) fixed_peer_input: String,
    #[serde(skip)]
    pub(crate) proxy_input: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct BonsaiAppSettings {
    pub(crate) network: Option<Network>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct NodeSettings {
    pub(crate) auto_start: Option<bool>,

    #[serde(flatten)]
    pub(crate) network_configs: NetworkConfigs,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct NetworkConfigs {
    #[serde(default)]
    pub(crate) bitcoin: NodeNetworkSpecific,
    #[serde(default)]
    pub(crate) signet: NodeNetworkSpecific,
    #[serde(default)]
    pub(crate) testnet3: NodeNetworkSpecific,
    #[serde(default)]
    pub(crate) testnet4: NodeNetworkSpecific,
    #[serde(default)]
    pub(crate) regtest: NodeNetworkSpecific,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct NodeNetworkSpecific {
    pub(crate) use_assume_utreexo: Option<bool>,
    pub(crate) pow_fraud_proofs: Option<bool>,
    pub(crate) backfill: Option<bool>,
    pub(crate) user_agent: Option<String>,
    pub(crate) allow_v1_fallback: Option<bool>,
    pub(crate) fixed_peer: Option<String>,
    pub(crate) max_banscore: Option<u32>,
    pub(crate) max_outbound: Option<u32>,
    pub(crate) max_inflight: Option<u32>,
    pub(crate) disable_dns_seeds: Option<bool>,
    pub(crate) proxy: Option<SocketAddr>,
}

impl NodeNetworkSpecific {
    /// Convert to UtreexoNodeConfig, using defaults where options are None
    pub(crate) fn to_config(&self, network: Network, data_dir: PathBuf) -> UtreexoNodeConfig {
        let default = UtreexoNodeConfig {
            network,
            datadir: String::from(data_dir.to_string_lossy()),
            ..Default::default()
        };

        // Get assume_utreexo value based on network if enabled
        let assume_utreexo = if self.use_assume_utreexo.unwrap_or(false) {
            Some(ChainParams::get_assume_utreexo(network))
        } else {
            None
        };

        UtreexoNodeConfig {
            network,
            datadir: String::from(data_dir.to_string_lossy()),
            assume_utreexo,
            pow_fraud_proofs: self.pow_fraud_proofs.unwrap_or(default.pow_fraud_proofs),
            backfill: self.backfill.unwrap_or(true),
            user_agent: self.user_agent.clone().unwrap_or(default.user_agent),
            allow_v1_fallback: self.allow_v1_fallback.unwrap_or(default.allow_v1_fallback),
            fixed_peer: self.fixed_peer.clone().or(default.fixed_peer),
            max_banscore: self.max_banscore.unwrap_or(default.max_banscore),
            max_outbound: self.max_outbound.unwrap_or(default.max_outbound),
            max_inflight: self.max_inflight.unwrap_or(default.max_inflight),
            disable_dns_seeds: self.disable_dns_seeds.unwrap_or(default.disable_dns_seeds),
            proxy: self.proxy.or(default.proxy),
            compact_filters: true,
            filter_start_height: Some(0),
        }
    }

    /// Create from UtreexoNodeConfig
    pub(crate) fn from_config(config: &UtreexoNodeConfig) -> Self {
        NodeNetworkSpecific {
            use_assume_utreexo: Some(config.assume_utreexo.is_some()),
            pow_fraud_proofs: Some(config.pow_fraud_proofs),
            backfill: Some(config.backfill),
            user_agent: Some(config.user_agent.clone()),
            allow_v1_fallback: Some(config.allow_v1_fallback),
            fixed_peer: config.fixed_peer.clone(),
            max_banscore: Some(config.max_banscore),
            max_outbound: Some(config.max_outbound),
            max_inflight: Some(config.max_inflight),
            disable_dns_seeds: Some(config.disable_dns_seeds),
            proxy: config.proxy,
        }
    }
}

impl NodeSettings {
    /// Get the network config for a given network
    pub(crate) fn get_network_config(&self, network: Network) -> &NodeNetworkSpecific {
        match network {
            Network::Bitcoin => &self.network_configs.bitcoin,
            Network::Signet => &self.network_configs.signet,
            Network::Testnet4 => &self.network_configs.testnet4,
            Network::Regtest => &self.network_configs.regtest,
            _ => unreachable!(),
        }
    }

    /// Get mutable network config for a given network
    pub(crate) fn get_network_config_mut(&mut self, network: Network) -> &mut NodeNetworkSpecific {
        match network {
            Network::Bitcoin => &mut self.network_configs.bitcoin,
            Network::Signet => &mut self.network_configs.signet,
            Network::Testnet4 => &mut self.network_configs.testnet4,
            Network::Regtest => &mut self.network_configs.regtest,
            _ => unreachable!(),
        }
    }
}

impl BonsaiSettings {
    pub(crate) fn base_dir() -> PathBuf {
        dirs::home_dir()
            .expect("Could not find home")
            .join(".bonsai")
    }

    /// Load settings from disk, or return default if file doesn't exist
    pub(crate) fn load() -> Self {
        let path = Self::base_dir().join(SETTINGS_FILE);

        if !path.exists() {
            return Self::default();
        }

        match std::fs::read_to_string(&path) {
            Ok(contents) => {
                let mut settings: Self = toml::from_str(&contents).unwrap_or_default();
                settings.node_restart_required = false;

                // Initialize input fields with current values
                let network = settings.bonsai.network.unwrap_or(Network::Signet);
                let config = settings.node.get_network_config(network);
                settings.user_agent_input = config.user_agent.clone().unwrap_or_default();
                settings.fixed_peer_input = config.fixed_peer.clone().unwrap_or_default();
                settings.proxy_input = config.proxy.map(|p| p.to_string()).unwrap_or_default();

                settings
            }
            Err(_) => Self::default(),
        }
    }

    /// Save settings to disk
    pub(crate) fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data_directory = Self::base_dir();
        let settings_path = data_directory.join(SETTINGS_FILE);

        match fs::create_dir_all(Self::base_dir()) {
            Ok(_) => {}
            Err(e) => {
                error!(
                    "Failed to create data directory at {}: {}",
                    data_directory.to_string_lossy(),
                    e
                );
            }
        }

        let settings_toml = toml::to_string_pretty(self)?;
        match fs::write(settings_path.clone(), settings_toml) {
            Ok(_) => {}
            Err(e) => {
                error!(
                    "Failed to write settings file to {}: {}",
                    settings_path.to_string_lossy(),
                    e
                );
            }
        };

        Ok(())
    }

    /// Get the [`UtreexoNodeConfig`] for starting the node.
    pub(crate) fn get_node_config(&self, network: Network, data_dir: &Path) -> UtreexoNodeConfig {
        let network = self.bonsai.network.unwrap_or(network);
        let data_dir = data_dir.join(network.to_string());

        let network_config = self.node.get_network_config(network);
        network_config.to_config(network, data_dir)
    }

    /// Update settings from a UtreexoNodeConfig (called after first run)
    pub(crate) fn update_from_config(&mut self, config: &UtreexoNodeConfig) {
        self.bonsai.network = Some(config.network);

        let network_config = self.node.get_network_config_mut(config.network);
        *network_config = NodeNetworkSpecific::from_config(config);
    }

    pub(crate) fn view(&self) -> Element<'_, BonsaiSettingsMessage> {
        use crate::settings::view::view_settings;
        view_settings(self)
    }

    pub(crate) fn update(&mut self, message: BonsaiSettingsMessage) -> Task<BonsaiSettingsMessage> {
        match message {
            BonsaiSettingsMessage::NetworkChanged(network) => {
                if self.bonsai.network != Some(network) {
                    self.bonsai.network = Some(network);
                    self.node_restart_required = true;
                    self.unsaved_changes = true;
                }
                Task::none()
            }

            BonsaiSettingsMessage::AutoStartChanged(enabled) => {
                self.node.auto_start = Some(enabled);
                self.unsaved_changes = true;
                Task::none()
            }

            BonsaiSettingsMessage::UseAssumeUtreexoChanged(enabled) => {
                let network = self.bonsai.network.unwrap_or(Network::Signet);
                let config = self.node.get_network_config_mut(network);
                if config.use_assume_utreexo != Some(enabled) {
                    config.use_assume_utreexo = Some(enabled);
                    self.node_restart_required = true;
                    self.unsaved_changes = true;
                }
                Task::none()
            }

            BonsaiSettingsMessage::PowFraudProofsChanged(enabled) => {
                let network = self.bonsai.network.unwrap_or(Network::Signet);
                let config = self.node.get_network_config_mut(network);
                if config.pow_fraud_proofs != Some(enabled) {
                    config.pow_fraud_proofs = Some(enabled);
                    self.node_restart_required = true;
                    self.unsaved_changes = true;
                }
                Task::none()
            }

            BonsaiSettingsMessage::BackfillChanged(enabled) => {
                let network = self.bonsai.network.unwrap_or(Network::Signet);
                let config = self.node.get_network_config_mut(network);
                if config.backfill != Some(enabled) {
                    config.backfill = Some(enabled);
                    self.node_restart_required = true;
                    self.unsaved_changes = true;
                }
                Task::none()
            }

            BonsaiSettingsMessage::UserAgentInputChanged(value) => {
                self.user_agent_input = value;
                Task::none()
            }

            BonsaiSettingsMessage::AllowV1FallbackChanged(enabled) => {
                let network = self.bonsai.network.unwrap_or(Network::Signet);
                let config = self.node.get_network_config_mut(network);
                if config.allow_v1_fallback != Some(enabled) {
                    config.allow_v1_fallback = Some(enabled);
                    self.node_restart_required = true;
                    self.unsaved_changes = true;
                }
                Task::none()
            }

            BonsaiSettingsMessage::FixedPeerInputChanged(value) => {
                self.fixed_peer_input = value;
                Task::none()
            }

            BonsaiSettingsMessage::ProxyInputChanged(value) => {
                self.proxy_input = value;
                Task::none()
            }

            BonsaiSettingsMessage::MaxBanscoreChanged(value) => {
                if let Ok(banscore) = value.parse::<u32>() {
                    let network = self.bonsai.network.unwrap_or(Network::Signet);
                    let config = self.node.get_network_config_mut(network);
                    if config.max_banscore != Some(banscore) {
                        config.max_banscore = Some(banscore);
                        self.node_restart_required = true;
                        self.unsaved_changes = true;
                    }
                }
                Task::none()
            }

            BonsaiSettingsMessage::MaxOutboundChanged(value) => {
                if let Ok(outbound) = value.parse::<u32>() {
                    let network = self.bonsai.network.unwrap_or(Network::Signet);
                    let config = self.node.get_network_config_mut(network);
                    if config.max_outbound != Some(outbound) {
                        config.max_outbound = Some(outbound);
                        self.node_restart_required = true;
                        self.unsaved_changes = true;
                    }
                }
                Task::none()
            }

            BonsaiSettingsMessage::MaxInflightChanged(value) => {
                if let Ok(inflight) = value.parse::<u32>() {
                    let network = self.bonsai.network.unwrap_or(Network::Signet);
                    let config = self.node.get_network_config_mut(network);
                    if config.max_inflight != Some(inflight) {
                        config.max_inflight = Some(inflight);
                        self.node_restart_required = true;
                        self.unsaved_changes = true;
                    }
                }
                Task::none()
            }

            BonsaiSettingsMessage::DisableDnsSeedsChanged(enabled) => {
                let network = self.bonsai.network.unwrap_or(Network::Signet);
                let config = self.node.get_network_config_mut(network);
                if config.disable_dns_seeds != Some(enabled) {
                    config.disable_dns_seeds = Some(enabled);
                    self.node_restart_required = true;
                    self.unsaved_changes = true;
                }
                Task::none()
            }

            BonsaiSettingsMessage::SaveSettings => {
                let network = self.bonsai.network.unwrap_or(Network::Signet);
                let config = self.node.get_network_config_mut(network);

                if !self.user_agent_input.is_empty()
                    && Some(&self.user_agent_input) != config.user_agent.as_ref()
                {
                    config.user_agent = Some(self.user_agent_input.clone());
                    self.node_restart_required = true;
                }

                let fixed_peer_value = if self.fixed_peer_input.is_empty() {
                    None
                } else {
                    match self.fixed_peer_input.parse::<SocketAddr>() {
                        Ok(_) => Some(self.fixed_peer_input.clone()),
                        Err(e) => {
                            error!(
                                "Invalid fixed peer address '{}': {}",
                                self.fixed_peer_input, e
                            );
                            None
                        }
                    }
                };
                if config.fixed_peer != fixed_peer_value {
                    config.fixed_peer = fixed_peer_value;
                    self.node_restart_required = true;
                }

                let proxy_value = if self.proxy_input.is_empty() {
                    None
                } else {
                    match self.proxy_input.parse::<SocketAddr>() {
                        Ok(addr) => Some(addr),
                        Err(e) => {
                            error!("Invalid proxy address '{}': {}", self.proxy_input, e);
                            None
                        }
                    }
                };
                if config.proxy != proxy_value {
                    config.proxy = proxy_value;
                    self.node_restart_required = true;
                }

                if let Ok(_) = self.save() {
                    self.unsaved_changes = false;
                }

                Task::none()
            }

            BonsaiSettingsMessage::RestartNode => {
                self.node_restart_required = false;
                Task::none()
            }

            BonsaiSettingsMessage::ClearRestartFlag => {
                self.node_restart_required = false;
                Task::none()
            }
        }
    }
}
