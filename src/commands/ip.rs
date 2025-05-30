use anyhow::Result;
use if_addrs::{get_if_addrs, IfAddr};

pub async fn external() -> Result<()> {
    let response = reqwest::get("https://checkip.amazonaws.com")
        .await?
        .text()
        .await?;
    
    println!("{}", response.trim());
    Ok(())
}

pub fn internal() -> Result<()> {
    let interfaces = get_if_addrs()?;
    
    for interface in interfaces {
        if !interface.is_loopback() {
            match interface.addr {
                IfAddr::V4(ipv4) => println!("{}: {}", interface.name, ipv4.ip),
                IfAddr::V6(ipv6) => println!("{}: {}", interface.name, ipv6.ip),
            }
        }
    }
    
    Ok(())
}