use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

use rumi2::{
    backup::BackupManager,
    config::{DeploymentType, RumiConfig, SshConfig},
    error::{Result, RumiError},
    session::RumiSession,
};

#[derive(Parser)]
#[command(name = "rumi2")]
#[command(about = "Rumi2 cli to help publish new website to a server via ssh")]
#[command(version = "1.0")]
#[command(author = "Bourse Numerique D'Afrique <dev@boursenumeriquedafrique.com>")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, help = "Configuration file path")]
    config: Option<PathBuf>,

    #[arg(long, help = "Enable verbose logging")]
    verbose: bool,

    #[arg(long, help = "Dry run mode - don't execute commands")]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Manage website hosting")]
    Hosting {
        #[command(subcommand)]
        action: HostingCommands,
    },
    #[command(about = "Manage server deployments")]
    Server {
        #[command(subcommand)]
        action: ServerCommands,
    },
    #[command(about = "Manage Ethereum nodes")]
    Ethereum {
        #[command(subcommand)]
        action: EthereumCommands,
    },
    #[command(about = "Manage backups")]
    Backup {
        #[command(subcommand)]
        action: BackupCommands,
    },
    #[command(about = "Manage configurations")]
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum HostingCommands {
    #[command(about = "Install a new website")]
    Install {
        #[arg(long, help = "Deployment name")]
        name: String,
        #[arg(long, help = "Domain name")]
        domain: String,
        #[arg(long, help = "Local distribution path")]
        dist_path: PathBuf,
        #[arg(long, help = "SSH connection details")]
        ssh_config: Option<String>,
    },
    #[command(about = "Update an existing website")]
    Update {
        #[arg(long, help = "Deployment name")]
        name: String,
        #[arg(long, help = "Local distribution path")]
        dist_path: Option<PathBuf>,
    },
    #[command(about = "Rollback to a previous version")]
    Rollback {
        #[arg(long, help = "Deployment name")]
        name: String,
        #[arg(long, help = "Backup ID to restore")]
        backup_id: String,
    },
    #[command(about = "List deployments")]
    List,
}

#[derive(Subcommand)]
enum ServerCommands {
    #[command(about = "Deploy a server application")]
    Deploy {
        #[arg(long, help = "Deployment name")]
        name: String,
        #[arg(long, help = "Domain name")]
        domain: String,
        #[arg(long, help = "Server binary path")]
        binary_path: PathBuf,
        #[arg(long, help = "Server port")]
        port: u16,
    },
    #[command(about = "Start a server")]
    Start {
        #[arg(long, help = "Deployment name")]
        name: String,
    },
    #[command(about = "Stop a server")]
    Stop {
        #[arg(long, help = "Deployment name")]
        name: String,
    },
    #[command(about = "Restart a server")]
    Restart {
        #[arg(long, help = "Deployment name")]
        name: String,
    },
    #[command(about = "Check server status")]
    Status {
        #[arg(long, help = "Deployment name")]
        name: String,
    },
}

#[derive(Subcommand)]
enum EthereumCommands {
    #[command(about = "Install Ethereum node")]
    Install {
        #[arg(long, help = "Deployment name")]
        name: String,
        #[arg(long, help = "Domain name")]
        domain: String,
        #[arg(long, help = "Network ID")]
        network_id: u32,
        #[arg(long, help = "HTTP address")]
        http_address: String,
        #[arg(long, help = "WebSocket address")]
        ws_address: String,
        #[arg(long, help = "External IP")]
        external_ip: String,
        #[arg(long, help = "Wallet address")]
        wallet_address: String,
    },
}

