use aws_config::BehaviorVersion;

use clap::Parser;
use cliclack::{intro, note, outro};

mod inputs;
use inputs::get_output::get_output;
use inputs::get_profile::get_profile;
use inputs::get_what::{What, get_what};

mod ecs_route;
use ecs_route::process_ecs::process_ecs;

mod lambda_route;
use lambda_route::process_lambda::process_lambda;
use tokio::runtime::Builder;

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
    let rt = Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap();

    rt.block_on(async {
        let args = Cli::parse();

        intro("Fetch Env").unwrap();

        let output = get_output(args.output).unwrap();

        let what = get_what(args.what).unwrap();

        let profile = get_profile(args.profile).unwrap();

        let config = aws_config::defaults(BehaviorVersion::latest())
            .profile_name(&profile)
            .load()
            .await;

        let mut command = match what {
            What::Ecs => {
                let task_definition = process_ecs(output.clone(), args.task_definition, &config)
                    .await
                    .unwrap();
                format!(
                    "aws-env --output {} --profile {} --what ecs --task-definition {}",
                    quote_arg(&output),
                    quote_arg(&profile),
                    quote_arg(&task_definition)
                )
            }
            What::Lambda => {
                let lambda = process_lambda(&profile, &output, args.lambda)
                    .await
                    .unwrap();
                format!(
                    "aws-env --output {} --profile {} --what lambda --lambda {}",
                    quote_arg(&output),
                    quote_arg(&profile),
                    quote_arg(&lambda)
                )
            }
        };

        if args.credentials == true {
            get_credentials(output, profile)
                .await
                .expect("Failed to get credentials");
            command.push_str(" --credentials");
        }

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
