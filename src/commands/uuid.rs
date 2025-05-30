use anyhow::{anyhow, Result};
use uuid::{Uuid, Version};

pub fn generate(version: Option<u8>) -> Result<()> {
    let uuid = match version.unwrap_or(7) {
        1 => Uuid::now_v1(&[1, 2, 3, 4, 5, 6]),
        4 => Uuid::new_v4(),
        7 => Uuid::now_v7(),
        v => return Err(anyhow!("Unsupported UUID version: {}", v)),
    };
    
    println!("{}", uuid);
    Ok(())
}

pub fn info(uuid_str: &str) -> Result<()> {
    let uuid = Uuid::parse_str(uuid_str)?;
    
    println!("UUID: {}", uuid);
    println!("Version: {}", uuid.get_version_num());
    
    match uuid.get_version() {
        Some(Version::Mac) => {
            println!("Type: Time-based (MAC address)");
            if let Some(timestamp) = uuid.get_timestamp() {
                let unix_ts = timestamp.to_unix();
                println!("Timestamp: {} seconds, {} nanoseconds", unix_ts.0, unix_ts.1);
            }
        }
        Some(Version::Random) => {
            println!("Type: Random");
        }
        Some(Version::SortRand) => {
            println!("Type: Time-ordered random");
            if let Some(timestamp) = uuid.get_timestamp() {
                let unix_ts = timestamp.to_unix();
                println!("Timestamp: {} seconds, {} nanoseconds", unix_ts.0, unix_ts.1);
            }
        }
        _ => {
            println!("Type: Unknown or unsupported version");
        }
    }
    
    Ok(())
}