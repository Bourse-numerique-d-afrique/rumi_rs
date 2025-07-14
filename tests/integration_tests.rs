use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rumi2 cli to help publish new website"))
        .stdout(predicate::str::contains("hosting"))
        .stdout(predicate::str::contains("server"))
        .stdout(predicate::str::contains("backup"))
        .stdout(predicate::str::contains("config"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.arg("--version");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rumi2 1.0"));
}

#[test]
fn test_hosting_subcommand_help() {
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["hosting", "--help"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Manage website hosting"))
        .stdout(predicate::str::contains("install"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("rollback"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_server_subcommand_help() {
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["server", "--help"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Manage server deployments"))
        .stdout(predicate::str::contains("deploy"))
        .stdout(predicate::str::contains("start"))
        .stdout(predicate::str::contains("stop"))
        .stdout(predicate::str::contains("restart"))
        .stdout(predicate::str::contains("status"));
}

#[test]
fn test_backup_subcommand_help() {
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["backup", "--help"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Manage backups"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("restore"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("cleanup"));
}

#[test]
fn test_config_subcommand_help() {
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["config", "--help"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Manage configurations"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("add-ssh"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("validate"));
}

#[test]
fn test_config_init() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");
    
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Configuration initialized"));
    
    assert!(config_path.exists());
}

#[test]
fn test_config_init_and_show() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");
    
    // Initialize config
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    cmd.assert().success();
    
    // Show config
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "show"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Current configuration"))
        .stdout(predicate::str::contains("deployments"))
        .stdout(predicate::str::contains("settings"));
}

#[test]
fn test_config_validate_default() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");
    
    // Initialize config
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    cmd.assert().success();
    
    // Validate config
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "validate"]);
    
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Configuration is valid"));
}

#[test]
fn test_dry_run_mode() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");
    
    // Initialize config first
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    cmd.assert().success();
    
    // Test dry run with hosting list (should work even without deployments)
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args([
        "--config", config_path.to_str().unwrap(),
        "--dry-run",
        "hosting", "list"
    ]);
    
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Running in dry-run mode"));
}

#[test]
fn test_verbose_mode() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");
    
    // Test verbose mode
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args([
        "--config", config_path.to_str().unwrap(),
        "--verbose",
        "config", "init"
    ]);
    
    cmd.assert().success();
}

#[test]
fn test_invalid_subcommand() {
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.arg("invalid-command");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

#[test]
fn test_missing_required_args() {
    // Test hosting install without required arguments
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["hosting", "install"]);
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_config_add_ssh() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");
    
    // Initialize config first
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    cmd.assert().success();
    
    // Add SSH configuration
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args([
        "--config", config_path.to_str().unwrap(),
        "config", "add-ssh",
        "--name", "test-server",
        "--host", "example.com",
        "--user", "testuser",
        "--port", "22"
    ]);
    
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("SSH configuration added"));
    
    // Verify configuration was added
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "show"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("example.com"))
        .stdout(predicate::str::contains("testuser"));
}

#[test]
fn test_backup_list_empty() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");
    
    // Initialize config
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    cmd.assert().success();
    
    // List backups (should handle gracefully when no deployments exist)
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "backup", "list"]);
    
    // This might fail due to no SSH configuration, which is expected
    // We're just testing that the command structure is correct
    cmd.assert().failure();
}

#[test]
fn test_hosting_list_empty() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");
    
    // Initialize config
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    cmd.assert().success();
    
    // List hosting deployments
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "hosting", "list"]);
    
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Listing deployments"));
}

// Error handling tests
#[test]
fn test_nonexistent_config_file() {
    let nonexistent_path = "/tmp/nonexistent_dir/config.json";
    
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", nonexistent_path, "config", "show"]);
    
    // Should create a default config if it doesn't exist
    cmd.assert().success();
}

#[test]
fn test_ethereum_install_help() {
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["ethereum", "install", "--help"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Install Ethereum node"))
        .stdout(predicate::str::contains("network-id"))
        .stdout(predicate::str::contains("http-address"))
        .stdout(predicate::str::contains("ws-address"))
        .stdout(predicate::str::contains("wallet-address"));
}

// Performance tests for CLI
#[test]
fn test_cli_response_time() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
    
    let duration = start.elapsed();
    
    // CLI should respond quickly (less than 1 second)
    assert!(duration.as_millis() < 1000, "CLI took too long to respond: {:?}", duration);
}

#[test]
fn test_config_operations_performance() {
    use std::time::Instant;
    
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");
    
    let start = Instant::now();
    
    // Initialize config
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "init"]);
    cmd.assert().success();
    
    // Show config
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "show"]);
    cmd.assert().success();
    
    // Validate config
    let mut cmd = Command::cargo_bin("rumi2").unwrap();
    cmd.args(["--config", config_path.to_str().unwrap(), "config", "validate"]);
    cmd.assert().success();
    
    let duration = start.elapsed();
    
    // All config operations should complete quickly
    assert!(duration.as_millis() < 2000, "Config operations took too long: {:?}", duration);
}