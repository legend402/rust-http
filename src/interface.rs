use serde::Serialize;

#[derive(Serialize)]
pub struct FileList {
  pub file_type: String,
  pub file_path: String,
}
