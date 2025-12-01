use std::{collections::HashMap, fs::OpenOptions, io::Write, path::PathBuf, str::FromStr};

const SECRETS_PATH: &str = "../get_input/secrets.toml";
const SESSION_KEY: &str = "session";
const CACHE_PATH: &str = "./cache/input.txt";

pub fn get_input(day: usize) -> Result<String, String> {
    if let Some(cached) = get_cached(CACHE_PATH) {
        println!("Using cached local input \"{}\"", CACHE_PATH);
        return Ok(cached);
    }

    println!("No input cache detected, fetching input");

    let session_token = format!("{}={}", SESSION_KEY, get_session_token(SECRETS_PATH)?);

    let uri = compose_uri(day);

    let client = reqwest::blocking::Client::builder()
        .build()
        .map_err(|e| e.to_string())?;

    let response = match client.get(&uri).header("Cookie", session_token).send() {
        Ok(response) => response.error_for_status(),
        Err(err) => {
            return Err(format!(
                "Failed to retrieve input from \"{}\": {}",
                uri, err
            ));
        }
    };

    let content = match response {
        Ok(ok_response) => ok_response.text(),
        Err(err) => {
            return Err(format!(
                "Failed to retrieve input from \"{}\": {}",
                uri, err
            ));
        }
    };

    let content = content.map_err(|e| e.to_string())?;

    store_cache(CACHE_PATH, &content)?;

    Ok(content)
}

fn compose_uri(day: usize) -> String {
    format!("https://adventofcode.com/2025/day/{}/input", day)
}

fn get_session_token(secrets_path: &str) -> Result<String, String> {
    let secrets = std::fs::read_to_string(secrets_path)
        .map_err(|e| format!("Could not read secrets file \"{}\": {}", secrets_path, e))?;

    let secrets: HashMap<String, String> = toml::from_str(&secrets)
        .map_err(|e| format!("Could not parse secrets file \"{}\": {}", secrets_path, e))?;

    secrets
        .get(SESSION_KEY)
        .ok_or(format!(
            "Could not find key \"{}\" in secrets file \"{}\"",
            SESSION_KEY, secrets_path
        ))
        .cloned()
}

fn get_cached(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn store_cache(path: &str, contents: &str) -> Result<(), String> {
    let os_path = PathBuf::from_str(path).map_err(|e| e.to_string())?;
    let dir_path = os_path.parent().ok_or("Cache path has no parent".to_string())?;
    std::fs::create_dir_all(dir_path).map_err(|e| e.to_string())?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    file.write_all(contents.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}
