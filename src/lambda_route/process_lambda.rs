use std::fs::File;
use std::io::{BufWriter, Write};

use super::get_env_vars::get_env_vars;
use super::get_function::get_function;

pub async fn process_lambda(
    profile: &String,
    output: &String,
    lambda: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let lambda = get_function(profile, lambda).await?;

    let envs = get_env_vars(profile, &lambda).await?;

    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);

    for (key, value) in envs.iter() {
        writeln!(writer, "{}=\"{}\"", key, value).expect("Unable to write to file"); // TODO: read about writeln! and expect.
    }

    Ok(lambda)
}
