//! # Error Types and Handling
//!
//! This module defines the error types used throughout the Rumi2 application.
//! All errors implement the standard [`std::error::Error`] trait and provide
//! comprehensive error information for debugging and user feedback.
//!
//! ## Error Categories
//!
//! - **SSH Errors**: Connection, authentication, and command execution failures
//! - **File Operations**: Upload, download, and file system errors  
//! - **Network Operations**: Network-related failures
//! - **Configuration**: Configuration validation and parsing errors
//! - **Deployment**: High-level deployment operation failures
//!
//! ## Examples
//!
//! ```rust
//! use rumi2::error::{RumiError, Result};
//!
//! fn example_function() -> Result<String> {
//!     // This would return an error
//!     Err(RumiError::ssh_connection("Failed to connect to server"))
//! }
//!
//! match example_function() {
//!     Ok(result) => println!("Success: {}", result),
//!     Err(RumiError::SshConnection(msg)) => eprintln!("SSH Error: {}", msg),
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! ```

use thiserror::Error;

/// Comprehensive error type for all Rumi2 operations
///
/// This enum covers all possible error conditions that can occur during
/// deployment operations, from low-level SSH failures to high-level
/// deployment orchestration errors.
#[derive(Error, Debug)]
pub enum RumiError {
    /// SSH connection establishment failed
    ///
    /// This error occurs when the initial TCP connection to the SSH server
    /// cannot be established, often due to network issues or wrong host/port.
    #[error("SSH connection failed: {0}")]
    SshConnection(String),

    /// SSH authentication failed
    ///
    /// Authentication using the provided credentials (keys, password, or agent)
    /// was rejected by the SSH server.
    #[error("SSH authentication failed: {0}")]
    SshAuthentication(String),

    /// Remote command execution failed
    ///
    /// A command was executed on the remote server but returned a non-zero
    /// exit code or encountered an execution error.
    #[error("Command execution failed: {0}")]
    CommandExecution(String),

    /// File operation failed
    ///
    /// File upload, download, creation, or other file system operations
    /// encountered an error on either local or remote systems.
    #[error("File operation failed: {0}")]
    FileOperation(String),

    /// Network operation failed
    ///
    /// General network-related errors that don't fall under SSH connection issues.
    #[error("Network operation failed: {0}")]
    Network(String),

    /// Configuration validation or parsing error
    ///
    /// The configuration file contains invalid data, missing required fields,
    /// or fails validation checks.
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Invalid user input
    ///
    /// User-provided arguments or input values are invalid or don't meet
    /// the required format or constraints.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// High-level deployment operation failed
    ///
    /// A deployment workflow failed, which may be due to multiple underlying
    /// issues during the deployment process.
    #[error("Deployment failed: {0}")]
    Deployment(String),

    /// SSL/TLS certificate operation failed
    ///
    /// Errors related to SSL certificate generation, installation, or management,
    /// typically with Let's Encrypt operations.
    #[error("Certificate operation failed: {0}")]
    Certificate(String),

    /// Nginx configuration failed
    ///
    /// Errors related to nginx configuration file generation, installation,
    /// or nginx service management.
    #[error("Nginx configuration failed: {0}")]
    Nginx(String),

    /// Firewall configuration failed
    ///
    /// Errors related to UFW (Uncomplicated Firewall) configuration and
    /// port management operations.
    #[error("Firewall configuration failed: {0}")]
    Firewall(String),

    /// Standard I/O error
    ///
    /// Wraps [`std::io::Error`] for file system and I/O operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    ///
    /// Wraps [`serde_json::Error`] for configuration file parsing and generation.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// SSH2 library error
    ///
    /// Wraps [`ssh2::Error`] for low-level SSH protocol operations.
    #[error("SSH2 error: {0}")]
    Ssh2(#[from] ssh2::Error),
}

/// Type alias for [`std::result::Result`] with [`RumiError`] as the error type
///
/// This simplifies function signatures throughout the codebase by providing
/// a consistent error type for all Rumi2 operations.
///
/// # Examples
///
/// ```rust
/// use rumi2::error::Result;
///
/// fn deploy_website() -> Result<()> {
///     // Deployment logic here
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, RumiError>;

impl RumiError {
    /// Creates a new SSH connection error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rumi2::error::RumiError;
    ///
    /// let error = RumiError::ssh_connection("Connection refused");
    /// ```
    pub fn ssh_connection<T: ToString>(msg: T) -> Self {
        Self::SshConnection(msg.to_string())
    }

    /// Creates a new SSH authentication error
    pub fn ssh_authentication<T: ToString>(msg: T) -> Self {
        Self::SshAuthentication(msg.to_string())
    }

    /// Creates a new command execution error
    pub fn command_execution<T: ToString>(msg: T) -> Self {
        Self::CommandExecution(msg.to_string())
    }

    /// Creates a new file operation error
    pub fn file_operation<T: ToString>(msg: T) -> Self {
        Self::FileOperation(msg.to_string())
    }

    /// Creates a new network operation error
    pub fn network<T: ToString>(msg: T) -> Self {
        Self::Network(msg.to_string())
    }

    /// Creates a new configuration error
    pub fn configuration<T: ToString>(msg: T) -> Self {
        Self::Configuration(msg.to_string())
    }

    /// Creates a new invalid input error
    pub fn invalid_input<T: ToString>(msg: T) -> Self {
        Self::InvalidInput(msg.to_string())
    }

    /// Creates a new deployment error
    pub fn deployment<T: ToString>(msg: T) -> Self {
        Self::Deployment(msg.to_string())
    }

    /// Creates a new certificate operation error
    pub fn certificate<T: ToString>(msg: T) -> Self {
        Self::Certificate(msg.to_string())
    }

    /// Creates a new nginx configuration error
    pub fn nginx<T: ToString>(msg: T) -> Self {
        Self::Nginx(msg.to_string())
    }

    /// Creates a new firewall configuration error
    pub fn firewall<T: ToString>(msg: T) -> Self {
        Self::Firewall(msg.to_string())
    }
}