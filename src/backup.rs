use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::error::{Result, RumiError};
use crate::session::RumiSession;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub deployment_name: String,
    pub domain: String,
    pub created_at: DateTime<Utc>,
    pub backup_type: BackupType,
    pub remote_path: String,
    pub size_bytes: u64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupType {
    Website,
    Server,
    Database,
    Configuration,
}

pub struct BackupManager {
    pub backup_base_path: String,
}

impl BackupManager {
    pub fn new(backup_base_path: String) -> Self {
        Self { backup_base_path }
    }

    pub fn create_website_backup(
        &self,
        session: &RumiSession,
        deployment_name: &str,
        domain: &str,
        website_path: &str,
    ) -> Result<BackupInfo> {
        let backup_id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let backup_name = format!("{}_{}_website_{}", deployment_name, domain, timestamp.format("%Y%m%d_%H%M%S"));
        let backup_path = format!("{}/{}", self.backup_base_path, backup_name);

        log::info!("Creating website backup for {} at {}", domain, website_path);

        // Create backup directory
        session.execute_command_checked(&format!("sudo mkdir -p {}", backup_path))?;

        // Create compressed backup
        let tar_command = format!(
            "sudo tar -czf {}/{}.tar.gz -C {} .",
            backup_path, backup_name, website_path
        );
        session.execute_command_checked(&tar_command)?;

        // Get backup size
        let size_result = session.execute_command_checked(&format!("stat -c%s {}/{}.tar.gz", backup_path, backup_name))?;
        let size_bytes = size_result.stdout.trim().parse::<u64>()
            .map_err(|e| RumiError::file_operation(format!("Failed to parse backup size: {}", e)))?;

        // Create backup metadata
        let backup_info = BackupInfo {
            id: backup_id,
            deployment_name: deployment_name.to_string(),
            domain: domain.to_string(),
            created_at: timestamp,
            backup_type: BackupType::Website,
            remote_path: format!("{}/{}.tar.gz", backup_path, backup_name),
            size_bytes,
            description: Some(format!("Website backup for {}", domain)),
        };

        // Save backup metadata
        self.save_backup_metadata(session, &backup_info)?;

        log::info!("Website backup created successfully: {}", backup_info.id);
        Ok(backup_info)
    }

    pub fn create_configuration_backup(
        &self,
        session: &RumiSession,
        deployment_name: &str,
        domain: &str,
    ) -> Result<BackupInfo> {
        let backup_id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let backup_name = format!("{}_{}_config_{}", deployment_name, domain, timestamp.format("%Y%m%d_%H%M%S"));
        let backup_path = format!("{}/{}", self.backup_base_path, backup_name);

        log::info!("Creating configuration backup for {}", domain);

        // Create backup directory
        session.execute_command_checked(&format!("sudo mkdir -p {}", backup_path))?;

        // Backup nginx configuration
        let nginx_backup_cmd = format!(
            "sudo cp -r /etc/nginx/sites-available/{} {}/nginx_config 2>/dev/null || true",
            domain, backup_path
        );
        session.execute_command(&nginx_backup_cmd)?;

        // Backup SSL certificates
        let ssl_backup_cmd = format!(
            "sudo cp -r /etc/letsencrypt/live/{} {}/ssl_certs 2>/dev/null || true",
            domain, backup_path
        );
        session.execute_command(&ssl_backup_cmd)?;

        // Create compressed backup
        let tar_command = format!(
            "sudo tar -czf {}/{}.tar.gz -C {} .",
            backup_path, backup_name, backup_path
        );
        session.execute_command_checked(&tar_command)?;

        // Clean up temporary files
        session.execute_command(&format!("sudo rm -rf {}/nginx_config {}/ssl_certs", backup_path, backup_path))?;

        // Get backup size
        let size_result = session.execute_command_checked(&format!("stat -c%s {}/{}.tar.gz", backup_path, backup_name))?;
        let size_bytes = size_result.stdout.trim().parse::<u64>()
            .map_err(|e| RumiError::file_operation(format!("Failed to parse backup size: {}", e)))?;

        let backup_info = BackupInfo {
            id: backup_id,
            deployment_name: deployment_name.to_string(),
            domain: domain.to_string(),
            created_at: timestamp,
            backup_type: BackupType::Configuration,
            remote_path: format!("{}/{}.tar.gz", backup_path, backup_name),
            size_bytes,
            description: Some(format!("Configuration backup for {}", domain)),
        };

        self.save_backup_metadata(session, &backup_info)?;

        log::info!("Configuration backup created successfully: {}", backup_info.id);
        Ok(backup_info)
    }

