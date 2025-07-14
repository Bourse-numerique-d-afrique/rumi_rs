use crate::config::*;
use crate::error::*;
use crate::backup::*;
use tempfile::TempDir;
use std::path::PathBuf;

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_ssh_config_creation() {
        let ssh_config = SshConfig {
            host: "example.com".to_string(),
            user: "testuser".to_string(),
            port: Some(22),
            public_key_path: Some(PathBuf::from("/home/user/.ssh/id_rsa.pub")),
            private_key_path: Some(PathBuf::from("/home/user/.ssh/id_rsa")),
            password: None,
        };

        assert_eq!(ssh_config.host, "example.com");
        assert_eq!(ssh_config.user, "testuser");
        assert_eq!(ssh_config.port, Some(22));
    }

    #[test]
    fn test_deployment_config_website() {
        let deployment = DeploymentConfig {
            name: "test-website".to_string(),
            domain: "example.com".to_string(),
            deployment_type: DeploymentType::Website,
            ssh: None,
            dist_path: Some(PathBuf::from("/var/www/html")),
            backup_count: Some(5),
        };

        assert_eq!(deployment.name, "test-website");
        assert_eq!(deployment.domain, "example.com");
        assert!(matches!(deployment.deployment_type, DeploymentType::Website));
    }

    #[test]
    fn test_deployment_config_server() {
        let deployment = DeploymentConfig {
            name: "test-server".to_string(),
            domain: "api.example.com".to_string(),
            deployment_type: DeploymentType::Server {
                port: 8080,
                binary_path: PathBuf::from("/usr/local/bin/myapp"),
            },
            ssh: None,
            dist_path: None,
            backup_count: Some(3),
        };

        assert_eq!(deployment.name, "test-server");
        assert_eq!(deployment.domain, "api.example.com");
        
        if let DeploymentType::Server { port, binary_path } = &deployment.deployment_type {
            assert_eq!(*port, 8080);
            assert_eq!(*binary_path, PathBuf::from("/usr/local/bin/myapp"));
        } else {
            panic!("Expected Server deployment type");
        }
    }

    #[test]
    fn test_deployment_config_ethereum() {
        let deployment = DeploymentConfig {
            name: "test-ethereum".to_string(),
            domain: "eth.example.com".to_string(),
            deployment_type: DeploymentType::Ethereum {
                network_id: 1337,
                http_address: "127.0.0.1".to_string(),
                ws_address: "127.0.0.1".to_string(),
                external_ip: "203.0.113.1".to_string(),
                wallet_address: "0x1234567890abcdef".to_string(),
            },
            ssh: None,
            dist_path: None,
            backup_count: Some(10),
        };

        assert_eq!(deployment.name, "test-ethereum");
        
        if let DeploymentType::Ethereum { network_id, .. } = &deployment.deployment_type {
            assert_eq!(*network_id, 1337);
        } else {
            panic!("Expected Ethereum deployment type");
        }
    }

    #[test]
    fn test_rumi_config_default() {
        let config = RumiConfig::default();
        
        assert!(config.deployments.is_empty());
        assert!(config.default_ssh.is_none());
        assert_eq!(config.settings.log_level, "info");
        assert_eq!(config.settings.backup_retention_days, 30);
    }

    #[test]
    fn test_rumi_config_add_deployment() {
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
        
        let retrieved = config.get_deployment("test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().domain, "example.com");
    }

    #[test]
    fn test_rumi_config_remove_deployment() {
        let mut config = RumiConfig::default();
        
        let deployment = DeploymentConfig {
            name: "test".to_string(),
            domain: "example.com".to_string(),
            deployment_type: DeploymentType::Website,
            ssh: None,
            dist_path: Some(PathBuf::from("/var/www/html")),
            backup_count: Some(5),
        };

        config.add_deployment(deployment);
        assert_eq!(config.deployments.len(), 1);
        
        let removed = config.remove_deployment("test");
        assert!(removed);
        assert_eq!(config.deployments.len(), 0);
        
        let not_removed = config.remove_deployment("nonexistent");
        assert!(!not_removed);
    }

    #[test]
    fn test_config_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        
        let mut config = RumiConfig::default();
        config.settings.log_level = "debug".to_string();
        
        // Test saving
        config.save_to_file(&config_path).unwrap();
        assert!(config_path.exists());
        
        // Test loading
        let loaded_config = RumiConfig::load_from_file(&config_path).unwrap();
        assert_eq!(loaded_config.settings.log_level, "debug");
    }

    #[test]
    fn test_config_validation_empty_name() {
        let mut config = RumiConfig::default();
        
        let invalid_deployment = DeploymentConfig {
            name: "".to_string(), // Invalid: empty name
            domain: "example.com".to_string(),
            deployment_type: DeploymentType::Website,
            ssh: None,
            dist_path: Some(PathBuf::from("/var/www/html")),
            backup_count: None,
        };

        config.add_deployment(invalid_deployment);
        let result = config.validate();
        assert!(result.is_err());
        
        if let Err(RumiError::InvalidInput(msg)) = result {
            assert!(msg.contains("name cannot be empty"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_config_validation_empty_domain() {
        let mut config = RumiConfig::default();
        
        let invalid_deployment = DeploymentConfig {
            name: "test".to_string(),
            domain: "".to_string(), // Invalid: empty domain
            deployment_type: DeploymentType::Website,
            ssh: None,
            dist_path: Some(PathBuf::from("/var/www/html")),
            backup_count: None,
        };

        config.add_deployment(invalid_deployment);
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_website_no_dist_path() {
        let mut config = RumiConfig::default();
        
        let invalid_deployment = DeploymentConfig {
            name: "test".to_string(),
            domain: "example.com".to_string(),
            deployment_type: DeploymentType::Website,
            ssh: None,
            dist_path: None, // Invalid: Website needs dist_path
            backup_count: None,
        };

        config.add_deployment(invalid_deployment);
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_server_zero_port() {
        let mut config = RumiConfig::default();
        
        let invalid_deployment = DeploymentConfig {
            name: "test".to_string(),
            domain: "example.com".to_string(),
            deployment_type: DeploymentType::Server {
                port: 0, // Invalid: port cannot be 0
                binary_path: PathBuf::from("/usr/local/bin/app"),
            },
            ssh: None,
            dist_path: None,
            backup_count: None,
        };

        config.add_deployment(invalid_deployment);
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_ethereum_zero_network_id() {
        let mut config = RumiConfig::default();
        
        let invalid_deployment = DeploymentConfig {
            name: "test".to_string(),
            domain: "example.com".to_string(),
            deployment_type: DeploymentType::Ethereum {
                network_id: 0, // Invalid: network_id cannot be 0
                http_address: "127.0.0.1".to_string(),
                ws_address: "127.0.0.1".to_string(),
                external_ip: "203.0.113.1".to_string(),
                wallet_address: "0x1234567890abcdef".to_string(),
            },
            ssh: None,
            dist_path: None,
            backup_count: None,
        };

        config.add_deployment(invalid_deployment);
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_serialization() {
        let config = RumiConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: RumiConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.settings.log_level, deserialized.settings.log_level);
        assert_eq!(config.settings.backup_retention_days, deserialized.settings.backup_retention_days);
    }
}

#[cfg(test)]
mod backup_tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_backup_info_creation() {
        let backup_info = BackupInfo {
            id: "test-backup-123".to_string(),
            deployment_name: "test-deployment".to_string(),
            domain: "example.com".to_string(),
            created_at: Utc::now(),
            backup_type: BackupType::Website,
            remote_path: "/var/backups/test.tar.gz".to_string(),
            size_bytes: 1024 * 1024, // 1MB
            description: Some("Test backup for example.com".to_string()),
        };

        assert_eq!(backup_info.id, "test-backup-123");
        assert_eq!(backup_info.deployment_name, "test-deployment");
        assert_eq!(backup_info.domain, "example.com");
        assert_eq!(backup_info.size_bytes, 1024 * 1024);
        assert!(matches!(backup_info.backup_type, BackupType::Website));
    }

    #[test]
    fn test_backup_info_serialization() {
        let backup_info = BackupInfo {
            id: "test-123".to_string(),
            deployment_name: "test-deployment".to_string(),
            domain: "example.com".to_string(),
            created_at: Utc::now(),
            backup_type: BackupType::Configuration,
            remote_path: "/var/backups/config.tar.gz".to_string(),
            size_bytes: 512,
            description: None,
        };

        let json = serde_json::to_string(&backup_info).unwrap();
        let deserialized: BackupInfo = serde_json::from_str(&json).unwrap();
        
        assert_eq!(backup_info.id, deserialized.id);
        assert_eq!(backup_info.deployment_name, deserialized.deployment_name);
        assert_eq!(backup_info.domain, deserialized.domain);
        assert_eq!(backup_info.size_bytes, deserialized.size_bytes);
    }

    #[test]
    fn test_backup_manager_creation() {
        let manager = BackupManager::new("/custom/backup/path".to_string());
        assert_eq!(manager.backup_base_path, "/custom/backup/path");
        
        let default_manager = BackupManager::default();
        assert_eq!(default_manager.backup_base_path, "/var/backups/rumi");
    }

    #[test]
    fn test_backup_types() {
        let website_backup = BackupType::Website;
        let server_backup = BackupType::Server;
        let database_backup = BackupType::Database;
        let config_backup = BackupType::Configuration;

        // Test serialization
        let json = serde_json::to_string(&website_backup).unwrap();
        assert_eq!(json, "\"Website\"");
        
        let json = serde_json::to_string(&server_backup).unwrap();
        assert_eq!(json, "\"Server\"");
        
        let json = serde_json::to_string(&database_backup).unwrap();
        assert_eq!(json, "\"Database\"");
        
        let json = serde_json::to_string(&config_backup).unwrap();
        assert_eq!(json, "\"Configuration\"");
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn test_rumi_error_creation() {
        let ssh_error = RumiError::ssh_connection("Connection failed");
        assert!(matches!(ssh_error, RumiError::SshConnection(_)));
        
        let auth_error = RumiError::ssh_authentication("Auth failed");
        assert!(matches!(auth_error, RumiError::SshAuthentication(_)));
        
        let cmd_error = RumiError::command_execution("Command failed");
        assert!(matches!(cmd_error, RumiError::CommandExecution(_)));
        
        let file_error = RumiError::file_operation("File not found");
        assert!(matches!(file_error, RumiError::FileOperation(_)));
        
        let network_error = RumiError::network("Network timeout");
        assert!(matches!(network_error, RumiError::Network(_)));
        
        let config_error = RumiError::configuration("Invalid config");
        assert!(matches!(config_error, RumiError::Configuration(_)));
        
        let input_error = RumiError::invalid_input("Invalid input");
        assert!(matches!(input_error, RumiError::InvalidInput(_)));
    }

    #[test]
    fn test_error_display() {
        let error = RumiError::ssh_connection("Connection refused");
        let error_string = format!("{}", error);
        assert!(error_string.contains("SSH connection failed"));
        assert!(error_string.contains("Connection refused"));
    }

    #[test]
    fn test_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let rumi_error: RumiError = io_error.into();
        assert!(matches!(rumi_error, RumiError::Io(_)));
    }

    #[test]
    fn test_error_from_json() {
        let invalid_json = "{ invalid json }";
        let json_error = serde_json::from_str::<serde_json::Value>(invalid_json).unwrap_err();
        let rumi_error: RumiError = json_error.into();
        assert!(matches!(rumi_error, RumiError::Json(_)));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_config_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("rumi_test.json");
        
        // Create a new config
        let mut config = RumiConfig::default();
        
        // Add SSH configuration
        let ssh_config = SshConfig {
            host: "test.example.com".to_string(),
            user: "testuser".to_string(),
            port: Some(2222),
            public_key_path: Some(PathBuf::from("/home/user/.ssh/test_rsa.pub")),
            private_key_path: Some(PathBuf::from("/home/user/.ssh/test_rsa")),
            password: None,
        };
        config.default_ssh = Some(ssh_config);
        
        // Add a website deployment
        let website_deployment = DeploymentConfig {
            name: "my-website".to_string(),
            domain: "mysite.com".to_string(),
            deployment_type: DeploymentType::Website,
            ssh: None,
            dist_path: Some(PathBuf::from("/local/dist")),
            backup_count: Some(7),
        };
        config.add_deployment(website_deployment);
        
        // Add a server deployment
        let server_deployment = DeploymentConfig {
            name: "my-api".to_string(),
            domain: "api.mysite.com".to_string(),
            deployment_type: DeploymentType::Server {
                port: 8080,
                binary_path: PathBuf::from("/usr/local/bin/my-api"),
            },
            ssh: None,
            dist_path: None,
            backup_count: Some(3),
        };
        config.add_deployment(server_deployment);
        
        // Validate the configuration
        config.validate().unwrap();
        
        // Save the configuration
        config.save_to_file(&config_path).unwrap();
        assert!(config_path.exists());
        
        // Load the configuration
        let loaded_config = RumiConfig::load_from_file(&config_path).unwrap();
        
        // Verify the loaded configuration
        assert!(loaded_config.default_ssh.is_some());
        assert_eq!(loaded_config.deployments.len(), 2);
        
        let website = loaded_config.get_deployment("my-website").unwrap();
        assert_eq!(website.domain, "mysite.com");
        assert!(matches!(website.deployment_type, DeploymentType::Website));
        
        let api = loaded_config.get_deployment("my-api").unwrap();
        assert_eq!(api.domain, "api.mysite.com");
        if let DeploymentType::Server { port, .. } = &api.deployment_type {
            assert_eq!(*port, 8080);
        } else {
            panic!("Expected Server deployment type");
        }
        
        // Test validation
        loaded_config.validate().unwrap();
    }

    #[test]
    fn test_config_path_generation() {
        let path = RumiConfig::get_config_path();
        assert!(path.is_absolute() || path.file_name().is_some());
    }
}

// Performance tests
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_config_serialization_performance() {
        let mut config = RumiConfig::default();
        
        // Add many deployments
        for i in 0..1000 {
            let deployment = DeploymentConfig {
                name: format!("deployment-{}", i),
                domain: format!("site{}.example.com", i),
                deployment_type: DeploymentType::Website,
                ssh: None,
                dist_path: Some(PathBuf::from(format!("/var/www/site{}", i))),
                backup_count: Some(5),
            };
            config.add_deployment(deployment);
        }
        
        let start = Instant::now();
        let json = serde_json::to_string(&config).unwrap();
        let serialization_time = start.elapsed();
        
        let start = Instant::now();
        let _deserialized: RumiConfig = serde_json::from_str(&json).unwrap();
        let deserialization_time = start.elapsed();
        
        // Serialization should be reasonably fast
        assert!(serialization_time.as_millis() < 100);
        assert!(deserialization_time.as_millis() < 100);
        
        println!("Serialization: {:?}, Deserialization: {:?}", 
                 serialization_time, deserialization_time);
    }

    #[test]
    fn test_config_validation_performance() {
        let mut config = RumiConfig::default();
        
        // Add many deployments
        for i in 0..1000 {
            let deployment = DeploymentConfig {
                name: format!("deployment-{}", i),
                domain: format!("site{}.example.com", i),
                deployment_type: DeploymentType::Website,
                ssh: None,
                dist_path: Some(PathBuf::from(format!("/var/www/site{}", i))),
                backup_count: Some(5),
            };
            config.add_deployment(deployment);
        }
        
        let start = Instant::now();
        config.validate().unwrap();
        let validation_time = start.elapsed();
        
        // Validation should be fast
        assert!(validation_time.as_millis() < 50);
        
        println!("Validation time for 1000 deployments: {:?}", validation_time);
    }
}