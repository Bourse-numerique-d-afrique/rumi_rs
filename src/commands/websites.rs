use std::io::prelude::*;
use std::path::Path;
use uuid::Uuid;

use ssh2::Session;
use crate::{NGINX_WEB_CONFIG_PATH, WEB_FOLDER, SSL_CERTIFICATE_PATH, SSL_CERTIFICATE_KEY_PATH};
use crate::utils::{get_web_nginx_config_file, upload_folder, new_channel, close_channel};


pub fn install_command<'a>(session: &'a Session, domain: &'a str, dist_path: &'a str) {
    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo apt update");
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    assert!(command.is_ok(), "Failed to update apt");
    close_channel(&mut chanel);

    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo apt-get -y install ufw");
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    assert!(command.is_ok(), "Failed to install ufw");
    close_channel(&mut chanel);
    
    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo apt install -y nginx certbot");
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    assert!(command.is_ok(), "Failed to install nginx");
    close_channel(&mut chanel);

    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo ufw allow 'Nginx HTTP");
    assert!(command.is_ok(), "Failed to allow Nginx HTTP");
    close_channel(&mut chanel);

    let cerbot_instruction = format!("sudo certbot certonly -y --standalone -d {} -d www.{} --agree-tos --email pondonda@gmail.com", domain, domain);

    let mut chanel = new_channel(session);
    let command = chanel.exec(&cerbot_instruction);
    assert!(command.is_ok(), "Failed to create certificate");
    close_channel(&mut chanel);

    let certificate_path = format!("{}/{}/fullchain.pem", SSL_CERTIFICATE_PATH, domain);
    let certificate_key_path = format!("{}/{}/privkey.pem", SSL_CERTIFICATE_KEY_PATH, domain);

    let random_uuid = Uuid::new_v4().to_string();
    let web_folder_path = format!("{}/{}_{}", WEB_FOLDER, domain, random_uuid);

    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo chmod 777 /var/www/ && sudo chmod 777 /etc/nginx/sites-available/ && sudo chmod 777 /etc/nginx/sites-enabled/");
    assert!(command.is_ok(), "Failed to grant permissions");
    close_channel(&mut chanel);

    let sftp = session.sftp().expect("failed to get sftp");

    let dist_path = Path::new(&dist_path);
    let upload = upload_folder(&sftp,  &dist_path, &web_folder_path);
    assert!(upload.is_ok(), "Failed to upload folder");

    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo rm /etc/nginx/sites-enabled/default");
    assert!(command.is_ok(), "Failed to remove default nginx config");
    close_channel(&mut chanel);

    let nginx_config = get_web_nginx_config_file(domain, &certificate_path, &certificate_key_path, &web_folder_path);

    let config_file_path = format!("{}/{}", NGINX_WEB_CONFIG_PATH, domain);
    let path = Path::new(&config_file_path);
    let mut file = sftp.create(path).expect("failed to create nginx config file");
    file.write_all(nginx_config.as_bytes()).expect("failed to write nginx config file");

    let mut chanel = new_channel(session);
    let command = chanel.exec(format!("sudo ln -s {} /etc/nginx/sites-enabled/ && ls -a /etc/nginx/sites-enabled", config_file_path).as_str());
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    println!("ouptut : {:?}", s);
    assert!(command.is_ok(), "Failed to allow port 80");
    close_channel(&mut chanel);


    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo ufw allow 80 && sudo ufw allow 443 && sudo systemctl restart nginx");
    assert!(command.is_ok(), "Failed to restart nginx");
    close_channel(&mut chanel);
}


pub fn update_command<'a>(session: &'a Session, domain: &'a str, dist_path: &'a str) {
    let certificate_path = format!("{}/{}/fullchain.pem", SSL_CERTIFICATE_PATH, domain);
    let certificate_key_path = format!("{}/{}/privkey.pem", SSL_CERTIFICATE_KEY_PATH, domain);

    let random_uuid = Uuid::new_v4().to_string();
    let web_folder_path = format!("{}/{}_{}", WEB_FOLDER, domain, random_uuid);

    let sftp = session.sftp().expect("failed to get sftp");

    let dist_path = Path::new(&dist_path);
    let upload = upload_folder(&sftp,  &dist_path, &web_folder_path);
    assert!(upload.is_ok(), "Failed to upload folder");

    let nginx_config = get_web_nginx_config_file(domain, &certificate_path, &certificate_key_path, &web_folder_path);

    let config_file_path = format!("{}/{}", NGINX_WEB_CONFIG_PATH, domain);
    let path = Path::new(&config_file_path);
    let mut file = sftp.create(path).expect("failed to create nginx config file");
    file.write_all(nginx_config.as_bytes()).expect("failed to write nginx config file");

    let mut chanel = new_channel(session);
    let command = chanel.exec(format!("sudo ln -s {} /etc/nginx/sites-enabled/ && ls -a /etc/nginx/sites-enabled", config_file_path).as_str());
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    println!("ouptut : {:?}", s);
    assert!(command.is_ok(), "Failed to allow port 80");
    close_channel(&mut chanel);

    // reload nginx without downtime
    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo systemctl reload nginx");
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    println!("ouptut : {:?}", s);
    assert!(command.is_ok(), "Failed to reload nginx");
    close_channel(&mut chanel);
}


pub fn rollback_command<'a>(session: &'a Session, domain: &'a str, version_name: &'a str) {
    let certificate_path = format!("{}/{}/fullchain.pem", SSL_CERTIFICATE_PATH, domain);
    let certificate_key_path = format!("{}/{}/privkey.pem", SSL_CERTIFICATE_KEY_PATH, domain);
    let web_folder_path = format!("{}/{}", WEB_FOLDER, version_name);

    let sftp = session.sftp().expect("failed to get sftp");

    let nginx_config = get_web_nginx_config_file(domain, &certificate_path, &certificate_key_path, &web_folder_path);

    let config_file_path = format!("{}/{}", NGINX_WEB_CONFIG_PATH, domain);
    let path = Path::new(&config_file_path);
    let mut file = sftp.create(path).expect("failed to create nginx config file");
    file.write_all(nginx_config.as_bytes()).expect("failed to write nginx config file");

    let mut chanel = new_channel(session);
    let command = chanel.exec(format!("sudo ln -s {} /etc/nginx/sites-enabled/ && ls -a /etc/nginx/sites-enabled", config_file_path).as_str());
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    println!("ouptut : {:?}", s);
    assert!(command.is_ok(), "Failed to allow port 80");
    close_channel(&mut chanel);

    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo systemctl reload nginx");
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    println!("ouptut : {:?}", s);
    assert!(command.is_ok(), "Failed to reload nginx");
    close_channel(&mut chanel);

    
}
