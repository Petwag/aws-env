use std::process::Command;

use cliclack::{input, note, spinner};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ListFunctionsOutput {
    #[serde(rename = "Functions")]
    functions: Vec<LambdaFunction>,
}

#[derive(Debug, Deserialize)]
struct LambdaFunction {
    #[serde(rename = "FunctionName")]
    function_name: String,
}

pub async fn get_function(
    profile: &String,
    function: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let function = match function {
        Some(f) => {
            note("Using lambda function", &f)?;
            f
        }
        None => {
            let spin = spinner();

            spin.start("Fetching lambda functions...");

            let output = Command::new("aws")
                .args([
                    "lambda",
                    "list-functions",
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

            let resp: ListFunctionsOutput = serde_json::from_str(&stdout)?;

            let functions: Vec<String> = resp
                .functions
                .into_iter()
                .map(|f| f.function_name)
                .collect();

            spin.stop("Lambda functions fetched");

            let choice: String = input("Search lambda function")
                .autocomplete(functions)
                .interact()?;

            choice
        }
    };

    Ok(function)
}
