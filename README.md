# Rumi2 🚀

**A robust CLI tool for deploying web applications, servers, and blockchain nodes with seamless SSH2 integration.**

Rumi2 simplifies the deployment process to your existing server infrastructure, ensuring secure and efficient transfers with comprehensive backup, rollback, and monitoring capabilities.

<p align="center">
<img src="https://github.com/Bourse-numerique-d-afrique/rumi_rs/blob/main/assets/rumi_2_logo.jpg" alt="Rumi2 logo">
</p>

[![Build Status](https://img.shields.io/github/workflow/status/Bourse-numerique-d-afrique/rumi_rs/CI)](https://github.com/Bourse-numerique-d-afrique/rumi_rs/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/rumi2.svg)](https://crates.io/crates/rumi2)

## ✨ Features

### 🌐 **Website Hosting**
- **One-command deployments** with automatic SSL certificate generation
- **Zero-downtime updates** with nginx reload
- **Automatic backups** before every deployment
- **Easy rollbacks** to previous versions
- **Custom domain management** with automatic nginx configuration

### 🖥️ **Server Management**
- **Binary deployment** with automatic service management
- **Start/stop/restart** server applications
- **Health monitoring** and status checking
- **Port management** and firewall configuration
- **Process lifecycle management**

### ⛓️ **Ethereum Node Deployment**
- **Full Ethereum node setup** with geth
- **Custom network configuration** with genesis files
- **Automatic wallet creation** and management
- **RPC and WebSocket endpoint configuration**
- **Nginx proxy setup** for secure access

### 💾 **Backup & Recovery**
- **Automatic backups** before deployments
- **Incremental backup strategies** with compression
- **Easy restoration** from any backup point
- **Cleanup policies** with configurable retention
- **Metadata tracking** for backup management

### ⚙️ **Configuration Management**
- **JSON-based configuration** with validation
- **Multiple deployment profiles** support
- **SSH connection management** with key-based auth
- **Environment-specific settings**
- **Configuration validation** and error checking

### 🛡️ **Security & Safety**
- **Dry-run mode** for safe testing
- **SSH key authentication** with fallback options
- **Secure credential handling** (no hardcoded secrets)
- **Firewall management** with ufw integration
- **SSL/TLS certificate automation** with Let's Encrypt

## 🚀 Quick Start

### Installation

```bash
# Install from crates.io (coming soon)
cargo install rumi2

# Or clone and build from source
git clone https://github.com/Bourse-numerique-d-afrique/rumi_rs.git
cd rumi_rs
cargo build --release
```

### Basic Setup

```bash
# Initialize configuration
rumi2 config init

# Add SSH connection details
rumi2 config add-ssh \
  --name production \
  --host your-server.com \
  --user deploy \
  --port 22 \
  --public-key ~/.ssh/id_rsa.pub \
  --private-key ~/.ssh/id_rsa

# Verify configuration
rumi2 config validate
```

### Deploy Your First Website

```bash
# Deploy a website
rumi2 hosting install \
  --name my-website \
  --domain example.com \
  --dist-path ./dist

# Update your website
rumi2 hosting update \
  --name my-website \
  --dist-path ./new-dist

# List all deployments
rumi2 hosting list
```

### Deploy a Server Application

```bash
# Deploy a server binary
rumi2 server deploy \
  --name my-api \
  --domain api.example.com \
  --binary-path ./target/release/my-app \
  --port 8080

# Manage the server
rumi2 server start --name my-api
rumi2 server status --name my-api
rumi2 server restart --name my-api
```

## 📖 Usage Guide

### Configuration File

Rumi2 uses a JSON configuration file located at `~/.config/rumi/rumi.json`:

```json
{
  "default_ssh": {
    "host": "your-server.com",
    "user": "deploy",
    "port": 22,
    "public_key_path": "~/.ssh/id_rsa.pub",
    "private_key_path": "~/.ssh/id_rsa"
  },
  "deployments": [
    {
      "name": "my-website",
      "domain": "example.com",
      "deployment_type": "Website",
      "dist_path": "./dist",
      "backup_count": 5
    }
  ],
  "settings": {
    "log_level": "info",
    "backup_retention_days": 30,
    "ssl_email": "admin@example.com",
    "dry_run": false
  }
}
```

### Command Reference

#### Configuration Commands
```bash
rumi2 config init                    # Initialize configuration
rumi2 config show                    # Display current configuration
rumi2 config validate                # Validate configuration
rumi2 config add-ssh [OPTIONS]       # Add SSH configuration
```

#### Website Hosting Commands
```bash
rumi2 hosting install [OPTIONS]      # Install new website
rumi2 hosting update [OPTIONS]       # Update existing website
rumi2 hosting rollback [OPTIONS]     # Rollback to previous version
rumi2 hosting list                   # List all website deployments
```

#### Server Management Commands
```bash
rumi2 server deploy [OPTIONS]        # Deploy server application
rumi2 server start --name <NAME>     # Start server
rumi2 server stop --name <NAME>      # Stop server
rumi2 server restart --name <NAME>   # Restart server
rumi2 server status --name <NAME>    # Check server status
```

#### Backup Management Commands
```bash
rumi2 backup create --name <NAME>    # Create backup
rumi2 backup list [--name <NAME>]    # List backups
rumi2 backup restore [OPTIONS]       # Restore from backup
rumi2 backup delete --backup-id <ID> # Delete specific backup
rumi2 backup cleanup [OPTIONS]       # Clean up old backups
```

#### Ethereum Node Commands
```bash
rumi2 ethereum install [OPTIONS]     # Install Ethereum node
```

### Global Options

```bash
--config <FILE>     # Use custom configuration file
--verbose           # Enable verbose logging
--dry-run           # Preview changes without executing
--help              # Show help information
--version           # Show version information
```

## 🔧 Advanced Features

### Dry Run Mode

Test your deployments safely without making actual changes:

```bash
rumi2 --dry-run hosting install --name test --domain test.com --dist-path ./dist
```

### Backup Management

Automatic backups are created before every deployment:

```bash
# List all backups
rumi2 backup list

# Restore specific backup
rumi2 backup restore --backup-id abc123-def456

# Clean up old backups (older than 30 days)
rumi2 backup cleanup --retention-days 30
```

### Multiple Environments

Manage different environments with separate config files:

```bash
# Production environment
rumi2 --config production.json hosting update --name app

# Staging environment  
rumi2 --config staging.json hosting update --name app
```

## 🛠️ Development

### Prerequisites

- Rust 1.70+ 
- OpenSSL development libraries
- SSH access to target servers

### Building from Source

```bash
git clone https://github.com/Bourse-numerique-d-afrique/rumi_rs.git
cd rumi_rs
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test config_tests
cargo test integration_tests

# Run with verbose output
cargo test -- --nocapture
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📋 Requirements

### System Requirements

- **Operating System**: Linux, macOS, Windows (WSL)
- **Memory**: 512MB RAM minimum
- **Storage**: 100MB available space
- **Network**: SSH access to target servers

### Server Requirements

- **SSH Server**: OpenSSH 7.0+
- **Operating System**: Ubuntu 18.04+, Debian 9+, CentOS 7+
- **Privileges**: sudo access for system configuration
- **Ports**: 22 (SSH), 80 (HTTP), 443 (HTTPS)

### Dependencies

The following packages will be automatically installed on the target server:

- `nginx` - Web server and reverse proxy
- `certbot` - SSL certificate management
- `ufw` - Uncomplicated Firewall
- `geth` - Ethereum client (for blockchain deployments)

## 🔒 Security

### Best Practices

- **Use SSH keys** instead of passwords when possible
- **Regularly rotate** SSH keys and certificates
- **Enable firewall** rules for production servers
- **Monitor deployment logs** for suspicious activity
- **Keep backups** in secure, separate locations

### Security Features

- No hardcoded credentials or secrets
- SSH key-based authentication with fallback options
- Automatic SSL certificate generation and renewal
- Firewall management with sensible defaults
- Secure file transfer with integrity checking

## 📚 Examples

### Complete Website Deployment

```bash
# 1. Initialize and configure
rumi2 config init
rumi2 config add-ssh --name prod --host server.com --user deploy

# 2. Deploy website
rumi2 hosting install \
  --name my-blog \
  --domain blog.example.com \
  --dist-path ./build

# 3. Update with new version
rumi2 hosting update --name my-blog --dist-path ./new-build

# 4. Check backups
rumi2 backup list --name my-blog

# 5. Rollback if needed
rumi2 backup restore --backup-id latest-backup-id
```

### API Server Deployment

```bash
# Deploy Go API server
rumi2 server deploy \
  --name user-api \
  --domain api.myapp.com \
  --binary-path ./target/release/user-api \
  --port 8080

# Monitor server status
rumi2 server status --name user-api
```

### Ethereum Node Setup

```bash
# Deploy private Ethereum network
rumi2 ethereum install \
  --name private-eth \
  --domain eth.mycompany.com \
  --network-id 1337 \
  --http-address "0.0.0.0" \
  --ws-address "0.0.0.0" \
  --external-ip "203.0.113.1" \
  --wallet-address "0x742d35Cc6634C0532925a3b8D80B11363DC2C7CC"
```

## 🎯 Roadmap

- [ ] **Docker deployment support**
- [ ] **Kubernetes integration**
- [ ] **Database migration management**
- [ ] **Multi-region deployments**
- [ ] **Integration with CI/CD pipelines**
- [ ] **Web dashboard interface**
- [ ] **Slack/Discord notifications**
- [ ] **Prometheus metrics export**

## 🤝 Support

### Getting Help

- **Documentation**: [docs.rs/rumi2](https://docs.rs/rumi2)
- **Issues**: [GitHub Issues](https://github.com/Bourse-numerique-d-afrique/rumi_rs/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Bourse-numerique-d-afrique/rumi_rs/discussions)

### Common Issues

**SSH Connection Failed**
```bash
# Check SSH configuration
rumi2 config show
# Test SSH connection manually
ssh -i ~/.ssh/id_rsa user@server.com
```

**Permission Denied**
```bash
# Ensure user has sudo privileges
# Check file permissions on SSH keys
chmod 600 ~/.ssh/id_rsa
chmod 644 ~/.ssh/id_rsa.pub
```

**Deployment Failed**
```bash
# Use dry-run mode to check configuration
rumi2 --dry-run hosting install --name test --domain test.com --dist-path ./dist
# Check logs with verbose mode
rumi2 --verbose hosting install --name test --domain test.com --dist-path ./dist
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👥 Authors

- **ONDONDA Prince Merveil** - [@princefr](https://github.com/princefr)
- **Bourse Numerique D'Afrique** - [dev@boursenumeriquedafrique.com](mailto:dev@boursenumeriquedafrique.com)

## 🙏 Acknowledgments

- The Rust community for excellent tooling and libraries
- OpenSSH project for secure remote access
- Let's Encrypt for free SSL certificates
- Nginx team for robust web server technology
- All contributors and users of this project

---

**Made with ❤️ by Bourse Numerique D'Afrique**