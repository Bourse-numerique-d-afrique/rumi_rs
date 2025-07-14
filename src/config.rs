//! # Configuration Management
//!
//! This module provides configuration management for Rumi2, including:
//!
//! - **Configuration Loading**: JSON-based configuration files with validation
//! - **SSH Management**: Multiple SSH connection profiles
//! - **Deployment Profiles**: Website, server, and Ethereum deployment configurations
//! - **Settings Management**: Global application settings and preferences
//!
//! ## Configuration Structure
//!
//! The configuration system supports:
//! - Default SSH connection settings for convenience
//! - Multiple named deployment configurations
//! - Environment-aware configuration file locations
//! - Comprehensive validation to prevent deployment errors
//!
//! ## Examples
//!
//! ```rust
//! use rumi2::config::{RumiConfig, SshConfig, DeploymentConfig, DeploymentType};
//! use std::path::PathBuf;
//!
//! # fn main() -> rumi2::error::Result<()> {
//! // Load configuration from default location
//! let config_path = RumiConfig::get_config_path();
//! let mut config = RumiConfig::load_from_file(&config_path)?;
//!
//! // Add an SSH configuration
//! let ssh_config = SshConfig {
//!     host: "server.example.com".to_string(),
//!     user: "deploy".to_string(),
//!     port: Some(22),
//!     public_key_path: Some(PathBuf::from("~/.ssh/id_rsa.pub")),
//!     private_key_path: Some(PathBuf::from("~/.ssh/id_rsa")),
//!     password: None,
//! };
//! config.default_ssh = Some(ssh_config);
//!
//! // Add a website deployment
//! let deployment = DeploymentConfig {
//!     name: "my-website".to_string(),
//!     domain: "example.com".to_string(),
//!     deployment_type: DeploymentType::Website,
//!     ssh: None, // Uses default SSH config
//!     dist_path: Some(PathBuf::from("./dist")),
//!     backup_count: Some(5),
//! };
//! config.add_deployment(deployment);
//!
//! // Validate and save
//! config.validate()?;
//! config.save_to_file(&config_path)?;
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{Result, RumiError};

/// Main configuration structure for Rumi2
///
/// This is the root configuration object that contains all settings,
/// SSH configurations, and deployment profiles for the application.
///
/// # Fields
///
/// - `default_ssh`: Optional default SSH configuration used when deployments
///   don't specify their own SSH settings
/// - `deployments`: List of configured deployment profiles
/// - `settings`: Global application settings and preferences
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RumiConfig {
    pub default_ssh: Option<SshConfig>,
    pub deployments: Vec<DeploymentConfig>,
    pub settings: Settings,
}

/// SSH connection configuration
///
/// Contains all necessary information to establish an SSH connection
/// to a remote server. Supports both key-based and password authentication.
///
/// # Examples
///
/// ```rust
/// use rumi2::config::SshConfig;
/// use std::path::PathBuf;
///
/// // Key-based authentication
/// let ssh_config = SshConfig {
///     host: "server.example.com".to_string(),
///     user: "deploy".to_string(),
///     port: Some(22),
///     public_key_path: Some(PathBuf::from("~/.ssh/id_rsa.pub")),
///     private_key_path: Some(PathBuf::from("~/.ssh/id_rsa")),
///     password: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfig {
    /// The hostname or IP address of the SSH server
    pub host: String,
    /// Username for SSH authentication
    pub user: String,
    /// SSH port (defaults to 22 if None)
    pub port: Option<u16>,
    /// Path to the public key file for key-based authentication
    pub public_key_path: Option<PathBuf>,
    /// Path to the private key file for key-based authentication
    pub private_key_path: Option<PathBuf>,
    /// Password for password-based authentication (not recommended for production)
    pub password: Option<String>,
}

/// Configuration for a specific deployment
///
/// Represents a complete deployment configuration including the deployment target,
/// connection details, and deployment-specific settings.
///
/// # Examples
///
/// ```rust
/// use rumi2::config::{DeploymentConfig, DeploymentType};
/// use std::path::PathBuf;
///
/// let website_deployment = DeploymentConfig {
///     name: "my-website".to_string(),
///     domain: "example.com".to_string(),
///     deployment_type: DeploymentType::Website,
///     ssh: None, // Uses default SSH config
///     dist_path: Some(PathBuf::from("./dist")),
///     backup_count: Some(5),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Unique name for this deployment configuration
    pub name: String,
    /// Domain name for the deployment (used for SSL certificates and nginx config)
    pub domain: String,
    /// Type of deployment (website, server, or Ethereum node)
    pub deployment_type: DeploymentType,
    /// SSH configuration specific to this deployment (overrides default if present)
    pub ssh: Option<SshConfig>,
    /// Local path to the distribution files for website deployments
    pub dist_path: Option<PathBuf>,
    /// Number of backups to retain for this deployment
    pub backup_count: Option<u32>,
}

