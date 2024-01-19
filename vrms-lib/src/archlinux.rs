use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use std::str;

pub async fn list_all() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let blacklist_url = "https://git.parabola.nu/blacklist.git/plain/blacklist.txt";
    let mut nonfree_packages = HashMap::new();

    let response = reqwest::get(blacklist_url).await?;
    let content = response.text().await?;

    content
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .for_each(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 2 {
                let package = parts[0].to_string();
                let reason_parts = &parts[2..];
                let reason = reason_parts.join(":");

                // Check if the reason contains any of the specified keywords
                if reason.contains("[nonfree]") || reason.contains("[uses-nonfree]") || reason.contains("[semifree]") {
                    nonfree_packages.insert(package, reason);
                }
            }
        });

    Ok(nonfree_packages)
}


pub async fn list_installed() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let all_nonfree_packages = list_all().await?;

    let output = Command::new("pacman")
        .arg("-Qq")
        .output()
        .expect("failed to execute pacman");

    let installed_packages = str::from_utf8(&output.stdout).unwrap();

    let mut installed_nonfree_packages = HashMap::new();

    for package in installed_packages.lines() {
        if let Some(reason) = all_nonfree_packages.get(package) {
            installed_nonfree_packages.insert(package.to_string(), reason.to_string());
        }
    }

    Ok(installed_nonfree_packages)
}
