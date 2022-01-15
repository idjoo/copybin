use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize)]
pub struct Cred {
    pub devkey: String,
    pub username: String,
    pub password: String,
}

impl Cred {
    fn create(path: &Path) -> std::io::Result<()> {
        // Create config dir
        let config_dir = path.parent().unwrap();
        let _ = fs::create_dir_all(config_dir);

        // Create config file
        let mut file = fs::File::create(path)?;

        // Populate config file
        let contents = b"devkey = \"\"\nusername = \"\"\npassword = \"\"\n";
        file.write_all(contents)?;

        Ok(())
    }

    pub fn load() -> Self {
        let config_dir = dirs::config_dir().unwrap();
        let config_path = config_dir.join("copybin/config.toml");
        if !config_path.exists() {
            println!("No config file found. Creating one...");
            Cred::create(&config_path).unwrap();
        }

        let mut file = fs::File::open(&config_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let cred: Cred = toml::from_str(&contents).unwrap();

        if cred.devkey.is_empty() {
            println!("No devkey found. Please enter one.");
        }
        if !cred.username.is_empty() && !cred.password.is_empty() {}

        cred
    }
}