/// Type of deployment being configured
///
/// Defines the specific type of application or service being deployed,
/// along with type-specific configuration parameters.
///
/// # Variants
///
/// - `Website`: Static website deployment with nginx hosting
/// - `Server`: Binary server deployment with reverse proxy
/// - `Ethereum`: Full Ethereum node deployment with geth
///
/// # Examples
///
/// ```rust
/// use rumi2::config::DeploymentType;
/// use std::path::PathBuf;
///
/// // Website deployment
/// let website = DeploymentType::Website;
///
/// // Server deployment
/// let server = DeploymentType::Server {
///     port: 3000,
///     binary_path: PathBuf::from("/usr/local/bin/myapp"),
/// };
///
/// // Ethereum node deployment
/// let ethereum = DeploymentType::Ethereum {
///     network_id: 1,
///     http_address: "127.0.0.1".to_string(),
///     ws_address: "127.0.0.1".to_string(),
///     external_ip: "203.0.113.1".to_string(),
///     wallet_address: "0x742d35cc6ba45c8b6e23f9e2c0e9b7f7a2e8b2f1".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentType {
    /// Static website deployment served by nginx
    Website,
    /// Binary server deployment with reverse proxy configuration
    Server { 
        /// Port the server binary listens on
        port: u16, 
        /// Path to the server binary on the local machine
        binary_path: PathBuf 
    },
    /// Ethereum node deployment with full geth setup
    Ethereum { 
        /// Ethereum network ID for the blockchain
        network_id: u32,
        /// HTTP RPC server bind address
        http_address: String,
        /// WebSocket server bind address
        ws_address: String,
        /// External IP address for P2P networking
        external_ip: String,
        /// Wallet address for mining rewards
        wallet_address: String,
    },
}

/// Global application settings and preferences
///
/// Contains system-wide configuration that applies to all deployments
/// and operations performed by Rumi2.
///
/// # Examples
///
/// ```rust
/// use rumi2::config::Settings;
/// use std::path::PathBuf;
///
/// let settings = Settings {
///     log_level: "debug".to_string(),
///     backup_retention_days: 14,
///     ssl_email: "admin@mycompany.com".to_string(),
///     nginx_config_path: PathBuf::from("/etc/nginx/sites-available"),
///     web_folder: PathBuf::from("/var/www"),
///     ssl_cert_path: PathBuf::from("/etc/letsencrypt/live"),
///     dry_run: false,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Logging level (debug, info, warn, error)
    pub log_level: String,
    /// Number of days to retain backups before automatic cleanup
    pub backup_retention_days: u32,
    /// Email address for Let's Encrypt SSL certificate registration
    pub ssl_email: String,
    /// Path where nginx site configurations are stored
    pub nginx_config_path: PathBuf,
    /// Root directory for website files
    pub web_folder: PathBuf,
    /// Path where SSL certificates are stored
    pub ssl_cert_path: PathBuf,
    /// Enable dry-run mode (preview changes without executing)
    pub dry_run: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            backup_retention_days: 30,
            ssl_email: "admin@example.com".to_string(),
            nginx_config_path: PathBuf::from("/etc/nginx/sites-available"),
            web_folder: PathBuf::from("/var/www"),
            ssl_cert_path: PathBuf::from("/etc/letsencrypt/live"),
            dry_run: false,
        }
    }
}


