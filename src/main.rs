use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

use clap::Parser;
use cliclack::{confirm, intro, note, outro};
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
    /// only override credentials
    #[arg(long)]
    only_credentials: bool,
    /// Override fetched values with another file (key=value format)
    #[arg(long)]
    override_file: Option<String>,
    /// task definition name (for ecs)
    #[arg(short, long)]
    task_definition: Option<String>,
    /// task definition version (for ecs)
    #[arg(short = 'v', long)]
    task_definition_version: Option<i32>,
    /// task definition container (for ecs)
    #[arg(long)]
    task_definition_container: Option<String>,
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

        let profile = get_profile(args.profile).unwrap();

        if args.only_credentials == true {
            let mut envs = read_envs_from_file(&output);
            let credentials = get_credentials(&profile)
                .await
                .expect("Failed to get credentials");

            envs.extend(credentials);

            write_envs_to_file(&output, &envs);

            note(
                "Re-run command",
                &format!(
                    "aws-env --output {} --profile {} --only-credentials",
                    quote_arg(&output),
                    quote_arg(&profile)
                ),
            )
            .unwrap();

            outro("Done").unwrap();
            return;
        }

        let what = get_what(args.what).unwrap();

        let (mut envs, mut command) = match what {
            What::Ecs => {
                let (envs, task_definition, task_definition_version, task_definition_container) =
                    process_ecs(
                        &profile,
                        args.task_definition,
                        args.task_definition_version,
                        args.task_definition_container,
                    )
                    .await
                    .unwrap();
                let mut formatted = format!(
                    "aws-env --output {} --profile {} --what ecs --task-definition {}",
                    quote_arg(&output),
                    quote_arg(&profile),
                    quote_arg(&task_definition)
                );

                formatted = match task_definition_version {
                    Some(version) => {
                        format!(
                            "{} --task-definition-version {}",
                            formatted,
                            quote_arg(&version)
                        )
                    }
                    None => formatted,
                };

                formatted = match task_definition_container {
                    Some(container) => {
                        format!(
                            "{} --task-definition-container {}",
                            formatted,
                            quote_arg(&container)
                        )
                    }
                    None => formatted,
                };
                (envs, formatted)
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
        } else {
            let creds = confirm("Do you want to fetch credentials?")
                .interact()
                .unwrap();

            if creds == true {
                let credentials = get_credentials(&profile)
                    .await
                    .expect("Failed to get credentials");

                envs.extend(credentials);

                command.push_str(" --credentials");
            }
        }

        if args.override_file.is_some() {
            let override_file = args.override_file.unwrap();
            let override_envs = read_envs_from_file(&override_file);
            envs.extend(override_envs);

            command.push_str(&format!(" --override-file {}", quote_arg(&override_file)));
        }

        write_envs_to_file(&output, &envs);

        note("Re-run command", &command).unwrap();

        outro("Done").unwrap();
    });
}

fn quote_arg(value: &str) -> String {
    if value.is_empty() || value.chars().any(char::is_whitespace) || value.contains('"') {
        return format!("\"{}\"", value.replace('"', "\\\""));
    } else {
        value.to_string()
    }
}

fn write_envs_to_file(output: &String, envs: &HashMap<String, String>) {
    let file = File::create(output).unwrap();
    let mut writer = BufWriter::new(file);

    let mut items: Vec<(&String, &String)> = envs.iter().collect();

    items.sort_by_key(|(key, _value)| *key);

    for (key, value) in items {
        writeln!(writer, "{}=\"{}\"", key, value).expect("Unable to write to file"); // TODO: read about writeln! and expect.
    }
}

fn read_envs_from_file(output: &String) -> HashMap<String, String> {
    let content = std::fs::read_to_string(output).unwrap();

    content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '=');

            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => {
                    Some((key.to_string(), value.trim_matches('"').to_string()))
                }
                _ => None,
            }
        })
        .collect()
}
