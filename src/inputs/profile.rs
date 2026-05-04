use cliclack::{note, select};
use std::process::Command;

pub fn get_profile(profile: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let profile = match profile {
        Some(p) => {
            note("Using profile", &p)?;
            p
        }
        None => {
            let profiles = get_aws_profiles()?;

            let items: Vec<(String, String, String)> = profiles
                .iter()
                .map(|p| (p.clone(), p.clone(), "".to_string()))
                .collect();

            select("Select the AWS profile")
                .items(&items)
                .interact()?
        }
    };

    Ok(profile)
}


fn get_aws_profiles() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("aws")
        .args(["configure", "list-profiles"])
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;

    Ok(stdout.lines().map(|s| s.to_string()).collect())
}