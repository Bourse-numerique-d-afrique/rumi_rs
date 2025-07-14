use crate::utils::{get_ethereum_nginx_config_file, get_genesis_file, get_startnode_command};
use crate::ETH_GETH_NGINX_CONFIG_PATH;
use crate::session::RumiSession;
use crate::error::Result;

pub struct EthereumConfig<'a> {
    pub domain: &'a str,
    pub network_id: &'a i32,
    pub http_address_ip: &'a str,
    pub ext_ip: &'a str,
    pub unlock_wallet_address: &'a str,
    pub ws_address_ip: &'a str,
}

pub fn install_command(session: &RumiSession, config: EthereumConfig) -> Result<()> {
    // Add Ethereum repository
    session.execute_command_checked("sudo add-apt-repository -y ppa:ethereum/ethereum")?;
    
    // Update and install packages
    session.execute_command_checked("sudo apt -y update")?;
    session.execute_command_checked("sudo apt-get install -y ethereum")?;
    session.execute_command_checked("sudo apt install -y nginx")?;
    session.execute_command_checked("sudo apt install -y certbot")?;
    
    // Get SSL certificate
    let certbot_instruction = format!(
        "sudo certbot certonly -y --standalone -d {} -d www.{}",
        config.domain, config.domain
    );
    session.execute_command_checked(&certbot_instruction)?;

    // Create genesis.json file
    let genesis = get_genesis_file("8eB0f73A356d2083aaEceE9794719f14b0898671", &56584);
    session.execute_command_checked("mkdir -p node")?;
    session.create_remote_file("node/genesis.json", &genesis)?;

    // Create password.sec file
    session.create_remote_file("node/password.sec", "4qF0PF11794591$$")?;

    // Create account
    session.execute_command_checked("geth account new --datadir node/data --password node/password.sec")?;

    // Initialize genesis file
    session.execute_command_checked("geth init --datadir node/data node/genesis.json")?;

    // Create nginx configuration
    let nginx_file = get_ethereum_nginx_config_file(&80, config.domain);
    session.create_remote_file(ETH_GETH_NGINX_CONFIG_PATH, &nginx_file)?;

    // Remove default nginx config
    session.execute_command_checked("sudo rm /etc/nginx/sites-enabled/default")?;

    // Test and reload nginx
    session.execute_command_checked("sudo nginx -t")?;
    session.execute_command_checked("sudo nginx -s reload")?;

    // Configure firewall
    session.execute_command_checked("sudo ufw delete allow 8545/tcp || true")?;
    session.execute_command_checked("sudo ufw delete allow 8546/tcp || true")?;
    session.execute_command_checked("sudo ufw allow 'Nginx Full'")?;
    session.execute_command_checked("sudo ufw allow ssh")?;
    session.execute_command_checked("sudo ufw delete allow http || true")?;
    session.execute_command_checked("sudo ufw enable")?;

    // Start geth
    let start_command = get_startnode_command(
        config.network_id,
        config.http_address_ip,
        config.ext_ip,
        config.unlock_wallet_address,
        config.ws_address_ip,
    );
    session.execute_command(&start_command)?; // Don't check exit code for background process

    Ok(())
}