use core::fmt;
use std::{collections::HashMap, process::Command};

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
    task_definition_version: &Option<String>,
    task_definition_container: &Option<String>,
    profile: &String,
) -> Result<(HashMap<String, String>, Option<String>), Box<dyn std::error::Error>> {
    let spin = spinner();

    spin.start("Fetching task definition...");

    let task_definition_full = match task_definition_version {
        Some(version) => format!("{}:{}", task_definition, version),
        None => task_definition.clone(),
    };

    let output = Command::new("aws")
        .args([
            "ecs",
            "describe-task-definition",
            "--task-definition",
            &task_definition_full,
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

    let selected_container = match task_definition_container {
        Some(container) => {
            if !container_list.contains(container) {
                return Err(
                    format!("Container '{}' not found in task definition", container).into(),
                );
            }
            container.clone()
        }
        None => {
            if container_list.len() == 1 {
                container_list[0].clone()
            } else {
                let container_items: Vec<(String, String, String)> = container_list
                    .iter()
                    .map(|name| (name.clone(), name.clone(), String::new()))
                    .collect();

                select("Choose container")
                    .items(&container_items)
                    .interact()?
            }
        }
    };

    let mut envs = HashMap::new();

    for container in containers {
        if container.name.as_deref() != Some(selected_container.as_str()) {
            continue;
        }

        if let Some(environment) = container.environment {
            for env in environment {
                let key = env.name.unwrap_or_default();
                let value = env.value.unwrap_or_default();

                envs.insert(key, value);
            }
        }
    }

    Ok((envs, Some(selected_container)))
}
