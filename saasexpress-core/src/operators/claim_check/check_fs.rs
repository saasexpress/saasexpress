use std::fmt;

use super::model::ClaimCheckReference;
use tracing::warn;
use uuid::Uuid;

type Result<T> = std::result::Result<T, ClaimCheckInvalidError>;

#[derive(Debug)]
pub struct ClaimCheckInvalidError;

impl fmt::Display for ClaimCheckInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Claim check is either invalid or has expired.")
    }
}

pub trait CheckStorage {
    fn put(&self, message: Vec<u8>) -> ClaimCheckReference;
    fn get(&self, claim_id: &str) -> Result<Vec<u8>>;
    fn clear(&self, claim_id: &str);
}

pub struct CheckFsImpl;

impl CheckStorage for CheckFsImpl {
    fn put(&self, message: Vec<u8>) -> ClaimCheckReference {
        let uuid = Uuid::new_v4();

        // write to filesystem
        let file_path = format!("/tmp/claim_check_{}.bin", uuid);
        std::fs::write(&file_path, &message).expect("Unable to write file");

        ClaimCheckReference {
            claim_type: "filesystem".to_string(),
            claim_value: uuid.to_string(),
        }
    }

    fn get(&self, claim_id: &str) -> Result<Vec<u8>> {
        let file_path = format!("/tmp/claim_check_{}.bin", claim_id);
        let result = std::fs::read(&file_path);
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(ClaimCheckInvalidError),
        }
    }

    fn clear(&self, claim_id: &str) {
        let file_path = format!("/tmp/claim_check_{}.bin", claim_id);
        if std::fs::remove_file(&file_path).is_err() {
            warn!("Claim already cleared or not found: {}", claim_id);
        }
    }
}
