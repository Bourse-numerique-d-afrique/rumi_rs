//! # Rumi2 - Deployment Management CLI
//!
//! Rumi2 is a robust CLI tool for deploying web applications, servers, and blockchain nodes 
//! with seamless SSH2 integration. It simplifies the deployment process to your existing 
//! server infrastructure, ensuring secure and efficient transfers with comprehensive backup, 
//! rollback, and monitoring capabilities.
//!
//! ## Features
//!
//! - **Website Hosting**: One-command deployments with automatic SSL certificate generation
//! - **Server Management**: Binary deployment with automatic service management  
//! - **Ethereum Node Deployment**: Full Ethereum node setup with geth
//! - **Backup & Recovery**: Automatic backups before deployments with easy restoration
//! - **Configuration Management**: JSON-based configuration with validation
//! - **Security & Safety**: Dry-run mode, SSH key authentication, firewall management
//!
//! ## Quick Start
//!
//! ```bash
//! # Initialize configuration
//! rumi2 config init
//!
//! # Add SSH connection details
//! rumi2 config add-ssh --name prod --host server.com --user deploy
//!
//! # Deploy a website
//! rumi2 hosting install --name my-site --domain example.com --dist-path ./dist
//! ```
//!
//! ## Architecture
//!
//! The library is organized into several key modules:
//!
//! - [`config`] - Configuration management and validation
//! - [`session`] - SSH session management and command execution
//! - [`backup`] - Backup and restore functionality
//! - [`commands`] - Deployment command implementations
//! - [`error`] - Error types and handling
//!
//! ## Examples
//!
//! ```rust,no_run
//! use rumi2::{config::RumiConfig, session::RumiSession};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Load configuration
//! let config_path = RumiConfig::get_config_path();
//! let config = RumiConfig::load_from_file(&config_path)?;
//!
//! // Create SSH session
//! if let Some(ssh_config) = config.default_ssh {
//!     let session = RumiSession::new(ssh_config)?;
//!     
//!     // Execute commands safely
//!     let result = session.execute_command("uptime")?;
//!     println!("Server uptime: {}", result.stdout);
//! }
//! # Ok(())
//! # }
//! ```

use ssh2::Session;
use std::net::TcpStream;

pub mod commands;
pub mod config;
pub mod error;
pub mod session;
pub mod backup;

#[cfg(test)]
mod tests;

/// Default path for server binary installations on Unix systems
pub const SERVER_BIN_PATH: &str = "/usr/local/bin";

/// Path where nginx site configuration files are stored (available sites)
pub const NGINX_WEB_CONFIG_PATH: &str = "/etc/nginx/sites-available";

/// Path where nginx enabled site configuration files are symlinked
pub const NGINX_WEB_SITE_ENABLED: &str = "/etc/nginx/sites-enabled";

/// Default web root directory for hosting website files
pub const WEB_FOLDER: &str = "/var/www";

/// Path where Let's Encrypt SSL certificates are stored
pub const SSL_CERTIFICATE_PATH: &str = "/etc/letsencrypt/live";

/// Path where Let's Encrypt SSL certificate private keys are stored
pub const SSL_CERTIFICATE_KEY_PATH: &str = "/etc/letsencrypt/live";

/// Path for Ethereum geth nginx proxy configuration
pub const ETH_GETH_NGINX_CONFIG_PATH: &str = "/etc/nginx/conf.d/geth.conf";

pub struct Rumi2 {}

impl Rumi2 {
    pub fn start(
        host: String,
        username: String,
        pubkeydata: String,
        privatekeydata: String,
        passphrase: String,
    ) -> Session {
        let tcp = TcpStream::connect(format!("{host}:22")).expect("Failed to connect to tcp");
        let mut session = Session::new().expect("Session could not be started");
        session.set_tcp_stream(tcp);
        session.handshake().expect("handshade didn't worked");
        session
            .userauth_pubkey_memory(
                &username,
                Some(&pubkeydata),
                &privatekeydata,
                Some(&passphrase),
            )
            .expect("Unable to authenticate with the given parameters");
        session
    }
}

pub mod ufw {
    use std::io::Read;

    use crate::utils::{close_channel, new_channel};
    use ssh2::Session;

    /// The install command for ufw
    ///
    pub fn install(session: &Session) {
        let mut chanel = new_channel(session);
        let command = chanel.exec("sudo apt-get -y install ufw");
        let mut s = String::new();
        chanel.read_to_string(&mut s).unwrap();
        assert!(command.is_ok(), "Failed to install ufw");
        close_channel(&mut chanel);
    }

    pub fn allow_nginx_http(session: &Session) {
        let mut chanel = new_channel(session);
        let command = chanel.exec("sudo ufw allow 'Nginx HTTP");
        assert!(command.is_ok(), "Failed to allow Nginx HTTP");
        close_channel(&mut chanel);
    }

    pub fn allow_port_and_443(session: &Session) {
        let mut chanel = new_channel(session);
        let command =
            chanel.exec("sudo ufw allow 80 && sudo ufw allow 443 && sudo systemctl restart nginx");
        assert!(command.is_ok(), "Failed to restart nginx");
        close_channel(&mut chanel);
    }

