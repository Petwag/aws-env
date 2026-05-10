use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::process::Command;

pub async fn get_credentials(
    output: String,
    profile: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let cli_result = Command::new("aws")
        .args(["configure", "export-credentials", "--profile", &profile])
        .output()?;

    if !cli_result.status.success() {
        return Err(String::from_utf8_lossy(&cli_result.stderr).into());
    }

    let stdout = String::from_utf8(cli_result.stdout)?;

    let access_key = extract_json_value(&stdout, "AccessKeyId").ok_or("Missing AccessKeyId")?;

    let secret_key =
        extract_json_value(&stdout, "SecretAccessKey").ok_or("Missing SecretAccessKey")?;

    let session_token = extract_json_value(&stdout, "SessionToken").unwrap_or_default();

    let file = OpenOptions::new()
        .create(true) // create if missing
        .append(true) // append instead of truncate
        .open(&output)?;

    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}=\"{}\"", "AWS_ACCESS_KEY_ID", access_key)
        .expect("Unable to write to file");
    writeln!(writer, "{}=\"{}\"", "AWS_SECRET_ACCESS_KEY", secret_key)
        .expect("Unable to write to file");
    writeln!(writer, "{}=\"{}\"", "AWS_SESSION_TOKEN", session_token)
        .expect("Unable to write to file");

    Ok(())
}

fn extract_json_value(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\":", key);

    let start = json.find(&pattern)? + pattern.len();

    let rest = json[start..].trim_start();

    if !rest.starts_with('"') {
        return None;
    }

    let rest = &rest[1..];

    let end = rest.find('"')?;

    Some(rest[..end].to_string())
}
