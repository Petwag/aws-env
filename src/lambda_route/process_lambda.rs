use std::collections::HashMap;

use super::get_env_vars::get_env_vars;
use super::get_function::get_function;

pub async fn process_lambda(
    profile: &String,
    lambda: Option<String>,
) -> Result<(HashMap<String, String>, String), Box<dyn std::error::Error>> {
    let lambda = get_function(profile, lambda).await?;

    let envs = get_env_vars(profile, &lambda).await?;

    Ok((envs, lambda))
}
