use std::{collections::HashMap, process::Command};

use cliclack::spinner;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FunctionConfigurationOutput {
    #[serde(rename = "Environment")]
    environment: Option<Environment>,
}

#[derive(Debug, Deserialize)]
struct Environment {
    #[serde(rename = "Variables")]
    variables: Option<HashMap<String, String>>,
}

pub async fn get_env_vars(
    profile: &String,
    function: &String,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let spin = spinner();

    spin.start("Fetching task definition...");

    let output = Command::new("aws")
        .args([
            "lambda",
            "get-function-configuration",
            "--function-name",
            function,
            "--output",
            "json",
            "--profile",
            profile,
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "aws cli failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let stdout = String::from_utf8(output.stdout)?;

    let res: FunctionConfigurationOutput = serde_json::from_str(&stdout)?;

    spin.stop("Task definition fetched");

    let vars = res
        .environment
        .and_then(|env| env.variables)
        .unwrap_or_default();

    Ok(vars)
}
