
use clap::{ValueEnum};
use cliclack::{select, note};

#[derive(Clone, ValueEnum, Debug)]
pub enum What {
    Lambda,
    Ecs,
}


pub fn get_what(what: Option<What>) -> Result<What, Box<dyn std::error::Error>> {
    let what = match what {
        Some(w) => {
            note("Using resource type", &format!("{:?}", w)).unwrap();
            w
        },
        None => {
            let choice = select("Choose a resource type")
                .item("lambda", "AWS Lambda", "")
                .item("ecs", "Amazon ECS", "")
                .interact()
                .unwrap();

            match choice {
                "lambda" => What::Lambda,
                "ecs" => What::Ecs,
                _ => unreachable!(),
            }
        }
    };

    Ok(what)
}