use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadStatus {
    code: String,
    status: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResult {
    errors: Option<Vec<String>>,
    warnings: Option<Vec<String>>,
    skipped: Option<Vec<String>>,
    total: Option<Vec<String>>,
    result: Option<Vec<String>>,
}
