use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::path::PathBuf;

use bdk_floresta::builder::NodeConfig;
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
    DisableDnsSeedsChanged(bool),

    // Actions
    SaveSettings,
    RestartNode,
    RequestDeleteNodeData,
    CancelDeleteNodeData,
    ConfirmDeleteNodeData,
    DeleteNodeDataBlocked(String),
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
    #[serde(skip)]
    pub(crate) fixed_peer_error: Option<String>,
    #[serde(skip)]
    pub(crate) proxy_error: Option<String>,
    #[serde(skip)]
    pub(crate) delete_node_data_confirm: bool,
    #[serde(skip)]
    pub(crate) delete_node_data_status: Option<String>,
    #[serde(skip)]
    pub(crate) delete_node_data_error: Option<String>,
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
    pub(crate) enable_powfps: Option<bool>,
    pub(crate) perform_backfill: Option<bool>,
    pub(crate) user_agent: Option<String>,
    pub(crate) allow_p2pv1_fallback: Option<bool>,
    pub(crate) fixed_peer: Option<SocketAddr>,
    pub(crate) max_banscore: Option<u32>,
    pub(crate) disable_dns_seeds: Option<bool>,
    pub(crate) socks5_proxy: Option<SocketAddr>,
}

impl NodeNetworkSpecific {
    /// Convert to UtreexoNodeConfig, using defaults where options are None
    pub(crate) fn to_config(&self, network: Network, data_dir: PathBuf) -> NodeConfig {
        let mut config = NodeConfig {
            network,
            data_directory: data_dir,
            ..Default::default()
        };

        if let Some(use_assume_utreexo) = self.use_assume_utreexo {
            config.assume_utreexo = use_assume_utreexo;
        }
        if let Some(enable_powfps) = self.enable_powfps {
            config.enable_powfps = enable_powfps;
        }
        if let Some(perform_backfill) = self.perform_backfill {
            config.perform_backfill = perform_backfill;
        }
        if let Some(user_agent) = &self.user_agent {
            config.user_agent = user_agent.clone();
        }
        if let Some(allow_p2pv1_fallback) = self.allow_p2pv1_fallback {
            config.allow_p2pv1_fallback = allow_p2pv1_fallback;
        }
        if let Some(fixed_peer) = self.fixed_peer {
            config.fixed_peer = Some(fixed_peer);
        }
        if let Some(max_banscore) = self.max_banscore {
            config.max_banscore = max_banscore;
        }
        if let Some(disable_dns_seeds) = self.disable_dns_seeds {
            config.disable_dns_seeds = disable_dns_seeds;
        }
        if let Some(socks5_proxy) = self.socks5_proxy {
            config.socks5_proxy = Some(socks5_proxy);
        }

        config
    }

    /// Create from UtreexoNodeConfig
    pub(crate) fn from_config(config: &NodeConfig) -> Self {
        NodeNetworkSpecific {
            use_assume_utreexo: Some(config.assume_utreexo),
            enable_powfps: Some(config.enable_powfps),
            perform_backfill: Some(config.perform_backfill),
            user_agent: Some(config.user_agent.clone()),
            allow_p2pv1_fallback: Some(config.allow_p2pv1_fallback),
            fixed_peer: config.fixed_peer,
            max_banscore: Some(config.max_banscore),
            disable_dns_seeds: Some(config.disable_dns_seeds),
            socks5_proxy: config.socks5_proxy,
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
    pub(crate) fn active_network(&self) -> Network {
        self.bonsai.network.unwrap_or(Network::Signet)
    }

    pub(crate) fn base_dir() -> PathBuf {
        dirs::home_dir()
            .or_else(dirs::data_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".bonsai")
    }

    fn current_network_config_mut(&mut self) -> &mut NodeNetworkSpecific {
        let network = self.active_network();
        self.node.get_network_config_mut(network)
    }

    fn mark_node_config_changed(&mut self) {
        self.node_restart_required = true;
        self.unsaved_changes = true;
    }

    fn update_node_config<T: PartialEq>(
        &mut self,
        update: impl FnOnce(&mut NodeNetworkSpecific) -> (&mut Option<T>, T),
    ) {
        let config = self.current_network_config_mut();
        let (field, value) = update(config);
        if field.as_ref() != Some(&value) {
            *field = Some(value);
            self.mark_node_config_changed();
        }
    }

    fn update_unsaved_input_state(&mut self) {
        let config = self.node.get_network_config(self.active_network());
        let user_agent_changed =
            config.user_agent.as_deref().unwrap_or_default() != self.user_agent_input.as_str();
        let fixed_peer_changed = config
            .fixed_peer
            .map(|addr| addr.to_string())
            .unwrap_or_default()
            != self.fixed_peer_input;
        let proxy_changed = config
            .socks5_proxy
            .map(|addr| addr.to_string())
            .unwrap_or_default()
            != self.proxy_input;

        if user_agent_changed || fixed_peer_changed || proxy_changed {
            self.unsaved_changes = true;
        }
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
                let network = settings.active_network();
                let config = settings.node.get_network_config(network);
                settings.user_agent_input = config.user_agent.clone().unwrap_or_default();
                settings.fixed_peer_input = config
                    .fixed_peer
                    .map(|addr| addr.to_string())
                    .unwrap_or_default();
                settings.proxy_input = config
                    .socks5_proxy
                    .map(|p| p.to_string())
                    .unwrap_or_default();

                settings
            }
            Err(_) => Self::default(),
        }
    }

