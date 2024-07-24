use ssh2::Session;
use std::fs::{self, File};
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the SSH session
    let tcp = TcpStream::connect("example.com:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // Authenticate with the server
    sess.userauth_password("username", "password")?;
    assert!(sess.authenticated());

    // Initialize SFTP session
    let sftp = sess.sftp()?;

    // Define the local directory and the remote directory
    let local_dir = "./local_folder";
    let remote_dir = "/remote_folder";

    // Recursively upload the folder
    upload_folder(&sftp, Path::new(local_dir), remote_dir)?;

    Ok(())
}

fn upload_folder(sftp: &ssh2::Sftp, local_path: &Path, remote_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create the remote directory
    match sftp.mkdir(Path::new(remote_path), 0o755) {
        Ok(_) => println!("Created directory: {}", remote_path),
        Err(e) => println!("Directory already exists or failed to create: {} - {}", remote_path, e),
    }

    // Iterate over the entries in the local directory
    for entry in fs::read_dir(local_path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name().into_string().unwrap();
        let remote_file_path = format!("{}/{}", remote_path, file_name);

        if path.is_dir() {
            // Recursively upload directories
            upload_folder(sftp, &path, &remote_file_path)?;
        } else {
            // Upload files
            upload_file(sftp, &path, &remote_file_path)?;
        }
    }

    Ok(())
}

fn upload_file(sftp: &ssh2::Sftp, local_file: &Path, remote_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut local_f = File::open(local_file)?;
    let mut buffer = Vec::new();
    local_f.read_to_end(&mut buffer)?;

    let mut remote_f = sftp.create(Path::new(remote_file))?;
    remote_f.write_all(&buffer)?;

    println!("Uploaded file: {}", remote_file);

    Ok(())
}
