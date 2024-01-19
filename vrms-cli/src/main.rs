use clap::{Arg,ArgAction,Command};
use std::error::Error;
use tokio;
use vrms_lib::archlinux::list_installed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("vrms")
        .version("0.1.0")
        .author("impulse")
        .about("Lists non-free packages installed")
        .arg(
            Arg::new("explain")
                .short('e')
                .long("explain")
                .help("Display reasons for non-free packages")
                .action(ArgAction::SetTrue), // Set this for a boolean flag
        )
        .get_matches();

    let nonfree_packages = list_installed().await?;

    if *matches.get_one::<bool>("explain").unwrap_or(&false) {
        for (package, reason) in &nonfree_packages {
            println!("{}: {}", package, reason);
        }
    } else {
        for package in nonfree_packages.keys() {
            println!("{}", package);
        }
    }

    Ok(())
}
