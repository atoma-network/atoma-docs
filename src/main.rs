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

    // Read the TypeScript code samples
    let ts_samples_path = PathBuf::from("../atoma-sdk-typescript/codeSamples.yaml");
    let ts_samples_content = fs::read_to_string(&ts_samples_path)
        .context("Failed to read TypeScript code samples file")?;
    let ts_samples: Value = serde_yaml::from_str(&ts_samples_content)
        .context("Failed to parse TypeScript code samples YAML")?;

    // Read the Python code samples
    let py_samples_path = PathBuf::from("../atoma-sdk-python/codeSamples.yaml");
    let py_samples_content =
        fs::read_to_string(&py_samples_path).context("Failed to read Python code samples file")?;
    let py_samples: Value = serde_yaml::from_str(&py_samples_content)
        .context("Failed to parse Python code samples YAML")?;

    // Apply the TypeScript overlay actions
    if let Some(actions) = ts_samples["actions"].as_sequence() {
        for action in actions {
            let target = action["target"].as_str();
            let update = &action["update"];

            if let Some(target_str) = target {
                let path = parse_json_path(target_str)?;
                merge_code_sample(&mut openapi, &path, update)?;
            }
        }
    }

    // Apply the Python overlay actions
    if let Some(actions) = py_samples["actions"].as_sequence() {
        for action in actions {
            let target = action["target"].as_str();
            let update = &action["update"];

            if let Some(target_str) = target {
                let path = parse_json_path(target_str)?;
                merge_code_sample(&mut openapi, &path, update)?;
            }
        }
    }

    // Write the merged result back to the original file
    let output = serde_yaml::to_string(&openapi).context("Failed to serialize merged YAML")?;
    fs::write(openapi_path, output).context("Failed to write output file")?;

    println!(
        "Successfully merged TypeScript and Python code samples into {}",
        openapi_path
    );
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

fn merge_code_sample(openapi: &mut Value, path: &[String], update: &Value) -> Result<()> {
    let mut current = openapi;

    // Navigate to the target location
    for (i, component) in path.iter().enumerate() {
        if i < path.len() - 1 {
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
            // For x-codeSamples, we want to append rather than replace
            if let Some(code_samples) = target.get_mut("x-codeSamples") {
                if let Some(samples_array) = code_samples.as_sequence_mut() {
                    if let Some(new_samples) = update.get("x-codeSamples") {
                        if let Some(new_array) = new_samples.as_sequence() {
                            for sample in new_array {
                                samples_array.push(sample.clone());
                            }
                        }
                    }
                }
            } else {
                // If x-codeSamples doesn't exist yet, just set it
                merge_values(target, update);
            }
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
