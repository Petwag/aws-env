use std::process::Command;

use cliclack::{input, note, spinner};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ListTaskDefinitionsOutput {
    #[serde(rename = "taskDefinitionArns")]
    task_definition_arns: Vec<String>,
}

pub async fn get_task_definition(
    task_definition: Option<String>,
    profile: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let task_definition = match task_definition {
        Some(td) => {
            note("Using task definition", &td)?;
            td
        }
        None => {
            let spin = spinner();

            spin.start("Fetching task definitions...");

            let output = Command::new("aws")
                .args([
                    "ecs",
                    "list-task-definitions",
                    "--sort",
                    "DESC",
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

            let res: ListTaskDefinitionsOutput = serde_json::from_str(&stdout)?;

            spin.stop("Task definitions fetched");

            let all: Vec<String> = res
                .task_definition_arns
                .iter()
                .map(|arn| arn.rsplit('/').next().unwrap_or(arn).to_string())
                .collect();

            let choice: String = input("Search task definition")
                .autocomplete(all)
                .interact()?;

            choice
        }
    };

    Ok(task_definition)
}
