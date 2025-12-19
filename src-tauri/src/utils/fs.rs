use anyhow::Context;

pub fn init_dir(dir: &str) -> Result<(), std::io::Error> {
    let data_dir = std::path::Path::new(dir);
    if !data_dir.exists() {
        match std::fs::create_dir(data_dir) {
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create data directory: {}", e)));
            }
            Ok(_) => {}
        }
    }

    let logs_dir = data_dir.join("logs");
    if !logs_dir.exists() {
        match std::fs::create_dir(logs_dir) {
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create logs directory: {}", e)));
            }
            Ok(_) => {}
        }
    }

    Ok(())
}