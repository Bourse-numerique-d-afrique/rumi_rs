use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{Result, RumiError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RumiConfig {
    pub default_ssh: Option<SshConfig>,
    pub deployments: Vec<DeploymentConfig>,
    pub settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfig {
    pub host: String,
    pub user: String,
    pub port: Option<u16>,
    pub public_key_path: Option<PathBuf>,
    pub private_key_path: Option<PathBuf>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub name: String,
    pub domain: String,
    pub deployment_type: DeploymentType,
    pub ssh: Option<SshConfig>,
    pub dist_path: Option<PathBuf>,
    pub backup_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentType {
    Website,
    Server { port: u16, binary_path: PathBuf },
    Ethereum { 
        network_id: u32,
        http_address: String,
        ws_address: String,
        external_ip: String,
        wallet_address: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub log_level: String,
    pub backup_retention_days: u32,
    pub ssl_email: String,
    pub nginx_config_path: PathBuf,
    pub web_folder: PathBuf,
    pub ssl_cert_path: PathBuf,
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

impl Default for RumiConfig {
    fn default() -> Self {
        Self {
            default_ssh: None,
            deployments: Vec::new(),
            settings: Settings::default(),
        }
    }
}

impl RumiConfig {
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

    pub fn get_deployment(&self, name: &str) -> Option<&DeploymentConfig> {
        self.deployments.iter().find(|d| d.name == name)
    }

    pub fn add_deployment(&mut self, deployment: DeploymentConfig) {
        // Remove existing deployment with same name
        self.deployments.retain(|d| d.name != deployment.name);
        self.deployments.push(deployment);
    }

    pub fn remove_deployment(&mut self, name: &str) -> bool {
        let initial_len = self.deployments.len();
        self.deployments.retain(|d| d.name != name);
        self.deployments.len() != initial_len
    }

    pub fn get_config_path() -> PathBuf {
        if let Ok(config_dir) = std::env::var("RUMI_CONFIG_DIR") {
            PathBuf::from(config_dir).join("rumi.json")
        } else if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("rumi").join("rumi.json")
        } else {
            PathBuf::from(".rumi.json")
        }
    }

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