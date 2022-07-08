use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadStatus {
    code: String,
    status: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResult {
    errors: usize,
    warnings: usize,
    skipped: usize,
    total: usize,
    result: serde_json::Value,
}
