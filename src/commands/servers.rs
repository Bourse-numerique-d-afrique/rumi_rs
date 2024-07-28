use std::{io::Write, path::Path};
use std::io::prelude::*;
use std::fs::File;
use ssh2::{Channel, Session};
use crate::utils::{new_channel, close_channel, get_servers_nginx_config_file};




pub fn install_command<'a>(session: &'a Session, domain: &'a str, app_name: &'a str) {
    let mut chanel = new_channel(session);
    let command = chanel.exec("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y");
    assert!(command.is_ok(), "Failed to install rust");
    close_channel(&mut chanel);

    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo apt install -y certbot");
    assert!(command.is_ok(), "Failed to install certbot");
    close_channel(&mut chanel);
    let cerbot_instruction = format!("sudo certbot certonly -y --standalone -d {} -d www.{}", domain, domain);

    let app_release_path = format!("~/target/release/{}", app_name);
    let remote_app_release_path = format!("/usr/local/bin/{}", app_name);

    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo chmod 777 /usr/local/bin/ && sudo chmod 777 /etc/nginx/sites-available/ && sudo chmod 777 /etc/nginx/sites-enabled/");
    assert!(command.is_ok(), "Failed to grant permissions");
    close_channel(&mut chanel);

    let mut local_file = File::open(app_release_path).expect("Failed to open app release file");
    let file_size = local_file.metadata().expect("failed getting file meta data").len();
    let mut remote_file = session.scp_send(Path::new(&remote_app_release_path), 0o755, file_size, None).expect("Failed to create remote file");
   

    let mut chanel = new_channel(session);
    let chmod_command = format!("sudo chmod +x {}", remote_app_release_path);
    let command = chanel.exec(&chmod_command);
    assert!(command.is_ok(), "Failed to set permissions");
    close_channel(&mut chanel);
    
    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo ufw allow 80");
    assert!(command.is_ok(), "Failed to allow port 80");
    close_channel(&mut chanel);

    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo ufw allow 443");
    assert!(command.is_ok(), "Failed to allow port 443");
    close_channel(&mut chanel);
}

pub fn update_command<'a>(session: &'a Session, port: &'a i32, server_port: &'a i32) {

}


pub fn rollback_command<'a>(session: &'a Session, version_name: &'a str, port: &'a i32, server_port: &'a i32) {

}



