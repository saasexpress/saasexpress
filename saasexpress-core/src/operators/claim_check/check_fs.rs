use super::model::ClaimCheckReference;
use uuid::Uuid;

pub trait CheckStorage {
    fn store(&self, message: Vec<u8>) -> ClaimCheckReference;
}

pub struct CheckFsImpl;

impl CheckStorage for CheckFsImpl {
    fn store(&self, message: Vec<u8>) -> ClaimCheckReference {
        let uuid = Uuid::new_v4();

        // write to filesystem
        let file_path = format!("/tmp/claim_check_{}.bin", uuid);
        std::fs::write(&file_path, &message).expect("Unable to write file");

        ClaimCheckReference {
            claim_type: "filesystem".to_string(),
            claim_value: uuid.to_string(),
        }
    }
}
