use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::Value;
use std::io::{self, Read};

pub fn decode() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let input = input.trim();
    let decoded_bytes = STANDARD.decode(input)?;
    
    let value: Value = rmp_serde::from_slice(&decoded_bytes)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    
    Ok(())
}