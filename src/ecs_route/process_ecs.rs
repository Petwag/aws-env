use std::collections::HashMap;

use super::get_env_vars::get_env_vars;
use super::get_task_definition::get_task_definition;

pub async fn process_ecs(
    profile: &String,
    task_definition: Option<String>,
    task_definition_version: Option<i32>,
) -> Result<(HashMap<String, String>, String, Option<String>), Box<dyn std::error::Error>> {
    let (task_definition, task_definition_version) =
        get_task_definition(task_definition, task_definition_version, &profile).await?;

    let envs = get_env_vars(&task_definition, &task_definition_version, &profile).await?;

    Ok((envs, task_definition, task_definition_version))
}
