use aws_sdk_lambda::Client;
use cliclack::{input, note, spinner};

pub async fn get_function(
    function: Option<String>,
    client: &Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let function = match function {
        Some(f) => {
            note("Using lambda function", &f)?;
            f
        },
        None => {
            let spin = spinner();

            spin.start("Fetching lambda functions...");

            let mut functions = Vec::new();
            let mut marker = None;

            loop {
                let resp = client
                    .list_functions()
                    .set_marker(marker.clone())
                    .send()
                    .await?;

                if let Some(fs) = resp.functions {
                    for f in fs {
                        if let Some(name) = f.function_name {
                            functions.push(name);
                        }
                    }
                }

                match resp.next_marker {
                    Some(next) => marker = Some(next),
                    None => break,
                }
            }


            spin.stop("Lambda functions fetched");


            let choice: String = input("Search lambda function")
                .autocomplete(functions) // 👈 static list
                .interact()?;

            choice
      
        }
    };

    Ok(function)
}