#[derive(Subcommand)]
enum BackupCommands {
    #[command(about = "Create a backup")]
    Create {
        #[arg(long, help = "Deployment name")]
        name: String,
        #[arg(long, help = "Backup type")]
        backup_type: Option<String>,
    },
    #[command(about = "List backups")]
    List {
        #[arg(long, help = "Filter by deployment name")]
        name: Option<String>,
    },
    #[command(about = "Restore from backup")]
    Restore {
        #[arg(long, help = "Backup ID")]
        backup_id: String,
        #[arg(long, help = "Restore path")]
        restore_path: Option<String>,
    },
    #[command(about = "Delete a backup")]
    Delete {
        #[arg(long, help = "Backup ID")]
        backup_id: String,
    },
    #[command(about = "Clean up old backups")]
    Cleanup {
        #[arg(long, help = "Retention days", default_value = "30")]
        retention_days: u32,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    #[command(about = "Initialize configuration")]
    Init,
    #[command(about = "Add SSH configuration")]
    AddSsh {
        #[arg(long, help = "Configuration name")]
        name: String,
        #[arg(long, help = "SSH host")]
        host: String,
        #[arg(long, help = "SSH user")]
        user: String,
        #[arg(long, help = "SSH port", default_value = "22")]
        port: u16,
        #[arg(long, help = "Public key path")]
        public_key: Option<PathBuf>,
        #[arg(long, help = "Private key path")]
        private_key: Option<PathBuf>,
    },
    #[command(about = "Show current configuration")]
    Show,
    #[command(about = "Validate configuration")]
    Validate,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .init();

    // Load configuration
    let config_path = cli.config.unwrap_or_else(RumiConfig::get_config_path);
    let mut config = RumiConfig::load_from_file(&config_path)?;

    // Set dry run mode
    config.settings.dry_run = cli.dry_run;

    if cli.dry_run {
        info!("Running in dry-run mode - no actual changes will be made");
    }

    match cli.command {
        Commands::Hosting { action } => handle_hosting_commands(action, &config).await,
        Commands::Server { action } => handle_server_commands(action, &config).await,
        Commands::Ethereum { action } => handle_ethereum_commands(action, &config).await,
        Commands::Backup { action } => handle_backup_commands(action, &config).await,
        Commands::Config { action } => handle_config_commands(action, &mut config, &config_path).await,
    }
}

async fn handle_hosting_commands(action: HostingCommands, config: &RumiConfig) -> Result<()> {
    match action {
        HostingCommands::Install { name, domain, dist_path, ssh_config: _ } => {
            info!("Installing website: {} at domain: {}", name, domain);
            
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;
            
            if config.settings.dry_run {
                info!("DRY RUN: Would install website {} from {:?}", name, dist_path);
                return Ok(());
            }

            // Create backup before installation
            let backup_manager = BackupManager::default();
            let website_path = format!("/var/www/{}*", domain.replace('.', "_"));
            
            if session.directory_exists(&website_path)? {
                info!("Creating backup before installation");
                backup_manager.create_website_backup(&session, &name, &domain, &website_path)?;
            }

            // Install website using the existing function but with improved error handling
            rumi2::commands::websites::install_command(&session, &domain, dist_path.to_str().unwrap())?;
            
            info!("Website installation completed successfully");
        }
        HostingCommands::Update { name, dist_path } => {
            info!("Updating website: {}", name);
            
            let deployment = config.get_deployment(&name)
                .ok_or_else(|| RumiError::configuration(format!("Deployment '{}' not found", name)))?;
            
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would update website {}", name);
                return Ok(());
            }

            // Create backup before update
            let backup_manager = BackupManager::default();
            let website_path = format!("/var/www/{}*", deployment.domain.replace('.', "_"));
            backup_manager.create_website_backup(&session, &name, &deployment.domain, &website_path)?;

            let update_path = dist_path.as_ref()
                .or(deployment.dist_path.as_ref())
                .ok_or_else(|| RumiError::invalid_input("No distribution path provided"))?;

            rumi2::commands::websites::update_command(&session, &deployment.domain, update_path.to_str().unwrap())?;
            
            info!("Website update completed successfully");
        }
        HostingCommands::Rollback { name, backup_id } => {
            info!("Rolling back website: {} to backup: {}", name, backup_id);
            
            let deployment = config.get_deployment(&name)
                .ok_or_else(|| RumiError::configuration(format!("Deployment '{}' not found", name)))?;
            
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would rollback website {} to backup {}", name, backup_id);
                return Ok(());
            }

            let backup_manager = BackupManager::default();
            let backups = backup_manager.list_backups(&session, Some(&name))?;
            
            let backup = backups.iter()
                .find(|b| b.id == backup_id)
                .ok_or_else(|| RumiError::file_operation(format!("Backup '{}' not found", backup_id)))?;

            let restore_path = format!("/var/www/{}_restored", deployment.domain.replace('.', "_"));
            backup_manager.restore_website_backup(&session, backup, &restore_path)?;

            // Update nginx config to point to restored version
            rumi2::commands::websites::update_command(&session, &deployment.domain, &restore_path)?;
            
            info!("Website rollback completed successfully");
        }
        HostingCommands::List => {
            info!("Listing deployments:");
            for deployment in &config.deployments {
                if matches!(deployment.deployment_type, DeploymentType::Website) {
                    println!("  - {}: {} (Website)", deployment.name, deployment.domain);
                }
            }
        }
    }
    Ok(())
}

