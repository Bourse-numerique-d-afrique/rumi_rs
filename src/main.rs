use rumi::utils::get_genesis_file;
use ssh2::{Channel, Session};
use std::io::{Error, Write};
use std::net::TcpStream;
// use rumi::commands::websites::{install_command, update_command, rollback_command};
use rumi::commands::servers::{install_command, update_command, rollback_command};


use std::ffi::OsString;
use std::path::{Path, PathBuf};

use clap::{arg, Command};




fn cli() -> Command {
    Command::new("hosting")
        .about("Hosting cli to help publish new website to a server via ssh")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("deploy")
                .about("Deploy a website to a ssh")
                .arg(arg!(<SSH_HOST> "the ssh host"))
                .arg(arg!(<SSH_USER> "the ssh user"))
                .arg(arg!(<SSH_PASSWORD> "the ssh password"))
                .arg(arg!(<WEBSITE_URL> "the url of the website"))
                .arg(arg!(<WEBSITE_FOLDER_PATH> "the folder of the website to use for the deployment"))
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("update")
                .about("Update an existing website")
                .arg(arg!(<SSH_HOST> "the ssh host"))
                .arg(arg!(<SSH_USER> "the ssh user"))
                .arg(arg!(<SSH_PASWWORD> "the ssh password"))
                .arg(arg!(<WEBSITE_FOLDER_PATH> "the folder of the website to use for the update"))
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("delete")
                .about("Delete an existing website")
                .arg(arg!(<SSH_HOST> "the ssh host"))
                .arg(arg!(<SSH_USER> "the ssh user"))
                .arg(arg!(<SSH_PASWWORD> "the ssh password"))
                .arg_required_else_help(true),
        )
}

fn get_session_and_channel<'a>(
    ssh_host: &'a str,
    ssh_user: &'a str,
    ssh_password: &'a str,
) -> (Session, Channel) {
    let tcp = TcpStream::connect(ssh_host).expect("failed to connect to tcp");
    let mut sess = Session::new().expect("session cann't be started");
    sess.set_tcp_stream(tcp);
    sess.handshake().expect("handshake didint worked");
    sess.userauth_password(ssh_user, ssh_password)
        .expect("failed to connect to the ssh host using the user, password combination");
    assert!(sess.authenticated());
    let channel = sess.channel_session().expect("failed t get the channel");
    (sess, channel)
}



