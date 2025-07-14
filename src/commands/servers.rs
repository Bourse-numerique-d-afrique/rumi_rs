use crate::utils::get_servers_nginx_config_file;
use crate::NGINX_WEB_CONFIG_PATH;
use crate::session::RumiSession;
use crate::error::Result;
use std::path::Path;
use uuid::Uuid;

pub fn install_command(
    session: &RumiSession,
    domain: &str,
    app_name: &str,
    bin_path: &str,
    port: &i32,
) -> Result<()> {
    // Install required packages
    session.execute_command_checked("sudo apt-get -y install ufw")?;
    session.execute_command_checked("sudo apt install -y nginx")?;
    session.execute_command_checked("sudo apt install -y certbot")?;
    
    // Configure firewall
    session.execute_command_checked("sudo ufw allow 'Nginx HTTP'")?;
    
    // Get SSL certificate
    let certbot_instruction = format!(
        "sudo certbot certonly -y --standalone -d {} -d www.{} --agree-tos --email pondonda@gmail.com",
        domain, domain
    );
    session.execute_command_checked(&certbot_instruction)?;

    let app_release_path = format!("{}/{}", bin_path, app_name);
    let id = Uuid::new_v4();
    let app_name_full = format!("{}_{}", id, app_name);
    let remote_app_release_path = format!("/usr/local/bin/{}", app_name_full);

    // Set permissions
    session.execute_command_checked(
        "sudo chmod 777 /var/www/ && sudo chmod 777 /etc/nginx/sites-available/ && sudo chmod 777 /etc/nginx/sites-enabled/"
    )?;
    session.execute_command_checked("sudo chmod 777 /usr/local/bin/")?;

    // Upload the binary
    session.upload_file(Path::new(&app_release_path), &remote_app_release_path)?;

    // Make binary executable
    let chmod_command = format!("sudo chmod +x {}", remote_app_release_path);
    session.execute_command_checked(&chmod_command)?;

    // Start the server
    let start_command = format!("nohup {} &", remote_app_release_path);
    session.execute_command(&start_command)?; // Don't check exit code for nohup

    // Allow port through firewall
    let ufw_command = format!("sudo ufw allow {}", port);
    session.execute_command_checked(&ufw_command)?;

    // Create nginx configuration
    let nginx_config = get_servers_nginx_config_file(&3000, domain, port);
    let config_file_path = format!("{}/{}", NGINX_WEB_CONFIG_PATH, domain);
    session.create_remote_file(&config_file_path, &nginx_config)?;

    // Enable site
    let enable_command = format!("sudo ln -s {} /etc/nginx/sites-enabled/", config_file_path);
    session.execute_command_checked(&enable_command)?;

    // Restart nginx
    session.execute_command_checked(
        "sudo ufw allow 80 && sudo ufw allow 443 && sudo systemctl restart nginx"
    )?;

    Ok(())
}