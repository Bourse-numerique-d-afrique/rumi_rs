use std::ffi::OsString;
use std::path::PathBuf;

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
                .arg(arg!(<SSH_USER> "the ssh user"))
                .arg(arg!(<SSH_PASSWORD> "the ssh password"))
                .arg(arg!(<WEBSITE_URL> "the url of the website"))
                .arg(arg!(<WEBSITE_FOLDER_PATH> "the folder of the website to use for the deployment"))
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("update")
                .about("Update an existing website")
                .arg(arg!(<SSH_USER> "the ssh user"))
                .arg(arg!(<SSH_PASWWORD> "the ssh password"))
                .arg(arg!(<WEBSITE_FOLDER_PATH> "the folder of the website to use for the update"))
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("delete")
                .about("Delete an existing website")
                .arg(arg!(<SSH_USER> "the ssh user"))
                .arg(arg!(<SSH_PASWWORD> "the ssh password"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("deploy", sub_matches)) => {
            let ssh_user = sub_matches.get_one::<String>("SSH_USER");
            let ssh_password = sub_matches.get_one::<String>("SSH_PASSWORD");
            let website_url = sub_matches.get_one::<String>("WEBSITE_URL");
            let website_folder_path = sub_matches.get_one::<String>("WEBSITE_FOLDER_PATH");
        }
        Some(("deploy", sub_matches)) => {
            let ssh_user = sub_matches.get_one::<String>("SSH_USER");
            let ssh_password = sub_matches.get_one::<String>("SSH_PASSWORD");
            let website_folder_path = sub_matches.get_one::<String>("WEBSITE_FOLDER_PATH");
        }
        Some(("delete", sub_matches)) => {
            let ssh_user = sub_matches.get_one::<String>("SSH_USER");
            let ssh_password = sub_matches.get_one::<String>("SSH_PASSWORD");
        }
        _ => unreachable!()
    }
}