async fn handle_server_commands(action: ServerCommands, config: &RumiConfig) -> Result<()> {
    match action {
        ServerCommands::Deploy { name, domain, binary_path, port } => {
            info!("Deploying server: {} at domain: {} on port: {}", name, domain, port);
            
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would deploy server {} from {:?}", name, binary_path);
                return Ok(());
            }

            rumi2::commands::servers::install_command(
                &session,
                &domain,
                &name,
                binary_path.to_str().unwrap(),
                &(port as i32),
            )?;
            
            info!("Server deployment completed successfully");
        }
        ServerCommands::Start { name } => {
            info!("Starting server: {}", name);
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would start server {}", name);
                return Ok(());
            }

            session.execute_command_checked(&format!("sudo systemctl start {}", name))?;
            info!("Server started successfully");
        }
        ServerCommands::Stop { name } => {
            info!("Stopping server: {}", name);
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would stop server {}", name);
                return Ok(());
            }

            session.execute_command_checked(&format!("sudo systemctl stop {}", name))?;
            info!("Server stopped successfully");
        }
        ServerCommands::Restart { name } => {
            info!("Restarting server: {}", name);
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would restart server {}", name);
                return Ok(());
            }

            session.execute_command_checked(&format!("sudo systemctl restart {}", name))?;
            info!("Server restarted successfully");
        }
        ServerCommands::Status { name } => {
            info!("Checking server status: {}", name);
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;

            let result = session.execute_command(&format!("sudo systemctl status {}", name))?;
            println!("Server status:\n{}", result.stdout);
        }
    }
    Ok(())
}

async fn handle_ethereum_commands(action: EthereumCommands, config: &RumiConfig) -> Result<()> {
    match action {
        EthereumCommands::Install { 
            name, domain, network_id, http_address, ws_address, external_ip, wallet_address 
        } => {
            info!("Installing Ethereum node: {} at domain: {}", name, domain);
            
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would install Ethereum node {}", name);
                return Ok(());
            }

            let eth_config = rumi2::commands::ethereum::EthereumConfig {
                domain: &domain,
                network_id: &(network_id as i32),
                http_address_ip: &http_address,
                ext_ip: &external_ip,
                unlock_wallet_address: &wallet_address,
                ws_address_ip: &ws_address,
            };

            rumi2::commands::ethereum::install_command(&session, eth_config)?;
            
            info!("Ethereum node installation completed successfully");
        }
    }
    Ok(())
}

