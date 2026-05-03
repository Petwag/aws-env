use aws_sdk_ecs::Client;
use cliclack::{input, note, spinner};

pub async fn get_task_definition(
    task_definition: Option<String>,
    client: &Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let task_definition = match task_definition {
        Some(td) => {
            note("Using task definition", &td)?;
            td
        },
        None => {
            let spin = spinner();

            spin.start("Fetching task definitions...");

            let res = client
                .list_task_definitions()
                .sort(aws_sdk_ecs::types::SortOrder::Desc)
                .send()
                .await?;

            spin.stop("Task definitions fetched");

            let all: Vec<String> = res
                .task_definition_arns()
                .iter()
                .map(|arn| {
                    arn.rsplit('/')
                        .next()
                        .unwrap_or(arn)
                        .to_string()
                })
                .collect();

            let choice: String = input("Search task definition")
                .autocomplete(all) // 👈 static list
                .interact()?;

            choice
      
        }
    };

    Ok(task_definition)
}