impl RumiConfig {
    /// Load configuration from a JSON file
    ///
    /// If the file doesn't exist, creates a new configuration file with default settings.
    /// Returns an error if the file exists but cannot be parsed.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rumi2::config::RumiConfig;
    /// use std::path::PathBuf;
    ///
    /// let config_path = PathBuf::from("/home/user/.config/rumi/rumi.json");
    /// let config = RumiConfig::load_from_file(&config_path)?;
    /// # Ok::<(), rumi2::error::RumiError>(())
    /// ```
    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            log::info!("Config file not found at {:?}, creating default", path);
            let config = Self::default();
            config.save_to_file(path)?;
            return Ok(config);
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| RumiError::configuration(format!("Failed to read config file: {}", e)))?;
        
        let config: RumiConfig = serde_json::from_str(&content)
            .map_err(|e| RumiError::configuration(format!("Failed to parse config file: {}", e)))?;
        
        Ok(config)
    }

    /// Save configuration to a JSON file
    ///
    /// Creates parent directories if they don't exist. The configuration
    /// is saved in pretty-printed JSON format for human readability.
    ///
    /// # Arguments
    ///
    /// * `path` - Path where the configuration file should be saved
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rumi2::config::RumiConfig;
    /// use std::path::PathBuf;
    ///
    /// let config = RumiConfig::default();
    /// let config_path = PathBuf::from("/home/user/.config/rumi/rumi.json");
    /// config.save_to_file(&config_path)?;
    /// # Ok::<(), rumi2::error::RumiError>(())
    /// ```
    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| RumiError::configuration(format!("Failed to create config directory: {}", e)))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| RumiError::configuration(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| RumiError::configuration(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// Get a deployment configuration by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the deployment to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(&DeploymentConfig)` if found, `None` otherwise.
    pub fn get_deployment(&self, name: &str) -> Option<&DeploymentConfig> {
        self.deployments.iter().find(|d| d.name == name)
    }

    /// Add or update a deployment configuration
    ///
    /// If a deployment with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `deployment` - The deployment configuration to add
    pub fn add_deployment(&mut self, deployment: DeploymentConfig) {
        // Remove existing deployment with same name
        self.deployments.retain(|d| d.name != deployment.name);
        self.deployments.push(deployment);
    }

    /// Remove a deployment configuration by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the deployment to remove
    ///
    /// # Returns
    ///
    /// Returns `true` if a deployment was removed, `false` if no deployment with that name existed.
    pub fn remove_deployment(&mut self, name: &str) -> bool {
        let initial_len = self.deployments.len();
        self.deployments.retain(|d| d.name != name);
        self.deployments.len() != initial_len
    }

    /// Get the default configuration file path
    ///
    /// The path is determined in the following order:
    /// 1. `RUMI_CONFIG_DIR` environment variable + "rumi.json"
    /// 2. User's config directory + "rumi/rumi.json"
    /// 3. Current directory + ".rumi.json"
    ///
    /// # Returns
    ///
    /// Returns the PathBuf to the configuration file location.
    pub fn get_config_path() -> PathBuf {
        if let Ok(config_dir) = std::env::var("RUMI_CONFIG_DIR") {
            PathBuf::from(config_dir).join("rumi.json")
        } else if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("rumi").join("rumi.json")
        } else {
            PathBuf::from(".rumi.json")
        }
    }

    /// Validate all deployment configurations
    ///
    /// Checks that all deployment configurations have valid settings
    /// appropriate for their deployment type.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all deployments are valid, or an error describing
    /// the first validation failure encountered.
    pub fn validate(&self) -> Result<()> {
        for deployment in &self.deployments {
            self.validate_deployment(deployment)?;
        }
        Ok(())
    }

    fn validate_deployment(&self, deployment: &DeploymentConfig) -> Result<()> {
        if deployment.name.is_empty() {
            return Err(RumiError::invalid_input("Deployment name cannot be empty"));
        }

        if deployment.domain.is_empty() {
            return Err(RumiError::invalid_input("Domain cannot be empty"));
        }

        match &deployment.deployment_type {
            DeploymentType::Server { port, binary_path } => {
                if *port == 0 {
                    return Err(RumiError::invalid_input("Server port cannot be 0"));
                }
                if !binary_path.is_absolute() {
                    return Err(RumiError::invalid_input("Binary path must be absolute"));
                }
            }
            DeploymentType::Ethereum { network_id, wallet_address, .. } => {
                if *network_id == 0 {
                    return Err(RumiError::invalid_input("Network ID cannot be 0"));
                }
                if wallet_address.is_empty() {
                    return Err(RumiError::invalid_input("Wallet address cannot be empty"));
                }
            }
            DeploymentType::Website => {
                // Website deployments need dist_path
                if deployment.dist_path.is_none() {
                    return Err(RumiError::invalid_input("Website deployments require dist_path"));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = RumiConfig::default();
        assert!(config.deployments.is_empty());
        assert!(config.default_ssh.is_none());
        assert_eq!(config.settings.log_level, "info");
    }

    #[test]
    fn test_config_serialization() {
        let config = RumiConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: RumiConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.settings.log_level, deserialized.settings.log_level);
    }

    #[test]
    fn test_config_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        
        let mut config = RumiConfig::default();
        config.settings.log_level = "debug".to_string();
        
        // Save config
        config.save_to_file(&config_path).unwrap();
        assert!(config_path.exists());
        
        // Load config
        let loaded_config = RumiConfig::load_from_file(&config_path).unwrap();
        assert_eq!(loaded_config.settings.log_level, "debug");
    }

    #[test]
    fn test_deployment_management() {
        let mut config = RumiConfig::default();
        
        let deployment = DeploymentConfig {
            name: "test".to_string(),
            domain: "example.com".to_string(),
            deployment_type: DeploymentType::Website,
            ssh: None,
            dist_path: Some(PathBuf::from("/var/www/html")),
            backup_count: Some(5),
        };
        
        config.add_deployment(deployment.clone());
        assert_eq!(config.deployments.len(), 1);
        
        let found = config.get_deployment("test");
        assert!(found.is_some());
        assert_eq!(found.unwrap().domain, "example.com");
        
        let removed = config.remove_deployment("test");
        assert!(removed);
        assert_eq!(config.deployments.len(), 0);
    }

    #[test]
    fn test_config_validation() {
        let config = RumiConfig::default();
        assert!(config.validate().is_ok());
        
        let mut config_with_invalid = RumiConfig::default();
        let invalid_deployment = DeploymentConfig {
            name: "".to_string(), // Invalid: empty name
            domain: "example.com".to_string(),
            deployment_type: DeploymentType::Website,
            ssh: None,
            dist_path: None,
            backup_count: None,
        };
        
        config_with_invalid.add_deployment(invalid_deployment);
        assert!(config_with_invalid.validate().is_err());
    }
}