


use std::{io::Write, path::Path};
use std::io::prelude::*;

use ssh2::{Channel, Error, Session};
use crate::utils::{get_genesis_file, get_ethereum_nginx_config_file, get_startnode_command};
use crate::ETH_GETH_NGINX_CONFIG_PATH;



pub fn install_command<'a>(chanel: &'a mut Channel, session: &'a Session, domain: &'a str, network_id: &'a i32, http_address_ip: &'a str, ext_ip: &'a str, unlock_wallet_address: &'a str, ws_address_ip: &'a str) {
    let command = chanel.exec("sudo add-apt-repository -y ppa:ethereum/ethereum");
    let mut s = String::new();
    chanel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    assert!(command.is_ok(), "Failed to add ethereum repository");
    
    let command = chanel.exec("sudo apt -y update");
    assert!(command.is_ok(), "Failed to update apt");
    let command = chanel.exec("sudo apt-get install -y ethereum");
    assert!(command.is_ok(), "Failed to install ethereum");
    let command = chanel.exec("sudo apt install -y nginx");
    assert!(command.is_ok(), "Failed to install nginx");
    let command = chanel.exec("sudo apt install -y certbot");
    assert!(command.is_ok(), "Failed to install certbot");
    let cerbot_instruction = format!("sudo certbot certonly -y --standalone -d {} -d www.{}", domain, domain);
    let command = chanel.exec(&cerbot_instruction);
    assert!(command.is_ok(), "Failed to get certificate");

    // create genesis.json file
    let genesis = get_genesis_file("8eB0f73A356d2083aaEceE9794719f14b0898671", &56584);
    let sftp = session.sftp().expect("failed to get sftp");
    let path = Path::new("node/genesis.json");
    let mut file = sftp.create(path).expect("failed to create genesis.json");
    file.write_all(genesis.as_bytes()).expect("failed to write genesis.json");

    // create password.sec file
    let path = Path::new("node/password.sec");
    let mut file = sftp.create(path).expect("failed to create password.sec");
    file.write_all(b"4qF0PF11794591$$").expect("failed to write password.sec");


    // create account
    let command: Result<(), Error> = chanel.exec("geth account new --datadir node/data  --password node/password.sec");
    assert!(command.is_ok(), "Failed to create account");

    // init genesis file
    let command: Result<(), Error> = chanel.exec("geth init --datadir node/data  node/genesis.json");
    assert!(command.is_ok(), "Failed to create genesis file");

    let sftp = session.sftp().expect("failed to get sftp");
    let nginx_file = get_ethereum_nginx_config_file(&80, domain);
    let path = Path::new(ETH_GETH_NGINX_CONFIG_PATH);
    let mut file = sftp.create(path).expect("failed to create nginx config file");
    file.write_all(nginx_file.as_bytes()).expect("failed to write nginx config file");

    let command = chanel.exec("sudo rm /etc/nginx/sites-enabled/default");
    assert!(command.is_ok(), "Failed to remove default nginx config");

    let command = chanel.exec("sudo nginx -t");
    assert!(command.is_ok(), "Failed to test nginx config");
    let command = chanel.exec("sudo nginx -s reload");
    assert!(command.is_ok(), "Failed to reload nginx");

    // If you want to be secure you should disable access to ports 8545 and 8546 from the outside again with:
    let command = chanel.exec("sudo ufw delete allow 8545/tcp");
    assert!(command.is_ok(), "Failed to delete port 8545");
    let command = chanel.exec("sudo ufw delete allow 8546/tcp");
    assert!(command.is_ok(), "Failed to delete port 8546");

    let command = chanel.exec("sudo ufw allow 'Nginx Full'");
    assert!(command.is_ok(), "Failed to allow nginx");
    let command = chanel.exec("sudo ufw allow ssh");
    assert!(command.is_ok(), "Failed to allow ssh");
    let command = chanel.exec("sudo ufw delete allow http");
    assert!(command.is_ok(), "Failed to delete http");
    let command = chanel.exec("sudo ufw enable");
    assert!(command.is_ok(), "Failed to enable ufw");

    // start geth
    let start_command = get_startnode_command(network_id, http_address_ip, ext_ip, unlock_wallet_address, ws_address_ip);
    let command: Result<(), Error> = chanel.exec(&start_command);
    assert!(command.is_ok(), "Failed to start geth");

}


pub fn install_with_bootnode_command<'a>(chanel: &'a mut Channel, session: &'a Session, domain: &'a str, network_id: &'a i32, http_address_ip: &'a str, ext_ip: &'a str, unlock_wallet_address: &'a str, ws_address_ip: &'a str, bootnode: &'a str) {

}

pub fn remove_node_command<'a>(chanel: &'a mut Channel, session: &'a Session, domain: &'a str) {

}
