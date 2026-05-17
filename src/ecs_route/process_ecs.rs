use std::collections::HashMap;

use super::get_env_vars::get_env_vars;
use super::get_task_definition::get_task_definition;

pub async fn process_ecs(
    profile: &String,
    task_definition: Option<String>,
) -> Result<(HashMap<String, String>, String), Box<dyn std::error::Error>> {
    let task_definition = get_task_definition(task_definition, &profile).await?;

    let envs = get_env_vars(&task_definition, &profile).await?;

    Ok((envs, task_definition))
}
