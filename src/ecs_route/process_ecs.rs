use std::fs::File;
use std::io::{Write, BufWriter};

use aws_sdk_ecs::Client;

use super::get_env_vars::get_env_vars;
use super::get_task_definition::get_task_definition;

pub async fn process_ecs(
    output: String,
    task_definition: Option<String>,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let task_definition = get_task_definition(task_definition, client).await?;
    
    let envs = get_env_vars(task_definition, client).await?;

    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);

    for (key, value) in envs.iter() {
        writeln!(writer, "{}=\"{}\"", key, value).expect("Unable to write to file"); // TODO: read about writeln! and expect.
    }

    Ok(())
}