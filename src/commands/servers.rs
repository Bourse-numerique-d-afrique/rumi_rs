use std::{io::Write, path::Path};
use std::io::prelude::*;
use std::fs::File;
use ssh2::{Channel, Error,  Session};




pub fn install_command<'a>(chanel: &'a mut Channel, session: &'a Session, domain: &'a str, app_name: &'a str) {
    let command = chanel.exec("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y");
    assert!(command.is_ok(), "Failed to install rust");
    let command = chanel.exec("sudo apt install -y certbot");
    assert!(command.is_ok(), "Failed to install certbot");
    let cerbot_instruction = format!("sudo certbot certonly -y --standalone -d {} -d www.{}", domain, domain);

    let app_release_path = format!("~/target/release/{}", app_name);
    let remote_app_release_path = format!("/usr/local/bin/{}", app_name);

    let mut local_file = File::open(app_release_path).expect("Failed to open app release file");
    let file_size = local_file.metadata().expect("failed getting file meta data").len();
    let mut remote_file = session.scp_send(Path::new(&remote_app_release_path), 0o755, file_size, None).expect("Failed to create remote file");
    // sudo chmod +x /usr/local/bin/my_app
        // Copy the local file to the remote file
    let mut buffer = Vec::new();
    local_file.read_to_end(&mut buffer).expect("Failed to read local file");
    remote_file.write_all(&buffer).expect("Failed to write to remote file");

    let chmod_command = format!("sudo chmod +x {}", remote_app_release_path);
    let command = chanel.exec(&chmod_command);
    assert!(command.is_ok(), "Failed to set permissions");
    let command = chanel.exec("sudo ufw allow 80");
    assert!(command.is_ok(), "Failed to allow port 80");
    let command = chanel.exec("sudo ufw allow 443");
    assert!(command.is_ok(), "Failed to allow port 443");
}

pub fn update_command<'a>(channel: &'a Channel, session: &'a Session,) {}



