
use memvid_core::Memvid;

pub fn create(filepath: &str) -> String {
    match Memvid::create(&filepath) {
        Ok(_) => {
            format!("Memory file created successfully at: {}", filepath)
        }
        Err(e) => {
            format!("Failed to create memory file at {}: {:?}", filepath, e)
        }
    }
}
