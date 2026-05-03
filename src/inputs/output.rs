use cliclack::{input, note};

pub fn get_output(output: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let output = match output {
        Some(o) => {
            note("Using output", &o)?;
            o
        }
        None => input("Select the output location").interact()?,
    };

    Ok(output)
}