    pub fn restore_website_backup(
        &self,
        session: &RumiSession,
        backup_info: &BackupInfo,
        restore_path: &str,
    ) -> Result<()> {
        log::info!("Restoring website backup {} to {}", backup_info.id, restore_path);

        if !session.file_exists(&backup_info.remote_path)? {
            return Err(RumiError::file_operation(format!("Backup file not found: {}", backup_info.remote_path)));
        }

        // Create restore directory
        session.execute_command_checked(&format!("sudo mkdir -p {}", restore_path))?;

        // Extract backup
        let extract_command = format!(
            "sudo tar -xzf {} -C {}",
            backup_info.remote_path, restore_path
        );
        session.execute_command_checked(&extract_command)?;

        // Set proper permissions
        session.execute_command_checked(&format!("sudo chown -R www-data:www-data {}", restore_path))?;
        session.execute_command_checked(&format!("sudo chmod -R 755 {}", restore_path))?;

        log::info!("Website backup restored successfully to {}", restore_path);
        Ok(())
    }

    pub fn list_backups(&self, session: &RumiSession, deployment_name: Option<&str>) -> Result<Vec<BackupInfo>> {
        let metadata_path = format!("{}/metadata", self.backup_base_path);
        
        if !session.directory_exists(&metadata_path)? {
            return Ok(Vec::new());
        }

        let list_command = format!("find {} -name '*.json' -type f", metadata_path);
        let result = session.execute_command_checked(&list_command)?;

        let mut backups = Vec::new();
        
        for line in result.stdout.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let cat_command = format!("cat {}", line.trim());
            if let Ok(content_result) = session.execute_command(&cat_command) {
                if let Ok(backup_info) = serde_json::from_str::<BackupInfo>(&content_result.stdout) {
                    if let Some(filter_name) = deployment_name {
                        if backup_info.deployment_name == filter_name {
                            backups.push(backup_info);
                        }
                    } else {
                        backups.push(backup_info);
                    }
                }
            }
        }

        // Sort by creation date (newest first)
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(backups)
    }

    pub fn delete_backup(&self, session: &RumiSession, backup_id: &str) -> Result<()> {
        let backups = self.list_backups(session, None)?;
        
        let backup = backups.iter()
            .find(|b| b.id == backup_id)
            .ok_or_else(|| RumiError::file_operation(format!("Backup not found: {}", backup_id)))?;

        log::info!("Deleting backup: {}", backup_id);

        // Delete backup file
        if session.file_exists(&backup.remote_path)? {
            session.execute_command_checked(&format!("sudo rm -f {}", backup.remote_path))?;
        }

        // Delete metadata
        let metadata_path = format!("{}/metadata/{}.json", self.backup_base_path, backup_id);
        if session.file_exists(&metadata_path)? {
            session.execute_command_checked(&format!("sudo rm -f {}", metadata_path))?;
        }

        log::info!("Backup deleted successfully: {}", backup_id);
        Ok(())
    }

    pub fn cleanup_old_backups(&self, session: &RumiSession, retention_days: u32) -> Result<()> {
        let backups = self.list_backups(session, None)?;
        let cutoff_date = Utc::now() - chrono::Duration::days(retention_days as i64);

        log::info!("Cleaning up backups older than {} days", retention_days);

        let mut deleted_count = 0;
        for backup in backups {
            if backup.created_at < cutoff_date {
                if let Err(e) = self.delete_backup(session, &backup.id) {
                    log::warn!("Failed to delete old backup {}: {}", backup.id, e);
                } else {
                    deleted_count += 1;
                }
            }
        }

        log::info!("Cleaned up {} old backups", deleted_count);
        Ok(())
    }

    fn save_backup_metadata(&self, session: &RumiSession, backup_info: &BackupInfo) -> Result<()> {
        let metadata_dir = format!("{}/metadata", self.backup_base_path);
        let metadata_path = format!("{}/{}.json", metadata_dir, backup_info.id);

        // Create metadata directory
        session.execute_command_checked(&format!("sudo mkdir -p {}", metadata_dir))?;

        // Serialize backup info
        let metadata_json = serde_json::to_string_pretty(backup_info)
            .map_err(|e| RumiError::file_operation(format!("Failed to serialize backup metadata: {}", e)))?;

        // Save metadata file
        session.create_remote_file(&metadata_path, &metadata_json)?;

        Ok(())
    }
}

impl Default for BackupManager {
    fn default() -> Self {
        Self::new("/var/backups/rumi".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_info_serialization() {
        let backup_info = BackupInfo {
            id: "test-123".to_string(),
            deployment_name: "test-deployment".to_string(),
            domain: "example.com".to_string(),
            created_at: Utc::now(),
            backup_type: BackupType::Website,
            remote_path: "/var/backups/test.tar.gz".to_string(),
            size_bytes: 1024,
            description: Some("Test backup".to_string()),
        };

        let json = serde_json::to_string(&backup_info).unwrap();
        let deserialized: BackupInfo = serde_json::from_str(&json).unwrap();
        
        assert_eq!(backup_info.id, deserialized.id);
        assert_eq!(backup_info.domain, deserialized.domain);
        assert_eq!(backup_info.size_bytes, deserialized.size_bytes);
    }

    #[test]
    fn test_backup_manager_creation() {
        let manager = BackupManager::new("/custom/backup/path".to_string());
        assert_eq!(manager.backup_base_path, "/custom/backup/path");
        
        let default_manager = BackupManager::default();
        assert_eq!(default_manager.backup_base_path, "/var/backups/rumi");
    }
}