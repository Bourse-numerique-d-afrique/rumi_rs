use std::io::Write;
use std::path::Path;

use ssh2::{Channel, Session};
use crate::{NGINX_WEB_CONFIG_PATH, WEB_FOLDER, SSL_CERTIFICATE_PATH, SSL_CERTIFICATE_KEY_PATH};
use crate::utils::{get_web_nginx_config_file, upload_folder};


pub fn install_command<'a>(chanel: &'a mut Channel, session: &'a Session, domain: &'a str, dist_path: &'a str) {
    let command = chanel.exec("sudo apt update");
    assert!(command.is_ok(), "Failed to update apt");
    let command = chanel.exec("sudo apt upgrade");
    assert!(command.is_ok(), "Failed to upgrade apt");
    let command = chanel.exec("sudo apt install nginx");
    assert!(command.is_ok(), "Failed to install nginx");
    let command = chanel.exec("sudo ufw allow 'Nginx HTTP");
    assert!(command.is_ok(), "Failed to allow Nginx HTTP");
    let command = chanel.exec("sudo apt install certbot");
    assert!(command.is_ok(), "Failed to install certbot");
    let cerbot_instruction = format!("sudo certbot certonly -y --standalone -d {} -d www.{}", domain, domain);

    let command = chanel.exec(&cerbot_instruction);
    assert!(command.is_ok(), "Failed to create certificate");

    let certificate_path = format!("{}/{}/fullchain.pem", SSL_CERTIFICATE_PATH, domain);
    let certificate_key_path = format!("{}/{}/privkey.pem", SSL_CERTIFICATE_KEY_PATH, domain);
    let web_folder_path = format!("{}/{}", WEB_FOLDER, domain);

    let sftp = session.sftp().expect("failed to get sftp");

    let dist_path = Path::new(&dist_path);
    let upload = upload_folder(&sftp,  &dist_path, &web_folder_path);
    assert!(upload.is_ok(), "Failed to upload folder");

    let command = chanel.exec("sudo rm /etc/nginx/sites-enabled/default");
    assert!(command.is_ok(), "Failed to remove default nginx config");

    let nginx_config = get_web_nginx_config_file(domain, &certificate_path, &certificate_key_path, &web_folder_path);
    
    let config_file_path = format!("{}/{}", NGINX_WEB_CONFIG_PATH, domain);
    let path = Path::new(&config_file_path);
    let mut file = sftp.create(path).expect("failed to create nginx config file");
    file.write_all(nginx_config.as_bytes()).expect("failed to write nginx config file");

    let command = chanel.exec("sudo ufw allow 80");
    assert!(command.is_ok(), "Failed to allow port 80");
    let command = chanel.exec("sudo ufw allow 443");
    assert!(command.is_ok(), "Failed to allow port 443");

    let command = chanel.exec("sudo systemctl restart nginx");
    assert!(command.is_ok(), "Failed to restart nginx");
}


pub fn update_command<'a>(channel: &'a Channel) {}

pub fn delete_command<'a>(channel: &'a Channel) {


}