async fn handle_backup_commands(action: BackupCommands, config: &RumiConfig) -> Result<()> {
    let backup_manager = BackupManager::default();

    match action {
        BackupCommands::Create { name, backup_type: _ } => {
            info!("Creating backup for deployment: {}", name);
            
            let deployment = config.get_deployment(&name)
                .ok_or_else(|| RumiError::configuration(format!("Deployment '{}' not found", name)))?;
            
            let ssh_config = get_ssh_config_for_deployment(config, &name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would create backup for {}", name);
                return Ok(());
            }

            match &deployment.deployment_type {
                DeploymentType::Website => {
                    let website_path = format!("/var/www/{}*", deployment.domain.replace('.', "_"));
                    let backup = backup_manager.create_website_backup(&session, &name, &deployment.domain, &website_path)?;
                    info!("Website backup created: {}", backup.id);
                }
                _ => {
                    let backup = backup_manager.create_configuration_backup(&session, &name, &deployment.domain)?;
                    info!("Configuration backup created: {}", backup.id);
                }
            }
        }
        BackupCommands::List { name } => {
            info!("Listing backups");
            
            // Use any deployment's SSH config for listing
            let ssh_config = if let Some(deployment_name) = &name {
                get_ssh_config_for_deployment(config, deployment_name)?
            } else if let Some(deployment) = config.deployments.first() {
                get_ssh_config_for_deployment(config, &deployment.name)?
            } else {
                return Err(RumiError::configuration("No deployments found for SSH connection"));
            };

            let session = RumiSession::new(ssh_config)?;
            let backups = backup_manager.list_backups(&session, name.as_deref())?;

            if backups.is_empty() {
                println!("No backups found");
            } else {
                println!("Backups:");
                for backup in backups {
                    println!("  - {}: {} ({}) - {} bytes - {}", 
                        backup.id, backup.deployment_name, backup.domain, 
                        backup.size_bytes, backup.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
                }
            }
        }
        BackupCommands::Restore { backup_id, restore_path } => {
            info!("Restoring backup: {}", backup_id);
            
            // Find which deployment this backup belongs to
            let mut found_deployment = None;
            for deployment in &config.deployments {
                let ssh_config = get_ssh_config_for_deployment(config, &deployment.name)?;
                let session = RumiSession::new(ssh_config)?;
                let backups = backup_manager.list_backups(&session, Some(&deployment.name))?;
                
                if backups.iter().any(|b| b.id == backup_id) {
                    found_deployment = Some(deployment.clone());
                    break;
                }
            }

            let deployment = found_deployment
                .ok_or_else(|| RumiError::file_operation(format!("Backup '{}' not found", backup_id)))?;

            let ssh_config = get_ssh_config_for_deployment(config, &deployment.name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would restore backup {}", backup_id);
                return Ok(());
            }

            let backups = backup_manager.list_backups(&session, Some(&deployment.name))?;
            let backup = backups.iter()
                .find(|b| b.id == backup_id)
                .ok_or_else(|| RumiError::file_operation(format!("Backup '{}' not found", backup_id)))?;

            let target_path = restore_path.unwrap_or_else(|| {
                format!("/var/www/{}_restored", deployment.domain.replace('.', "_"))
            });

            backup_manager.restore_website_backup(&session, backup, &target_path)?;
            info!("Backup restored to: {}", target_path);
        }
        BackupCommands::Delete { backup_id } => {
            info!("Deleting backup: {}", backup_id);
            
            // Find which deployment this backup belongs to
            let mut found_deployment = None;
            for deployment in &config.deployments {
                let ssh_config = get_ssh_config_for_deployment(config, &deployment.name)?;
                let session = RumiSession::new(ssh_config)?;
                let backups = backup_manager.list_backups(&session, Some(&deployment.name))?;
                
                if backups.iter().any(|b| b.id == backup_id) {
                    found_deployment = Some(deployment.clone());
                    break;
                }
            }

            let deployment = found_deployment
                .ok_or_else(|| RumiError::file_operation(format!("Backup '{}' not found", backup_id)))?;

            let ssh_config = get_ssh_config_for_deployment(config, &deployment.name)?;
            let session = RumiSession::new(ssh_config)?;

            if config.settings.dry_run {
                info!("DRY RUN: Would delete backup {}", backup_id);
                return Ok(());
            }

            backup_manager.delete_backup(&session, &backup_id)?;
            info!("Backup deleted successfully");
        }
        BackupCommands::Cleanup { retention_days } => {
            info!("Cleaning up backups older than {} days", retention_days);
            
            for deployment in &config.deployments {
                let ssh_config = get_ssh_config_for_deployment(config, &deployment.name)?;
                let session = RumiSession::new(ssh_config)?;

                if config.settings.dry_run {
                    info!("DRY RUN: Would cleanup backups for {}", deployment.name);
                    continue;
                }

                backup_manager.cleanup_old_backups(&session, retention_days)?;
            }
            info!("Backup cleanup completed");
        }
    }
    Ok(())
}

async fn handle_config_commands(action: ConfigCommands, config: &mut RumiConfig, config_path: &PathBuf) -> Result<()> {
    match action {
        ConfigCommands::Init => {
            info!("Initializing configuration");
            let default_config = RumiConfig::default();
            default_config.save_to_file(config_path)?;
            info!("Configuration initialized at: {:?}", config_path);
        }
        ConfigCommands::AddSsh { name: _, host, user, port, public_key, private_key } => {
            info!("Adding SSH configuration");
            
            let ssh_config = SshConfig {
                host,
                user,
                port: Some(port),
                public_key_path: public_key,
                private_key_path: private_key,
                password: None,
            };

            config.default_ssh = Some(ssh_config);
            config.save_to_file(config_path)?;
            info!("SSH configuration added");
        }
        ConfigCommands::Show => {
            println!("Current configuration:");
            println!("{}", serde_json::to_string_pretty(config)?);
        }
        ConfigCommands::Validate => {
            info!("Validating configuration");
            config.validate()?;
            info!("Configuration is valid");
        }
    }
    Ok(())
}

fn get_ssh_config_for_deployment(config: &RumiConfig, deployment_name: &str) -> Result<SshConfig> {
    // First try to get SSH config from the specific deployment
    if let Some(deployment) = config.get_deployment(deployment_name) {
        if let Some(ssh_config) = &deployment.ssh {
            return Ok(ssh_config.clone());
        }
    }

    // Fall back to default SSH config
    config.default_ssh.clone()
        .ok_or_else(|| RumiError::configuration("No SSH configuration found for deployment"))
}