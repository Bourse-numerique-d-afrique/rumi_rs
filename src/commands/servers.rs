use crate::utils::{close_channel, get_servers_nginx_config_file, new_channel};
use crate::NGINX_WEB_CONFIG_PATH;
use crate::{certbot, nginx, ufw};
use ssh2::Session;
use std::fs::File;
use std::io::prelude::*;
use std::{io::Write, path::Path};
use uuid::Uuid;

pub fn install_command<'a>(
    session: &'a Session,
    domain: &'a str,
    app_name: &'a str,
    bin_path: &'a str,
    port: &'a i32,
) {
    ufw::install(session);
    nginx::install(session);
    certbot::install(session);
    ufw::allow_nginx_http(session);
    certbot::get_ssl_certificate_for_domain(session, domain, "pondonda@gmail.com");

    let app_release_path = format!("{}/{}", bin_path, app_name);
    let id = Uuid::new_v4();
    let app_name_full = format!("{}_{}", id.to_string(), app_name);
    let remote_app_release_path = format!("/usr/local/bin/{}", app_name_full);

    nginx::enable_write_to_folders(session);

    let mut chanel = new_channel(session);
    let command = chanel.exec("sudo chmod 777 /usr/local/bin/");
    assert!(command.is_ok(), "Failed to set permissions");
    close_channel(&mut chanel);

    let mut local_file = File::open(app_release_path).expect("Failed to open app release file");
    let file_size = local_file
        .metadata()
        .expect("failed getting file meta data")
        .len();
    let mut remote_file = session
        .scp_send(Path::new(&remote_app_release_path), 0o755, file_size, None)
        .expect("Failed to create remote file");
    let mut buffer = Vec::new();
    local_file
        .read_to_end(&mut buffer)
        .expect("failed to read to end");

    remote_file.write_all(&buffer).expect("failed to write all");
    remote_file.send_eof().expect("dddd");
    remote_file.wait_eof().expect("dddd");
    remote_file.close().expect("error closing");
    remote_file.wait_close().expect("dsdsd");

    let mut chanel = new_channel(session);
    let chmod_command = format!("sudo chmod +x {}", remote_app_release_path);
    let command = chanel.exec(&chmod_command);
    assert!(command.is_ok(), "Failed to set permissions");
    close_channel(&mut chanel);

    let mut chanel = new_channel(session);
    let command = chanel.exec(format!("nohup ./{}", &remote_app_release_path).as_str());
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    assert!(command.is_ok(), "Failed to launch the server");
    close_channel(&mut chanel);

    ufw::allow_port(session, port);
    let sftp = session.sftp().expect("failed to get sftp");
    let nginx_config = get_servers_nginx_config_file(&3000, domain, port);

    let config_file_path = format!("{}/{}", NGINX_WEB_CONFIG_PATH, domain);
    let path = Path::new(&config_file_path);
    let mut file = sftp
        .create(path)
        .expect("failed to create nginx config file");
    file.write_all(nginx_config.as_bytes())
        .expect("failed to write nginx config file");
    nginx::make_site_enabled(session, &config_file_path);
    nginx::restart(session)
}
