use std::process::Command;

use cliclack::{select, spinner};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DescribeTaskDefinitionOutput {
    #[serde(rename = "taskDefinition")]
    task_definition: Option<TaskDefinition>,
}

#[derive(Debug, Deserialize)]
struct TaskDefinition {
    #[serde(rename = "containerDefinitions")]
    container_definitions: Vec<ContainerDefinition>,
}

#[derive(Debug, Deserialize)]
struct ContainerDefinition {
    #[serde(rename = "name")]
    name: Option<String>,

    #[serde(rename = "environment")]
    environment: Option<Vec<EnvironmentVariable>>,
}

#[derive(Debug, Deserialize)]
struct EnvironmentVariable {
    #[serde(rename = "name")]
    name: Option<String>,

    #[serde(rename = "value")]
    value: Option<String>,
}

pub async fn get_env_vars(
    task_definition: &String,
    profile: &String,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let spin = spinner();

    spin.start("Fetching task definition...");

    let output = Command::new("aws")
        .args([
            "ecs",
            "describe-task-definition",
            "--task-definition",
            task_definition,
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

    let res: DescribeTaskDefinitionOutput = serde_json::from_str(&stdout)?;

    spin.stop("Task definition fetched");

    let task_def = res.task_definition.ok_or("No task definition found")?;

    let containers = task_def.container_definitions;

    let container_list: Vec<String> = containers.iter().filter_map(|c| c.name.clone()).collect();

    if container_list.is_empty() {
        return Err("No containers found".into());
    }

    let selected_container = if container_list.len() == 1 {
        container_list[0].clone()
    } else {
        let container_items: Vec<(String, String, String)> = container_list
            .iter()
            .map(|name| (name.clone(), name.clone(), String::new()))
            .collect();

        select("Choose container")
            .items(&container_items)
            .interact()?
    };

    let mut envs = Vec::new();

    for container in containers {
        if container.name.as_deref() != Some(selected_container.as_str()) {
            continue;
        }

        if let Some(environment) = container.environment {
            for env in environment {
                let key = env.name.unwrap_or_default();
                let value = env.value.unwrap_or_default();

                envs.push((key, value));
            }
        }
    }

    envs.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(envs)
}
