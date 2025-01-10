use anyhow::{Context, Result};
use serde_yaml::{self, Value};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Read the OpenAPI spec
    let openapi_path = "cloud/openapi.yml";
    let openapi_content =
        fs::read_to_string(openapi_path).context("Failed to read OpenAPI file")?;
    let mut openapi: Value =
        serde_yaml::from_str(&openapi_content).context("Failed to parse OpenAPI YAML")?;

    // Read the code samples overlay from the SDK directory
    let samples_path = PathBuf::from("../atoma-sdk-typescript/codeSamples.yaml");
    let samples_content =
        fs::read_to_string(&samples_path).context("Failed to read code samples file")?;
    let samples: Value =
        serde_yaml::from_str(&samples_content).context("Failed to parse code samples YAML")?;

    // Apply the overlay actions
    if let Some(actions) = samples["actions"].as_sequence() {
        for action in actions {
            let target = action["target"].as_str();
            let update = &action["update"];

            if let Some(target_str) = target {
                // Parse the JSON Path-like target
                let path = parse_json_path(target_str)?;

                // Apply the update to the target path in OpenAPI spec
                apply_update(&mut openapi, &path, update)?;
            }
        }
    }

    // Write the merged result back to the original file
    let output = serde_yaml::to_string(&openapi).context("Failed to serialize merged YAML")?;
    fs::write(openapi_path, output).context("Failed to write output file")?;

    println!("Successfully merged code samples into {}", openapi_path);
    Ok(())
}

fn parse_json_path(path: &str) -> Result<Vec<String>> {
    // Remove the leading $ if present
    let path = path.trim_start_matches('$');

    // Split the path into components and clean them
    let mut components = Vec::new();
    let mut current = String::new();
    let mut in_brackets = false;

    for c in path.chars() {
        match c {
            '[' => {
                if !current.is_empty() {
                    components.push(current.clone());
                    current.clear();
                }
                in_brackets = true;
            }
            ']' => {
                if !current.is_empty() {
                    components.push(current.trim_matches('"').to_string());
                    current.clear();
                }
                in_brackets = false;
            }
            _ => {
                if in_brackets || (!c.is_whitespace() && c != '/') {
                    current.push(c);
                }
            }
        }
    }

    if !current.is_empty() {
        components.push(current);
    }

    Ok(components)
}

fn apply_update(openapi: &mut Value, path: &[String], update: &Value) -> Result<()> {
    let mut current = openapi;

    // Navigate to the target location
    for (i, component) in path.iter().enumerate() {
        if i < path.len() - 1 {
            // Handle paths component that might contain special characters
            let clean_component = component.split('#').next().unwrap_or(component);
            current = current.get_mut(clean_component).context(format!(
                "Failed to navigate to path component: {}",
                clean_component
            ))?;
        }
    }

    // Apply the update at the final location
    if let Some(last) = path.last() {
        let clean_last = last.split('#').next().unwrap_or(last);
        if let Some(target) = current.get_mut(clean_last) {
            // Merge the update into the target
            merge_values(target, update);
        } else {
            // If the target doesn't exist, create it
            if let Some(map) = current.as_mapping_mut() {
                map.insert(Value::String(clean_last.to_string()), update.clone());
            }
        }
    }

    Ok(())
}

fn merge_values(target: &mut Value, update: &Value) {
    match (&mut *target, update) {
        (Value::Mapping(target_map), Value::Mapping(update_map)) => {
            for (key, value) in update_map {
                target_map.insert(key.clone(), value.clone());
            }
        }
        _ => {
            *target = update.clone();
        }
    }
}
