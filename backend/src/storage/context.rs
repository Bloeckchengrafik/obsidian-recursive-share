use crate::storage::meta::Meta;
use rocket::fs::TempFile;
use std::path::PathBuf;

pub struct StorageContext {
    pub path: PathBuf,
}

impl StorageContext {
    pub fn new(path: PathBuf, id: &str, root_name: &str) -> Self {
        let meta = Meta::new(id.to_string(), root_name.to_string());
        let path = path.join(id);

        std::fs::create_dir_all(&path).expect("Failed to create directory");
        let meta_path = path.join("meta.json");
        std::fs::write(
            &meta_path,
            serde_json::to_string(&meta).expect("Failed to serialize meta"),
        )
        .expect("Failed to write meta file");

        Self { path }
    }

    pub fn load(path: PathBuf) -> Self {
        Self {
            path,
        }
    }

    pub fn meta(&self) -> Meta {
        let meta_path = self.path.join("meta.json");
        let meta_data = std::fs::read_to_string(&meta_path).expect("Failed to read meta file");
        serde_json::from_str(&meta_data).expect("Failed to deserialize meta")
    }

    pub async fn add_file(&self, file: &mut TempFile<'_>, name: &str) {
        println!("File name: {}", name);
        let file_path = self.path.join(name);
        println!("File path: {}", file_path.to_str().unwrap());
        std::fs::create_dir_all(&self.path).expect("Failed to create directory");
        file.persist_to(&file_path)
            .await
            .expect("Failed to persist file");
    }

    pub fn list_files_recursively(&self) -> Vec<String> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(&self.path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    files.push(name.to_string());
                }
            } else if path.is_dir() {
                files.extend(StorageContext::load(path).list_files_recursively());
            }
        }
        files
            .iter()
            .filter(|f| f != &"meta.json")
            .map(|f| f.to_string())
            .collect()
    }

    pub fn get_content(&self) -> String {
        let meta = self.meta();
        let root_file = self.path.clone()
            .join(meta.title);

        let content = std::fs::read_to_string(root_file)
            .expect("Failed to read root file");

        content
    }
}
