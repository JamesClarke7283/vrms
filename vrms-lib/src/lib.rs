pub mod archlinux;
use std::collections::HashMap;
use std::error::Error;

pub async fn list_installed() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let os_type = sys_info::os_type()?;
    if os_type == "Linux" {
        let os_release = sys_info::linux_os_release()?;
        let distro = os_release.id_like.unwrap_or_default().to_lowercase();
        if &distro == "arch" {
            return archlinux::list_installed().await;
        }
    }
    Err("Unsupported distribution. Currently, only Arch Linux is supported.".into())
}
