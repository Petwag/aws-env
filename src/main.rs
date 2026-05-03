use aws_config::BehaviorVersion;

use clap::{Parser};
use cliclack::{intro, outro};


mod inputs;
use inputs::what::{get_what, What};
use inputs::output::{get_output};
use inputs::profile::{get_profile};

mod ecs_route;
use ecs_route::process_ecs::process_ecs;

mod lambda_route;
use lambda_route::process_lambda::process_lambda;


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
    /// task definition name (for ecs)
    #[arg(short, long)]
    task_definition: Option<String>,
    /// lambda function name (for lambda)
    #[arg(short, long)]
    lambda: Option<String>,
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();

    intro("Fetch Env").unwrap();

    let output = get_output(args.output).unwrap();

    let what = get_what(args.what).unwrap();

    let profile = get_profile(args.profile).unwrap();

    let config = aws_config::defaults(BehaviorVersion::latest())
    .profile_name(&profile)
    .load()
    .await;


    match what {
        What::Ecs => process_ecs(output, args.task_definition, &config).await.unwrap(),
        What::Lambda => process_lambda(output, args.lambda, &config).await.unwrap(),
    }

    outro("Done").unwrap();

}