    /// Save settings to disk
    pub(crate) fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data_directory = Self::base_dir();
        let settings_path = data_directory.join(SETTINGS_FILE);

        fs::create_dir_all(&data_directory).map_err(|e| {
            error!(
                "Failed to create data directory at {}: {}",
                data_directory.to_string_lossy(),
                e
            );
            e
        })?;

        let settings_toml = toml::to_string_pretty(self)?;
        fs::write(&settings_path, settings_toml).map_err(|e| {
            error!(
                "Failed to write settings file to {}: {}",
                settings_path.to_string_lossy(),
                e
            );
            e
        })?;

        Ok(())
    }

    /// Get the [`UtreexoNodeConfig`] for starting the node.
    pub(crate) fn get_node_config(&self, network: Network, data_dir: &Path) -> NodeConfig {
        let network = self.bonsai.network.unwrap_or(network);
        let data_dir = data_dir.join(network.to_string());

        let network_config = self.node.get_network_config(network);
        network_config.to_config(network, data_dir)
    }

    pub(crate) fn active_network_data_dir(&self) -> PathBuf {
        Self::base_dir().join(self.active_network().to_string())
    }

    /// Update settings from a UtreexoNodeConfig (called after first run)
    pub(crate) fn update_from_config(&mut self, config: &NodeConfig) {
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
                self.update_node_config(|config| (&mut config.use_assume_utreexo, enabled));
                Task::none()
            }

            BonsaiSettingsMessage::PowFraudProofsChanged(enabled) => {
                self.update_node_config(|config| (&mut config.enable_powfps, enabled));
                Task::none()
            }

            BonsaiSettingsMessage::BackfillChanged(enabled) => {
                self.update_node_config(|config| (&mut config.perform_backfill, enabled));
                Task::none()
            }

            BonsaiSettingsMessage::UserAgentInputChanged(value) => {
                self.user_agent_input = value;
                self.update_unsaved_input_state();
                Task::none()
            }

            BonsaiSettingsMessage::AllowV1FallbackChanged(enabled) => {
                self.update_node_config(|config| (&mut config.allow_p2pv1_fallback, enabled));
                Task::none()
            }

            BonsaiSettingsMessage::FixedPeerInputChanged(value) => {
                self.fixed_peer_input = value;
                self.fixed_peer_error = None;
                self.update_unsaved_input_state();
                Task::none()
            }

            BonsaiSettingsMessage::ProxyInputChanged(value) => {
                self.proxy_input = value;
                self.proxy_error = None;
                self.update_unsaved_input_state();
                Task::none()
            }

            BonsaiSettingsMessage::MaxBanscoreChanged(value) => {
                if let Ok(banscore) = value.parse::<u32>() {
                    self.update_node_config(|config| (&mut config.max_banscore, banscore));
                }
                Task::none()
            }

            BonsaiSettingsMessage::DisableDnsSeedsChanged(enabled) => {
                self.update_node_config(|config| (&mut config.disable_dns_seeds, enabled));
                Task::none()
            }

            BonsaiSettingsMessage::SaveSettings => {
                let mut changed = false;
                let mut restart_required = false;
                let user_agent_input = self.user_agent_input.clone();
                let fixed_peer_input = self.fixed_peer_input.clone();
                let proxy_input = self.proxy_input.clone();

                let fixed_peer_parsed = if fixed_peer_input.is_empty() {
                    None
                } else {
                    match fixed_peer_input.parse::<SocketAddr>() {
                        Ok(addr) => Some(addr),
                        Err(e) => {
                            self.fixed_peer_error = Some(format!("Invalid socket address: {e}"));
                            error!("Invalid fixed peer address '{}': {}", fixed_peer_input, e);
                            return Task::none();
                        }
                    }
                };

                let proxy_value = if proxy_input.is_empty() {
                    None
                } else {
                    match proxy_input.parse::<SocketAddr>() {
                        Ok(addr) => Some(addr),
                        Err(e) => {
                            self.proxy_error = Some(format!("Invalid socket address: {e}"));
                            error!("Invalid proxy address '{}': {}", proxy_input, e);
                            return Task::none();
                        }
                    }
                };

                let config = self.current_network_config_mut();

                if !user_agent_input.is_empty()
                    && Some(&user_agent_input) != config.user_agent.as_ref()
                {
                    config.user_agent = Some(user_agent_input);
                    changed = true;
                    restart_required = true;
                }

                if config.fixed_peer != fixed_peer_parsed {
                    config.fixed_peer = fixed_peer_parsed;
                    changed = true;
                    restart_required = true;
                }

                if config.socks5_proxy != proxy_value {
                    config.socks5_proxy = proxy_value;
                    changed = true;
                    restart_required = true;
                }

                if changed {
                    self.unsaved_changes = true;
                }
                if restart_required {
                    self.node_restart_required = true;
                }

                if self.save().is_ok() {
                    self.unsaved_changes = false;
                    self.fixed_peer_error = None;
                    self.proxy_error = None;
                }

                Task::none()
            }

            BonsaiSettingsMessage::RestartNode => {
                self.node_restart_required = false;
                Task::none()
            }

            BonsaiSettingsMessage::RequestDeleteNodeData => {
                self.delete_node_data_confirm = true;
                self.delete_node_data_status = None;
                self.delete_node_data_error = None;
                Task::none()
            }

            BonsaiSettingsMessage::CancelDeleteNodeData => {
                self.delete_node_data_confirm = false;
                Task::none()
            }

            BonsaiSettingsMessage::ConfirmDeleteNodeData => {
                let data_dir = self.active_network_data_dir();
                self.delete_node_data_confirm = false;
                self.delete_node_data_status = None;
                self.delete_node_data_error = None;

                if !data_dir.exists() {
                    self.delete_node_data_status = Some(format!(
                        "No node data found for {}",
                        self.active_network().to_string().to_uppercase()
                    ));
                    return Task::none();
                }

                match fs::remove_dir_all(&data_dir) {
                    Ok(_) => {
                        self.delete_node_data_status = Some(format!(
                            "Deleted node data for {}",
                            self.active_network().to_string().to_uppercase()
                        ));
                    }
                    Err(e) => {
                        self.delete_node_data_error = Some(format!(
                            "Failed to delete {}: {e}",
                            data_dir.to_string_lossy()
                        ));
                        error!(
                            "Failed to delete node data at {}: {}",
                            data_dir.to_string_lossy(),
                            e
                        );
                    }
                }

                Task::none()
            }

            BonsaiSettingsMessage::DeleteNodeDataBlocked(reason) => {
                self.delete_node_data_confirm = false;
                self.delete_node_data_status = None;
                self.delete_node_data_error = Some(reason);
                Task::none()
            }

            BonsaiSettingsMessage::ClearRestartFlag => {
                self.node_restart_required = false;
                Task::none()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::path::PathBuf;

    use bitcoin::Network;

    use super::BonsaiSettings;
    use super::BonsaiSettingsMessage;

    #[test]
    fn invalid_fixed_peer_does_not_clear_existing_config() {
        let existing_peer: SocketAddr = "127.0.0.1:8333".parse().unwrap();
        let mut settings = BonsaiSettings::default();
        settings.bonsai.network = Some(Network::Signet);
        settings
            .node
            .get_network_config_mut(Network::Signet)
            .fixed_peer = Some(existing_peer);
        settings.fixed_peer_input = "not-a-socket".to_string();

        let _ = settings.update(BonsaiSettingsMessage::SaveSettings);

        assert_eq!(
            settings.node.get_network_config(Network::Signet).fixed_peer,
            Some(existing_peer)
        );
        assert!(settings.fixed_peer_error.is_some());
    }

    #[test]
    fn invalid_proxy_does_not_clear_existing_config() {
        let existing_proxy: SocketAddr = "127.0.0.1:9050".parse().unwrap();
        let mut settings = BonsaiSettings::default();
        settings.bonsai.network = Some(Network::Signet);
        settings
            .node
            .get_network_config_mut(Network::Signet)
            .socks5_proxy = Some(existing_proxy);
        settings.proxy_input = "not-a-socket".to_string();

        let _ = settings.update(BonsaiSettingsMessage::SaveSettings);

        assert_eq!(
            settings
                .node
                .get_network_config(Network::Signet)
                .socks5_proxy,
            Some(existing_proxy)
        );
        assert!(settings.proxy_error.is_some());
    }

    #[test]
    fn text_input_changes_mark_settings_unsaved() {
        let mut settings = BonsaiSettings::default();

        let _ = settings.update(BonsaiSettingsMessage::FixedPeerInputChanged(
            "127.0.0.1:8333".to_string(),
        ));

        assert!(settings.unsaved_changes);
    }

    #[test]
    fn node_config_includes_network_specific_settings() {
        let fixed_peer: SocketAddr = "127.0.0.1:38333".parse().unwrap();
        let proxy: SocketAddr = "127.0.0.1:9050".parse().unwrap();
        let mut settings = BonsaiSettings::default();
        settings.bonsai.network = Some(Network::Signet);

        let network_config = settings.node.get_network_config_mut(Network::Signet);
        network_config.use_assume_utreexo = Some(false);
        network_config.enable_powfps = Some(false);
        network_config.perform_backfill = Some(false);
        network_config.user_agent = Some("bonsai-test".to_string());
        network_config.allow_p2pv1_fallback = Some(false);
        network_config.fixed_peer = Some(fixed_peer);
        network_config.max_banscore = Some(42);
        network_config.disable_dns_seeds = Some(true);
        network_config.socks5_proxy = Some(proxy);

        let node_config =
            settings.get_node_config(Network::Signet, &PathBuf::from("/tmp/bonsai-test"));

        assert!(!node_config.assume_utreexo);
        assert!(!node_config.enable_powfps);
        assert!(!node_config.perform_backfill);
        assert_eq!(node_config.user_agent, "bonsai-test");
        assert!(!node_config.allow_p2pv1_fallback);
        assert_eq!(node_config.fixed_peer, Some(fixed_peer));
        assert_eq!(node_config.max_banscore, 42);
        assert!(node_config.disable_dns_seeds);
        assert_eq!(node_config.socks5_proxy, Some(proxy));
    }

    #[test]
    fn delete_node_data_request_requires_confirmation() {
        let mut settings = BonsaiSettings::default();

        let _ = settings.update(BonsaiSettingsMessage::RequestDeleteNodeData);
        assert!(settings.delete_node_data_confirm);

        let _ = settings.update(BonsaiSettingsMessage::CancelDeleteNodeData);
        assert!(!settings.delete_node_data_confirm);
    }

    #[test]
    fn blocked_node_data_delete_records_error() {
        let mut settings = BonsaiSettings::default();

        let _ = settings.update(BonsaiSettingsMessage::DeleteNodeDataBlocked(
            "stop node first".to_string(),
        ));

        assert!(!settings.delete_node_data_confirm);
        assert_eq!(
            settings.delete_node_data_error.as_deref(),
            Some("stop node first")
        );
    }
}
