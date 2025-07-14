use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use std::time::Duration;
use crate::config::SshConfig;
use crate::error::{Result, RumiError};

pub struct RumiSession {
    pub session: Session,
    config: SshConfig,
}

impl RumiSession {
    pub fn new(config: SshConfig) -> Result<Self> {
        let host = format!("{}:{}", config.host, config.port.unwrap_or(22));
        
        log::info!("Connecting to {}", host);
        
        let tcp = TcpStream::connect(&host)
            .map_err(|e| RumiError::ssh_connection(format!("Failed to connect to {}: {}", host, e)))?;
        
        // Set timeout
        tcp.set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| RumiError::ssh_connection(format!("Failed to set read timeout: {}", e)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| RumiError::ssh_connection(format!("Failed to set write timeout: {}", e)))?;
        
        let mut session = Session::new()
            .map_err(|e| RumiError::ssh_connection(format!("Failed to create SSH session: {}", e)))?;
        
        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| RumiError::ssh_connection(format!("SSH handshake failed: {}", e)))?;
        
        // Authenticate
        Self::authenticate(&mut session, &config)?;
        
        log::info!("Successfully connected to {}", config.host);
        
        Ok(Self { session, config })
    }

    fn authenticate(session: &mut Session, config: &SshConfig) -> Result<()> {
        let username = &config.user;
        
        // Try public key authentication first if keys are provided
        if let (Some(public_key_path), Some(private_key_path)) = (&config.public_key_path, &config.private_key_path) {
            log::debug!("Attempting public key authentication");
            
            if public_key_path.exists() && private_key_path.exists() {
                let passphrase = config.password.as_deref();
                
                session.userauth_pubkey_file(
                    username,
                    Some(public_key_path),
                    private_key_path,
                    passphrase
                ).map_err(|e| RumiError::ssh_authentication(format!("Public key authentication failed: {}", e)))?;
                
                if session.authenticated() {
                    log::info!("Public key authentication successful");
                    return Ok(());
                }
            } else {
                log::warn!("Public key files not found, falling back to password authentication");
            }
        }
        
        // Try password authentication
        if let Some(password) = &config.password {
            log::debug!("Attempting password authentication");
            
            session.userauth_password(username, password)
                .map_err(|e| RumiError::ssh_authentication(format!("Password authentication failed: {}", e)))?;
            
            if session.authenticated() {
                log::info!("Password authentication successful");
                return Ok(());
            }
        }
        
        // Try SSH agent
        log::debug!("Attempting SSH agent authentication");
        session.userauth_agent(username)
            .map_err(|e| RumiError::ssh_authentication(format!("SSH agent authentication failed: {}", e)))?;
        
        if session.authenticated() {
            log::info!("SSH agent authentication successful");
            return Ok(());
        }
        
        Err(RumiError::ssh_authentication("All authentication methods failed"))
    }

    pub fn execute_command(&self, command: &str) -> Result<CommandResult> {
        log::debug!("Executing command: {}", command);
        
        let mut channel = self.session.channel_session()
            .map_err(|e| RumiError::command_execution(format!("Failed to create channel: {}", e)))?;
        
        channel.exec(command)
            .map_err(|e| RumiError::command_execution(format!("Failed to execute command '{}': {}", command, e)))?;
        
        let mut stdout = String::new();
        channel.read_to_string(&mut stdout)
            .map_err(|e| RumiError::command_execution(format!("Failed to read command output: {}", e)))?;
        
        let mut stderr = String::new();
        channel.stderr().read_to_string(&mut stderr)
            .map_err(|e| RumiError::command_execution(format!("Failed to read command stderr: {}", e)))?;
        
        channel.wait_close()
            .map_err(|e| RumiError::command_execution(format!("Failed to close channel: {}", e)))?;
        
        let exit_status = channel.exit_status()
            .map_err(|e| RumiError::command_execution(format!("Failed to get exit status: {}", e)))?;
        
        let result = CommandResult {
            stdout,
            stderr,
            exit_status,
            command: command.to_string(),
        };
        
        log::debug!("Command '{}' completed with exit status: {}", command, exit_status);
        
        if exit_status != 0 {
            log::warn!("Command failed: {}", result.stderr);
        }
        
        Ok(result)
    }

    pub fn execute_command_checked(&self, command: &str) -> Result<CommandResult> {
        let result = self.execute_command(command)?;
        
        if result.exit_status != 0 {
            return Err(RumiError::command_execution(
                format!("Command '{}' failed with exit code {}: {}", 
                    command, result.exit_status, result.stderr)
            ));
        }
        
        Ok(result)
    }

    pub fn upload_file(&self, local_path: &Path, remote_path: &str) -> Result<()> {
        log::debug!("Uploading file from {:?} to {}", local_path, remote_path);
        
        let file = std::fs::File::open(local_path)
            .map_err(|e| RumiError::file_operation(format!("Failed to open local file {:?}: {}", local_path, e)))?;
        
        let metadata = file.metadata()
            .map_err(|e| RumiError::file_operation(format!("Failed to get file metadata: {}", e)))?;
        
        let mut remote_file = self.session.scp_send(Path::new(remote_path), 0o644, metadata.len(), None)
            .map_err(|e| RumiError::file_operation(format!("Failed to create remote file: {}", e)))?;
        
        std::io::copy(&mut std::fs::File::open(local_path)?, &mut remote_file)
            .map_err(|e| RumiError::file_operation(format!("Failed to copy file data: {}", e)))?;
        
        remote_file.send_eof()
            .map_err(|e| RumiError::file_operation(format!("Failed to send EOF: {}", e)))?;
        remote_file.wait_eof()
            .map_err(|e| RumiError::file_operation(format!("Failed to wait for EOF: {}", e)))?;
        remote_file.close()
            .map_err(|e| RumiError::file_operation(format!("Failed to close remote file: {}", e)))?;
        remote_file.wait_close()
            .map_err(|e| RumiError::file_operation(format!("Failed to wait for close: {}", e)))?;
        
        log::info!("Successfully uploaded file to {}", remote_path);
        Ok(())
    }

    pub fn upload_directory(&self, local_path: &Path, remote_path: &str) -> Result<()> {
        log::info!("Uploading directory from {:?} to {}", local_path, remote_path);
        
        let sftp = self.session.sftp()
            .map_err(|e| RumiError::file_operation(format!("Failed to create SFTP session: {}", e)))?;
        
        crate::utils::upload_folder(&sftp, local_path, remote_path)
            .map_err(|e| RumiError::file_operation(format!("Failed to upload directory: {}", e)))?;
        
        log::info!("Successfully uploaded directory to {}", remote_path);
        Ok(())
    }

    pub fn create_remote_file(&self, remote_path: &str, content: &str) -> Result<()> {
        log::debug!("Creating remote file: {}", remote_path);
        
        let sftp = self.session.sftp()
            .map_err(|e| RumiError::file_operation(format!("Failed to create SFTP session: {}", e)))?;
        
        let mut file = sftp.create(Path::new(remote_path))
            .map_err(|e| RumiError::file_operation(format!("Failed to create remote file: {}", e)))?;
        
        use std::io::Write;
        file.write_all(content.as_bytes())
            .map_err(|e| RumiError::file_operation(format!("Failed to write file content: {}", e)))?;
        
        log::debug!("Successfully created remote file: {}", remote_path);
        Ok(())
    }

    pub fn file_exists(&self, remote_path: &str) -> Result<bool> {
        let result = self.execute_command(&format!("test -f {}", remote_path));
        match result {
            Ok(cmd_result) => Ok(cmd_result.exit_status == 0),
            Err(_) => Ok(false),
        }
    }

    pub fn directory_exists(&self, remote_path: &str) -> Result<bool> {
        let result = self.execute_command(&format!("test -d {}", remote_path));
        match result {
            Ok(cmd_result) => Ok(cmd_result.exit_status == 0),
            Err(_) => Ok(false),
        }
    }

    pub fn get_config(&self) -> &SshConfig {
        &self.config
    }

    pub fn test_connection(&self) -> Result<()> {
        self.execute_command_checked("echo 'connection test'")?;
        log::info!("Connection test successful");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_status: i32,
    pub command: String,
}

impl CommandResult {
    pub fn is_success(&self) -> bool {
        self.exit_status == 0
    }

    pub fn output(&self) -> &str {
        &self.stdout
    }

    pub fn error(&self) -> &str {
        &self.stderr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_result() {
        let result = CommandResult {
            stdout: "test output".to_string(),
            stderr: "".to_string(),
            exit_status: 0,
            command: "echo test".to_string(),
        };
        
        assert!(result.is_success());
        assert_eq!(result.output(), "test output");
        assert_eq!(result.error(), "");
    }

    #[test]
    fn test_command_result_failure() {
        let result = CommandResult {
            stdout: "".to_string(),
            stderr: "command not found".to_string(),
            exit_status: 127,
            command: "nonexistent_command".to_string(),
        };
        
        assert!(!result.is_success());
        assert_eq!(result.error(), "command not found");
    }

    // Note: SSH connection tests would require a test server setup
    // For now, we'll focus on unit tests for the configuration and validation logic
}