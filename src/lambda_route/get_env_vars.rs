use aws_sdk_lambda::Client;
use cliclack::{spinner};

pub async fn get_env_vars(
    function: String,
    client: &Client,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let spin = spinner();

    spin.start("Fetching task definition...");

    let res = client
        .get_function_configuration()
        .function_name(function)
        .send()
        .await?;

    spin.stop("Task definition fetched");

    let vars = res
        .environment
        .and_then(|env| env.variables)
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<(String, String)>>();

    Ok(vars)
}