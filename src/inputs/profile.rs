use cliclack::{input, note};

pub fn get_profile(profile: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let profile = match profile {
        Some(p) => {
            note("Using profile", &p)?;
            p
        }
        None => input("Select AWS profile").interact()?,
    };

    Ok(profile)
}