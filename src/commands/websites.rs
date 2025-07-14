use std::path::Path;
use uuid::Uuid;

use crate::utils::get_web_nginx_config_file;
use crate::{NGINX_WEB_CONFIG_PATH, SSL_CERTIFICATE_KEY_PATH, SSL_CERTIFICATE_PATH, WEB_FOLDER};
use crate::session::RumiSession;
use crate::error::Result;

pub fn install_command(session: &RumiSession, domain: &str, dist_path: &str) -> Result<()> {
    // Update packages
    session.execute_command_checked("sudo apt update")?;
    
    // Install required packages
    session.execute_command_checked("sudo apt-get -y install ufw")?;
    session.execute_command_checked("sudo apt install -y nginx certbot")?;
    
    // Configure firewall
    session.execute_command_checked("sudo ufw allow 'Nginx HTTP'")?;
    
    // Get SSL certificate
    let certbot_instruction = format!(
        "sudo certbot certonly -y --standalone -d {} -d www.{} --agree-tos --email pondonda@gmail.com",
        domain, domain
    );
    session.execute_command_checked(&certbot_instruction)?;

    let certificate_path = format!("{}/{}/fullchain.pem", SSL_CERTIFICATE_PATH, domain);
    let certificate_key_path = format!("{}/{}/privkey.pem", SSL_CERTIFICATE_KEY_PATH, domain);

    let random_uuid = Uuid::new_v4().to_string();
    let web_folder_path = format!("{}/{}_{}", WEB_FOLDER, domain, random_uuid);

    // Set permissions
    session.execute_command_checked(
        "sudo chmod 777 /var/www/ && sudo chmod 777 /etc/nginx/sites-available/ && sudo chmod 777 /etc/nginx/sites-enabled/"
    )?;

    // Upload website files
    let dist_path = Path::new(dist_path);
    session.upload_directory(dist_path, &web_folder_path)?;

    // Remove default nginx config
    session.execute_command_checked("sudo rm /etc/nginx/sites-enabled/default")?;

    // Create nginx configuration
    let nginx_config = get_web_nginx_config_file(
        domain,
        &certificate_path,
        &certificate_key_path,
        &web_folder_path,
    );

    let config_file_path = format!("{}/{}", NGINX_WEB_CONFIG_PATH, domain);
    session.create_remote_file(&config_file_path, &nginx_config)?;

    // Enable site
    let enable_command = format!(
        "sudo ln -s {} /etc/nginx/sites-enabled/ && ls -a /etc/nginx/sites-enabled",
        config_file_path
    );
    session.execute_command_checked(&enable_command)?;

    // Restart nginx
    session.execute_command_checked(
        "sudo ufw allow 80 && sudo ufw allow 443 && sudo systemctl restart nginx"
    )?;

    Ok(())
}

pub fn update_command(session: &RumiSession, domain: &str, dist_path: &str) -> Result<()> {
    let certificate_path = format!("{}/{}/fullchain.pem", SSL_CERTIFICATE_PATH, domain);
    let certificate_key_path = format!("{}/{}/privkey.pem", SSL_CERTIFICATE_KEY_PATH, domain);

    let random_uuid = Uuid::new_v4().to_string();
    let web_folder_path = format!("{}/{}_{}", WEB_FOLDER, domain, random_uuid);

    // Upload new website files
    let dist_path = Path::new(dist_path);
    session.upload_directory(dist_path, &web_folder_path)?;

    // Update nginx configuration
    let nginx_config = get_web_nginx_config_file(
        domain,
        &certificate_path,
        &certificate_key_path,
        &web_folder_path,
    );

    let config_file_path = format!("{}/{}", NGINX_WEB_CONFIG_PATH, domain);
    session.create_remote_file(&config_file_path, &nginx_config)?;

    // Enable site
    let enable_command = format!(
        "sudo ln -s {} /etc/nginx/sites-enabled/ && ls -a /etc/nginx/sites-enabled",
        config_file_path
    );
    session.execute_command_checked(&enable_command)?;

    // Reload nginx without downtime
    session.execute_command_checked("sudo systemctl reload nginx")?;

    Ok(())
}

pub fn rollback_command(session: &RumiSession, domain: &str, version_name: &str) -> Result<()> {
    let certificate_path = format!("{}/{}/fullchain.pem", SSL_CERTIFICATE_PATH, domain);
    let certificate_key_path = format!("{}/{}/privkey.pem", SSL_CERTIFICATE_KEY_PATH, domain);
    let web_folder_path = format!("{}/{}", WEB_FOLDER, version_name);

    // Update nginx configuration to point to the rollback version
    let nginx_config = get_web_nginx_config_file(
        domain,
        &certificate_path,
        &certificate_key_path,
        &web_folder_path,
    );

    let config_file_path = format!("{}/{}", NGINX_WEB_CONFIG_PATH, domain);
    session.create_remote_file(&config_file_path, &nginx_config)?;

    // Enable site
    let enable_command = format!(
        "sudo ln -s {} /etc/nginx/sites-enabled/ && ls -a /etc/nginx/sites-enabled",
        config_file_path
    );
    session.execute_command_checked(&enable_command)?;

    // Reload nginx
    session.execute_command_checked("sudo systemctl reload nginx")?;

    Ok(())
}