use clap::{arg, Command};
use std::io::Error;

fn cli() -> Command {
    Command::new("run")
        .about("Rumi2 cli to help publish new website to a server via ssh")
        .author("Bourse Numerique D'Afrique <dev@boursenumeriquedafrique.com>")
        .version("1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("hosting")
                .about("Manage the hosting lifcycle of you website")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("install")
                        .about("Install a website on a new server using a ssh connexion")
                        .arg(arg!(--ssh_cert_public_key <SSH_CERT_PUBLIC_KEY> "the ssh public key"))
                        .arg(arg!(--ssh_cert_private_key <SSH_CERT_PRIVATE_KEY> "the ssh private key"))
                        .arg(arg!(--ssh_host <SSH_HOST> "the ssh host"))
                        .arg(arg!(--ssh_user <SSH_USER> "the ssh user"))
                        .arg(arg!(--ssh_password <SSH_PASSWORD> "the ssh password"))
                        .arg(arg!(--domain <DOMAIN> "the url of the website"))
                        .arg(arg!(--dist_path <DIST_PATH> "the url of the website"))
                        .arg(arg!(--version_id <VERSION_ID> "the version id"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("update")
                        .about(
                            "Update an existing website running on a server using a ssh connexion",
                        )
                        .arg(arg!(--ssh_cert_public_key <SSH_CERT_PUBLIC_KEY> "the ssh public key"))
                        .arg(arg!(--ssh_cert_private_key <SSH_CERT_PRIVATE_KEY> "the ssh private key"))
                        .arg(arg!(--ssh_host <SSH_HOST> "the ssh host"))
                        .arg(arg!(--ssh_user <SSH_USER> "the ssh user"))
                        .arg(arg!(--ssh_password <SSH_PASSWORD> "the ssh password"))
                        .arg(arg!(--domain <DOMAIN> "the url of the website"))
                        .arg(arg!(--dist_path <DIST_PATH> "the url of the website"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("rollback")
                        .about("Rollback to a former website version")
                        .arg(arg!(--ssh_cert_public_key <SSH_CERT_PUBLIC_KEY> "the ssh public key"))
                        .arg(arg!(--ssh_cert_private_key <SSH_CERT_PRIVATE_KEY> "the ssh private key"))
                        .arg(arg!(--ssh_host <SSH_HOST> "the ssh host"))
                        .arg(arg!(--ssh_user <SSH_USER> "the ssh user"))
                        .arg(arg!(--ssh_password <SSH_PASSWORD> "the ssh password"))
                        .arg(arg!(--domain <DOMAIN> "the url of the website"))
                        .arg(arg!(--version_id <VERSION_ID> "the url of the website"))
                        .arg_required_else_help(true),
                ),
        )
}

fn main() -> Result<(), Error> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("hosting", hosting_matches)) => match hosting_matches.subcommand() {
            Some(("install", install_matches)) => {
                use rumi2::commands::websites::install_command;

                let ssh_cert_public_key = install_matches
                    .get_one::<String>("ssh_cert_public_key")
                    .map(|s| s.as_str())
                    .expect("SSH_CERT_PUBLIC_KEY parameter value is missing");
                let ssh_cert_private_key = install_matches
                    .get_one::<String>("ssh_cert_private_key")
                    .map(|s| s.as_str())
                    .expect("SSH_CERT_PRIVATE_KEY parameter value is missing");
                let ssh_host = install_matches
                    .get_one::<String>("ssh_host")
                    .map(|s| s.as_str())
                    .expect("SSH_HOST parameter value is missing");
                let ssh_user = install_matches
                    .get_one::<String>("ssh_user")
                    .map(|s| s.as_str())
                    .expect("SSH_USER parameter value is missing");
                let ssh_password = install_matches
                    .get_one::<String>("ssh_password")
                    .map(|s| s.as_str())
                    .expect("SSH_PASSWORD parameter value is missing");
                let domain = install_matches
                    .get_one::<String>("domain")
                    .map(|s| s.as_str())
                    .expect("DOMAIN parameter value is missing");
                let dist_path = install_matches
                    .get_one::<String>("dist_path")
                    .map(|s| s.as_str())
                    .expect("DIST_PATH parameter value is missing");

                let version_id = install_matches
                    .get_one::<String>("version_id")
                    .map(|s| s.as_str())
                    .expect("VERSION_ID paramer value is missing");

                let session = rumi2::Rumi2::start(
                    ssh_host.to_string(),
                    ssh_user.to_string(),
                    ssh_cert_public_key.to_string(),
                    ssh_cert_private_key.to_string(),
                    ssh_password.to_string(),
                );
                install_command(&session, domain, dist_path);
            }

            Some(("update", update_matches)) => {
                use rumi2::commands::websites::update_command;

                let ssh_cert_public_key = update_matches
                    .get_one::<String>("ssh_cert_public_key")
                    .map(|s| s.as_str())
                    .expect("SSH_CERT_PUBLIC_KEY parameter value is missing");
                let ssh_cert_private_key = update_matches
                    .get_one::<String>("ssh_cert_private_key")
                    .map(|s| s.as_str())
                    .expect("SSH_CERT_PRIVATE_KEY parameter value is missing");
                let ssh_host = update_matches
                    .get_one::<String>("ssh_host")
                    .map(|s| s.as_str())
                    .expect("SSH_HOST parameter value is missing");
                let ssh_user = update_matches
                    .get_one::<String>("ssh_user")
                    .map(|s| s.as_str())
                    .expect("SSH_USER parameter value is missing");
                let ssh_password = update_matches
                    .get_one::<String>("ssh_password")
                    .map(|s| s.as_str())
                    .expect("SSH_PASSWORD parameter value is missing");
                let domain = update_matches
                    .get_one::<String>("domain")
                    .map(|s| s.as_str())
                    .expect("DOMAIN parameter value is missing");
                let dist_path = update_matches
                    .get_one::<String>("dist_path")
                    .map(|s| s.as_str())
                    .expect("DIST_PATH parameter value is missing");

                let session = rumi2::Rumi2::start(
                    ssh_host.to_string(),
                    ssh_user.to_string(),
                    ssh_cert_public_key.to_string(),
                    ssh_cert_private_key.to_string(),
                    ssh_password.to_string(),
                );
                update_command(&session, domain, dist_path)
            }

            Some(("rollback", rollback_matches)) => {
                use rumi2::commands::websites::rollback_command;

                let ssh_cert_public_key = rollback_matches
                    .get_one::<String>("ssh_cert_public_key")
                    .map(|s| s.as_str())
                    .expect("SSH_CERT_PUBLIC_KEY parameter value is missing");
                let ssh_cert_private_key = rollback_matches
                    .get_one::<String>("ssh_cert_private_key")
                    .map(|s| s.as_str())
                    .expect("SSH_CERT_PRIVATE_KEY parameter value is missing");
                let ssh_host = rollback_matches
                    .get_one::<String>("ssh_host")
                    .map(|s| s.as_str())
                    .expect("SSH_HOST parameter value is missing");
                let ssh_user = rollback_matches
                    .get_one::<String>("ssh_user")
                    .map(|s| s.as_str())
                    .expect("SSH_USER parameter value is missing");
                let ssh_password = rollback_matches
                    .get_one::<String>("ssh_password")
                    .map(|s| s.as_str())
                    .expect("SSH_PASSWORD parameter value is missing");
                let domain = rollback_matches
                    .get_one::<String>("domain")
                    .map(|s| s.as_str())
                    .expect("DOMAIN parameter value is missing");
                let version_id = rollback_matches
                    .get_one::<String>("version_id")
                    .map(|s| s.as_str())
                    .expect("VERSION_ID parameter value is missing");

                let session = rumi2::Rumi2::start(
                    ssh_host.to_string(),
                    ssh_user.to_string(),
                    ssh_cert_public_key.to_string(),
                    ssh_cert_private_key.to_string(),
                    ssh_password.to_string(),
                );
                rollback_command(&session, domain, version_id);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    Ok(())
}
