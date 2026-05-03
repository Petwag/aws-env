use aws_sdk_ecs::Client;
use cliclack::{select, spinner};

pub async fn get_env_vars(
    task_definition: String,
    client: &Client,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let spin = spinner();

    spin.start("Fetching task definition...");

    let res = client
        .describe_task_definition()
        .task_definition(task_definition)
        .send()
        .await?;

    spin.stop("Task definition fetched");

    let task_def = res
        .task_definition()
        .ok_or("No task definition found")?;

    let containers = task_def.container_definitions();

    // collect container names
    let container_list: Vec<String> = containers
        .iter()
        .filter_map(|c| c.name().map(|s| s.to_string()))
        .collect();

    if container_list.is_empty() {
        return Err("No containers found".into());
    }

    // pick container
    let selected_container = if container_list.len() == 1 {
        container_list[0].to_string()
    } else {
        let container_items: Vec<(String, String, String)> = container_list
            .iter()
            .map(|name| (name.clone(), name.clone(), String::new()))
            .collect();

        select("Choose container")
        .items(&container_items)
        .interact()
        .unwrap()
    };

    // extract envs only for selected container
    let mut envs = Vec::new();

    for container in containers {
        if container.name() != Some(selected_container.as_str()) {
            continue;
        }

        for env in container.environment() {
            let key = env.name().unwrap_or("").to_string();
            let value = env.value().unwrap_or("").to_string();

            envs.push((key, value));
        }
    }

    envs.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(envs)
}