use std::fs::File;
use std::io::{Write, BufWriter};

use aws_config::SdkConfig;
use aws_sdk_lambda::Client;

use super::get_env_vars::get_env_vars;
use super::get_function::get_function;

pub async fn process_lambda(
    output: String,
    lambda: Option<String>,
    config: &SdkConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(&config);

    let lambda = get_function(lambda, &client).await?;
    
    let envs = get_env_vars(lambda, &client).await?;

    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);

    for (key, value) in envs.iter() {
        writeln!(writer, "{}=\"{}\"", key, value).expect("Unable to write to file"); // TODO: read about writeln! and expect.
    }

    Ok(())
}