fn main() -> Result<(), Error> {
    let tcp = TcpStream::connect("62.210.163.54:22").expect("failed to connect to tcp");
    let mut sess = Session::new().expect("session cann't be started");
    sess.set_tcp_stream(tcp);
    sess.handshake().expect("handshake didint worked");
    let privatekey_path = Path::new("/Users/cornalinehumbert/.ssh/id_ed25519");
    let public_key_path = Path::new("/Users/cornalinehumbert/.ssh/id_ed25519.pub");
    //sess.userauth_pubkey_memory(username, pubkeydata, privatekeydata, passphrase)
    sess.userauth_pubkey_file("ubuntu", Some(public_key_path), privatekey_path, Some("4qF0PF")).expect("Unuable to authenticate");
    assert!(sess.authenticated());
    println!("i'm authenticated");

    // install_command(&sess, "testweb.boursenumeriquedafrique.com", "/Users/cornalinehumbert/javascript/webtesting/dist");
    // rollback_command(&sess, "testweb.boursenumeriquedafrique.com", "testweb.boursenumeriquedafrique.com_fe3ccc56-1851-4c76-a44a-94715570c31c");
    // update_command(&sess, "testweb.boursenumeriquedafrique.com", "/Users/cornalinehumbert/javascript/webtesting/dist");
    Ok(())

    // let matches = cli().get_matches();

    // match matches.subcommand() {
    //     Some(("deploy", sub_matches)) => {
    //         let ssh_host = sub_matches.get_one::<String>("SSH_HOST");
    //         assert!(ssh_host.is_some(), "");
    //         let ssh_user = sub_matches.get_one::<String>("SSH_USER");
    //         assert!(ssh_user.is_some(), "");
    //         let ssh_password = sub_matches.get_one::<String>("SSH_PASSWORD");
    //         assert!(ssh_password.is_some(), "");
    //         let website_url = sub_matches.get_one::<String>("WEBSITE_URL");
    //         assert!(website_url.is_some(), "");
    //         let website_folder_path = sub_matches.get_one::<String>("WEBSITE_FOLDER_PATH");
    //         assert!(website_folder_path.is_some(), "");

    //         let (session, mut channel) = get_session_and_channel(
    //             &ssh_host.unwrap(),
    //             &ssh_user.unwrap(),
    //             &ssh_password.unwrap(),
    //         );
    //         Ok(())
    //     }
    //     Some(("update", sub_matches)) => {
    //         let ssh_host = sub_matches.get_one::<String>("SSH_HOST");
    //         assert!(ssh_host.is_some(), "");
    //         let ssh_user = sub_matches.get_one::<String>("SSH_USER");
    //         assert!(ssh_user.is_some(), "");
    //         let ssh_password = sub_matches.get_one::<String>("SSH_PASSWORD");
    //         assert!(ssh_password.is_some(), "");
    //         let website_folder_path = sub_matches.get_one::<String>("WEBSITE_FOLDER_PATH");
    //         assert!(website_folder_path.is_some(), "");
    //         let (session, mut channel) = get_session_and_channel(
    //             &ssh_host.unwrap(),
    //             &ssh_user.unwrap(),
    //             &ssh_password.unwrap(),
    //         );
    //         Ok(())
    //     }
    //     Some(("delete", sub_matches)) => {
    //         let ssh_host = sub_matches.get_one::<String>("SSH_HOST");
    //         assert!(ssh_host.is_some(), "");
    //         let ssh_user = sub_matches.get_one::<String>("SSH_USER");
    //         assert!(ssh_user.is_some(), "");
    //         let ssh_password = sub_matches.get_one::<String>("SSH_PASSWORD");
    //         assert!(ssh_password.is_some(), "");
    //         let (session, mut channel) = get_session_and_channel(
    //             &ssh_host.unwrap(),
    //             &ssh_user.unwrap(),
    //             &ssh_password.unwrap(),
    //         );

    //         Ok(())
    //     }
    //     _ => unreachable!(),
    // }
}
// fn main() -> Result<(), Error> {
//     // let output = if cfg!(target_os = "windows") {
//     //     Command::new("cmd")
//     //         .args(["/C", "echo hello"])
//     //         .output()
//     //         .expect("failed to execute process")
//     // } else {
//     //     Command::new("sh")
//     //         .arg("-c")
//     //         .arg("echo hello")
//     //         .output()
//     //         .expect("failed to execute process")
//     // };
//     //

//     //Command::new("/bin/bash -c '$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)'").status().expect("failed instaalling homebrew");
//     Command::new("sh")
//         .arg("-c")
//         .arg("xcode-select --install")
//         .status()
//         .expect("failed to install xcode command line tools");
//     Command::new("sh")
//         .arg("-c")
//         .arg("brew update")
//         .status()
//         .expect("failed to brew updte");
//     Command::new("sh")
//         .arg("-c")
//         .arg(" brew tap ethereum/ethereum")
//         .status()
//         .expect("failed to tap ethereum");
//     Command::new("sh")
//         .arg("-c")
//         .arg("brew install ethereum")
//         .status()
//         .expect("failed to install ethereum");
//     Command::new("sh")
//         .arg("-c")
//         .arg("brew upgrade ethereum")
//         .status()
//         .expect("failed to upgrade ethereum");

//     Command::new("sh")
//         .arg("-c")
//         .arg("mkdir ethereum && cd ethereum")
//         .status()
//         .expect("failed to upgrade ethereum");

//     let genesis = get_genesis_file("8eB0f73A356d2083aaEceE9794719f14b0898671", &56584);
//     let mut file = File::create("ethereum/genesis.json")?;
//     file.write_all(genesis.as_bytes())?;

//     let mut file = File::create("ethereum/password.sec")?;
//     file.write_all(b"4qF0PF11794591$$")?;

//     Command::new("sh")
//         .arg("-c")
//         .arg("geth account new --datadir ethereum/data  --password ethereum/password.sec")
//         .status()
//         .expect("failed to upgrade ethereum");

//     Command::new("sh")
//         .arg("-c")
//         .arg("geth init --datadir ethereum/data  ethereum/genesis.json")
//         .status()
//         .expect("fialed to create genesis file");

//     Ok(())
// }