    pub fn allow_port<'a>(session: &'a Session, port: &'a i32) {
        let mut chanel = new_channel(session);
        let command_string = format!("sudo ufw allow {port} && sudo systemctl restart nginx");
        let command = chanel.exec(&command_string);
        assert!(command.is_ok(), "Failed to restart nginx");
        close_channel(&mut chanel);
    }
}

pub mod nginx {
    use crate::utils::{close_channel, new_channel};
    use ssh2::Session;
    use std::io::Read;

    pub fn install(session: &Session) {
        let mut chanel = new_channel(session);
        let command = chanel.exec("sudo apt install -y nginx");
        let mut s = String::new();
        chanel.read_to_string(&mut s).unwrap();
        assert!(command.is_ok(), "Failed to install nginx");
        close_channel(&mut chanel);
    }

    pub fn enable_write_to_folders(session: &Session) {
        let mut chanel = new_channel(session);
        let command = chanel.exec("sudo chmod 777 /var/www/ && sudo chmod 777 /etc/nginx/sites-available/ && sudo chmod 777 /etc/nginx/sites-enabled/");
        assert!(command.is_ok(), "Failed to grant permissions");
        close_channel(&mut chanel);
    }

    pub fn make_site_enabled<'a>(session: &'a Session, config_file_path: &'a str) {
        let mut chanel = new_channel(session);
        let command = chanel.exec(
            format!(
                "sudo ln -s {} /etc/nginx/sites-enabled/ && ls -a /etc/nginx/sites-enabled",
                config_file_path
            )
            .as_str(),
        );
        let mut s = String::new();
        chanel.read_to_string(&mut s).unwrap();
        println!("ouptut : {:?}", s);
        assert!(command.is_ok(), "Failed to allow port 80");
        close_channel(&mut chanel);
    }

    pub fn remove_default_enable_folder(session: &Session) {
        let mut chanel = new_channel(session);
        let command = chanel.exec("sudo rm /etc/nginx/sites-enabled/default");
        assert!(command.is_ok(), "Failed to remove default nginx config");
        close_channel(&mut chanel);
    }

    pub fn restart(session: &Session) {
        let mut chanel = new_channel(session);
        let command =
            chanel.exec("sudo ufw allow 80 && sudo ufw allow 443 && sudo systemctl restart nginx");
        assert!(command.is_ok(), "Failed to restart nginx");
        close_channel(&mut chanel);
    }

    pub fn reload(session: &Session) {
        // reload nginx without downtime
        let mut chanel = new_channel(session);
        let command = chanel.exec("sudo systemctl reload nginx");
        let mut s = String::new();
        chanel.read_to_string(&mut s).unwrap();
        println!("ouptut : {:?}", s);
        assert!(command.is_ok(), "Failed to reload nginx");
        close_channel(&mut chanel);
    }
}

pub mod certbot {
    use crate::utils::{close_channel, new_channel};
    use ssh2::Session;
    use std::io::Read;

    pub fn install(session: &Session) {
        let mut chanel = new_channel(session);
        let command = chanel.exec("sudo apt install -y certbot");
        let mut s = String::new();
        chanel.read_to_string(&mut s).unwrap();
        assert!(command.is_ok(), "Failed to install nginx");
        close_channel(&mut chanel);
    }

    pub fn get_ssl_certificate_for_domain<'a>(
        session: &'a Session,
        domain: &'a str,
        email: &'a str,
    ) {
        let cerbot_instruction = format!(
            "sudo certbot certonly -y --standalone -d {} -d www.{} --agree-tos --email {}",
            domain, domain, email
        );

        let mut chanel = new_channel(session);
        let command = chanel.exec(&cerbot_instruction);
        assert!(command.is_ok(), "Failed to create certificate");
        close_channel(&mut chanel);
    }
}

pub mod utils {
    use std::{
        fs::{self, File},
        io::{Read, Write},
        path::Path,
    };

    use ssh2::{Channel, Session};

    pub fn new_channel(session: &Session) -> Channel {
        
        session.channel_session().unwrap()
    }

    pub fn close_channel(channel: &mut Channel) {
        channel.wait_close().expect("closing channel failed");
    }

