use thiserror::Error;

#[derive(Error, Debug)]
pub enum RumiError {
    #[error("SSH connection failed: {0}")]
    SshConnection(String),

    #[error("SSH authentication failed: {0}")]
    SshAuthentication(String),

    #[error("Command execution failed: {0}")]
    CommandExecution(String),

    #[error("File operation failed: {0}")]
    FileOperation(String),

    #[error("Network operation failed: {0}")]
    Network(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Deployment failed: {0}")]
    Deployment(String),

    #[error("Certificate operation failed: {0}")]
    Certificate(String),

    #[error("Nginx configuration failed: {0}")]
    Nginx(String),

    #[error("Firewall configuration failed: {0}")]
    Firewall(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("SSH2 error: {0}")]
    Ssh2(#[from] ssh2::Error),
}

pub type Result<T> = std::result::Result<T, RumiError>;

impl RumiError {
    pub fn ssh_connection<T: ToString>(msg: T) -> Self {
        Self::SshConnection(msg.to_string())
    }

    pub fn ssh_authentication<T: ToString>(msg: T) -> Self {
        Self::SshAuthentication(msg.to_string())
    }

    pub fn command_execution<T: ToString>(msg: T) -> Self {
        Self::CommandExecution(msg.to_string())
    }

    pub fn file_operation<T: ToString>(msg: T) -> Self {
        Self::FileOperation(msg.to_string())
    }

    pub fn network<T: ToString>(msg: T) -> Self {
        Self::Network(msg.to_string())
    }

    pub fn configuration<T: ToString>(msg: T) -> Self {
        Self::Configuration(msg.to_string())
    }

    pub fn invalid_input<T: ToString>(msg: T) -> Self {
        Self::InvalidInput(msg.to_string())
    }

    pub fn deployment<T: ToString>(msg: T) -> Self {
        Self::Deployment(msg.to_string())
    }

    pub fn certificate<T: ToString>(msg: T) -> Self {
        Self::Certificate(msg.to_string())
    }

    pub fn nginx<T: ToString>(msg: T) -> Self {
        Self::Nginx(msg.to_string())
    }

    pub fn firewall<T: ToString>(msg: T) -> Self {
        Self::Firewall(msg.to_string())
    }
}