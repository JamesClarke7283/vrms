use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use std::str;

pub async fn list_all() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let blacklist_url =
        "https://salsa.debian.org/debian/check-dfsg-status/-/raw/master/reasons/check-dfsg-status";
    let mut nonfree_packages = HashMap::new();

    let response = reqwest::get(blacklist_url).await?;
    let content = response.text().await?;

    for line in content.lines() {
        if !line.starts_with('#') && !line.trim().is_empty() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let package = parts[0].trim().to_string();
                let reason = parts[1].trim().to_string();
                nonfree_packages.insert(package, reason);
            }
        }
    }

    Ok(nonfree_packages)
}

pub async fn list_installed() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let all_nonfree_packages = list_all().await?;

    let output = Command::new("dpkg")
        .args(&["--get-selections"])
        .output()
        .expect("failed to execute dpkg");

    let installed_packages = str::from_utf8(&output.stdout).unwrap();

    let mut installed_nonfree_packages = HashMap::new();

    for line in installed_packages.lines() {
        let package = line.split_whitespace().next().unwrap_or_default();
        if let Some(reason) = all_nonfree_packages.get(package) {
            installed_nonfree_packages.insert(package.to_string(), reason.to_string());
        }
    }

    Ok(installed_nonfree_packages)
}