    pub fn get_servers_nginx_config_file<'a>(
        port: &'a i32,
        domain: &'a str,
        server_port: &'a i32,
    ) -> String {
        // the port nginx is listening doesnt change but the proxy_pass port can change has it depend
        // on which server version is in production right now.
        format!(
            r#"
        server {{
          listen {port};
          listen [::]:{port};
          server_name {domain} www.{domain};

          location ^~ / {{
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header Host $http_host;
            proxy_set_header X-NginX-Proxy true;
            proxy_pass http://127.0.0.1:{server_port}/;
          }}
        }}
        "#
        )
    }

    pub fn get_web_nginx_config_file<'a>(
        domain: &'a str,
        ssl_fullchain_path: &'a str,
        ssl_pem_path: &'a str,
        website_dist_path: &'a str,
    ) -> String {
        // https://medium.com/@kornchotpitakkul/deploy-a-node-js-and-vue-js-with-nginx-ssl-on-ubuntu-465f31216dc9
        format!(
            r#"
            server {{
                 listen      80;
                 listen      [::]:80;
                 server_name {domain} www.{domain};
                 return 301  https://$server_name$request_uri;
            }}
            server {{
                 listen       443 ssl http2;
                 listen       [::]:443 ssl http2;
                 server_name  {domain} www.{domain};
                 ssl_certificate {ssl_fullchain_path};
                 ssl_certificate_key {ssl_pem_path};
                 root {website_dist_path};
                 index  index.html;
                 location / {{
                      root   {website_dist_path};
                      index  index.html;
                      try_files $uri $uri/ /index.html;
                 }}
                 error_page  500 502 503 504  /50x.html;
                 location = /50x.html {{
                      root   /usr/share/nginx/html;
                 }}
            }}
            "#
        )
    }

    pub fn get_ethereum_nginx_config_file<'a>(port: &'a i32, domain: &'a str) -> String {
        format!(
            r#"
            server {{
              listen {port};
              listen [::]:{port};
              server_name {domain} www.{domain};

              location ^~ /ws {{
                proxy_http_version 1.1;
                proxy_set_header Upgrade $http_upgrade;
                proxy_set_header Connection "upgrade";
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                proxy_set_header Host $http_host;
                proxy_set_header X-NginX-Proxy true;
                proxy_pass http://127.0.0.1:8546/;
              }}

              location ^~ /rpc {{
                proxy_http_version 1.1;
                proxy_set_header Upgrade $http_upgrade;
                proxy_set_header Connection "upgrade";
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                proxy_set_header Host $http_host;
                proxy_set_header X-NginX-Proxy true;
                proxy_pass http://127.0.0.1:8545/;
              }}
            }}
            "#
        )
    }

    pub fn get_startnode_command<'a>(
        newtork_id: &'a i32,
        http_address_ip: &'a str,
        ext_ip: &'a str,
        unlock_wallet_address: &'a str,
        ws_address_ip: &'a str,
    ) -> String {
        format!(
            r#"nohup geth --networkid {newtork_id}  --datadir data --nodiscover --http --http.port "8545"  --port "30303" --http.addr "{http_address_ip}"  --http.corsdomain "*" --nat any --http.api "eth,web3,personal,net,miner,admin" --http.vhosts "*" --nat extip:{ext_ip}  --unlock '{unlock_wallet_address}' --password './password.sec'  --mine --miner.threads 4  --ipcpath "./data/geth.ipc" --allow-insecure-unlock --miner.etherbase '{unlock_wallet_address}' --miner.gasprice 1  --syncmode full --ws --ws.addr "{ws_address_ip}"  --ws.api "eth,net,web3,admin" --ws.origins "*""#
        )
    }

    pub fn get_genesis_file<'a>(address: &'a str, chain_id: &'a i32) -> String {
        format!(
            r#"
            {{
              "config": {{
                "chainId": {chain_id},
                "homesteadBlock": 0,
                "eip150Block": 0,
                "eip155Block": 0,
                "eip158Block": 0,
                "byzantiumBlock": 0,
                "constantinopleBlock": 0,
                "petersburgBlock": 0,
                "istanbulBlock": 0,
                "berlinBlock": 0,
                "clique": {{
                  "period": 1,
                  "epoch": 30000
                }}
              }},
              "difficulty": "1",
              "gasLimit": "8000000",
              "extradata": "0x0000000000000000000000000000000000000000000000000000000000000000{address}0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
              "alloc": {{
                "{address}": {{ "balance": "300000000" }},
                "f41c74c9ae680c1aa78f42e5647a62f353b7bdde": {{ "balance": "40000000" }}
              }}
            }}
           "#,
            address = address,
            chain_id = chain_id
        )
    }

    pub fn upload_folder(
        sftp: &ssh2::Sftp,
        local_path: &Path,
        remote_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create the remote directory
        match sftp.mkdir(Path::new(remote_path), 0o755) {
            Ok(_) => println!("Created directory: {}", remote_path),
            Err(e) => println!(
                "Directory already exists or failed to create: {} - {}",
                remote_path, e
            ),
        }

        // Iterate over the entries in the local directory
        for entry in fs::read_dir(local_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().into_string().unwrap();
            let remote_file_path = format!("{}/{}", remote_path, file_name);

            if path.is_dir() {
                // Recursively upload directories
                upload_folder(sftp, &path, &remote_file_path)?;
            } else {
                // Upload files
                upload_file(sftp, &path, &remote_file_path)?;
            }
        }

        Ok(())
    }

    pub fn upload_file(
        sftp: &ssh2::Sftp,
        local_file: &Path,
        remote_file: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut local_f = File::open(local_file)?;
        let mut buffer = Vec::new();
        local_f.read_to_end(&mut buffer)?;

        let mut remote_f = sftp.create(Path::new(remote_file))?;
        remote_f.write_all(&buffer)?;

        println!("Uploaded file: {}", remote_file);

        Ok(())
    }
}
