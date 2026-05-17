use std::fs::File;
use std::io::{BufWriter, Write};

use clap::Parser;
use cliclack::{intro, note, outro};
use tokio::runtime::Builder;

mod inputs;
use inputs::get_output::get_output;
use inputs::get_profile::get_profile;
use inputs::get_what::{What, get_what};

mod ecs_route;
use ecs_route::process_ecs::process_ecs;

mod lambda_route;
use lambda_route::process_lambda::process_lambda;

use crate::inputs::get_credentials::get_credentials;

#[derive(Parser)]
struct Cli {
    /// The file to write the env to
    #[arg(short, long)]
    output: Option<String>,
    /// aws profile to use
    #[arg(short, long)]
    profile: Option<String>,
    /// type or ressource
    #[arg(short, long, value_enum)]
    what: Option<What>,
    /// add credentials to output
    #[arg(short, long)]
    credentials: bool,
    /// task definition name (for ecs)
    #[arg(short, long)]
    task_definition: Option<String>,
    /// lambda function name (for lambda)
    #[arg(short, long)]
    lambda: Option<String>,
}

fn main() {
    let rt = Builder::new_current_thread().build().unwrap();

    rt.block_on(async {
        let args = Cli::parse();

        intro("Fetch Env").unwrap();

        let output = get_output(args.output).unwrap();

        let what = get_what(args.what).unwrap();

        let profile = get_profile(args.profile).unwrap();

        let (mut envs, mut command) = match what {
            What::Ecs => {
                let (envs, task_definition) =
                    process_ecs(&profile, args.task_definition).await.unwrap();
                (
                    envs,
                    format!(
                        "aws-env --output {} --profile {} --what ecs --task-definition {}",
                        quote_arg(&output),
                        quote_arg(&profile),
                        quote_arg(&task_definition)
                    ),
                )
            }
            What::Lambda => {
                let (envs, lambda) = process_lambda(&profile, args.lambda).await.unwrap();
                (
                    envs,
                    format!(
                        "aws-env --output {} --profile {} --what lambda --lambda {}",
                        quote_arg(&output),
                        quote_arg(&profile),
                        quote_arg(&lambda)
                    ),
                )
            }
        };

        if args.credentials == true {
            let credentials = get_credentials(&profile)
                .await
                .expect("Failed to get credentials");

            envs.extend(credentials);

            command.push_str(" --credentials");
        }

        write_envs_to_file(&output, &envs);

        note("Re-run command", &command).unwrap();

        outro("Done").unwrap();
    });
}

fn quote_arg(value: &str) -> String {
    if value.is_empty() || value.chars().any(char::is_whitespace) || value.contains('"') {
        format!("\"{}\"", value.replace('"', "\\\""))
    } else {
        value.to_string()
    }
}

fn write_envs_to_file(output: &String, envs: &Vec<(String, String)>) {
    let file = File::create(output).unwrap();
    let mut writer = BufWriter::new(file);

    for (key, value) in envs.iter() {
        writeln!(writer, "{}=\"{}\"", key, value).expect("Unable to write to file"); // TODO: read about writeln! and expect.
    }
}
