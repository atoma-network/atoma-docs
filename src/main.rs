use anyhow::{Context, Result};
use serde::Serialize;
use serde_yaml::{self, Mapping, Value};
use std::fs;

fn main() -> Result<()> {
    let openapi_path = "cloud-api-reference/openapi.yml";

    // First merge the SDK samples
    merge_sdk_samples(
        openapi_path,
        "../atoma-sdk-typescript/codeSamples.yaml",
        "../atoma-sdk-python/codeSamples.yaml",
    )?;

    // Then apply the monkey patches
    apply_monkey_patches(openapi_path)?;

    Ok(())
}

fn apply_monkey_patches(openapi_path: &str) -> Result<()> {
    // Read the OpenAPI spec
    let openapi_content =
        fs::read_to_string(openapi_path).context("Failed to read OpenAPI file")?;
    let mut openapi: Value =
        serde_yaml::from_str(&openapi_content).context("Failed to parse OpenAPI YAML")?;

    // Create a new ordered mapping for paths
    let mut new_paths = Mapping::new();

    // Get the original paths in order and copy them to the new mapping, excluding the stream endpoints
    if let Some(paths) = openapi.get("paths").and_then(|v| v.as_mapping()) {
        for (path, value) in paths {
            if path.as_str() != Some("/v1/chat/completions#stream")
                && path.as_str() != Some("/v1/confidential/chat/completions#stream")
                && path.as_str() != Some("/v1/completions#stream")
                && path.as_str() != Some("/v1/confidential/completions#stream")
            {
                new_paths.insert(path.clone(), value.clone());
            }
        }
    }

    // Replace the paths object with our ordered version
    if let Some(paths_obj) = openapi.get_mut("paths") {
        *paths_obj = Value::Mapping(new_paths);
    }

    // Update code sample labels in the chat completions and completions endpoints
    if let Some(chat_endpoint) = openapi
        .get_mut("paths")
        .and_then(|v| v.get_mut("/v1/chat/completions"))
        .and_then(|v| v.get_mut("post"))
    {
        if let Some(samples) = chat_endpoint
            .get_mut("x-codeSamples")
            .and_then(|v| v.as_sequence_mut())
        {
            for sample in samples {
                if let Some(label) = sample.get_mut("label").and_then(|v| v.as_str()) {
                    let new_label = match label {
                        "chat_completions_create" => "default",
                        "chat_completions_create_stream" => "streaming",
                        _ => continue,
                    };

                    if let Some(label_value) = sample.get_mut("label") {
                        *label_value = Value::String(new_label.to_string());
                    }
                }
            }
        }
    }

    // Update code sample labels in the confidential chat completions endpoint
    if let Some(chat_endpoint) = openapi
        .get_mut("paths")
        .and_then(|v| v.get_mut("/v1/confidential/chat/completions"))
        .and_then(|v| v.get_mut("post"))
    {
        if let Some(samples) = chat_endpoint
            .get_mut("x-codeSamples")
            .and_then(|v| v.as_sequence_mut())
        {
            for sample in samples {
                if let Some(label) = sample.get_mut("label").and_then(|v| v.as_str()) {
                    let new_label = match label {
                        "confidential_chat_completions_create" => "default",
                        "confidential_chat_completions_create_stream" => "streaming",
                        _ => continue,
                    };

                    if let Some(label_value) = sample.get_mut("label") {
                        *label_value = Value::String(new_label.to_string());
                    }
                }
            }
        }
    }

    // Update code sample labels in the completions endpoint
    if let Some(completions_endpoint) = openapi
        .get_mut("paths")
        .and_then(|v| v.get_mut("/v1/completions"))
        .and_then(|v| v.get_mut("post"))
    {
        if let Some(samples) = completions_endpoint
            .get_mut("x-codeSamples")
            .and_then(|v| v.as_sequence_mut())
        {
            for sample in samples {
                if let Some(label) = sample.get_mut("label").and_then(|v| v.as_str()) {
                    let new_label = match label {
                        "completions_create" => "default",
                        "completions_create_stream" => "streaming",
                        _ => continue,
                    };

                    if let Some(label_value) = sample.get_mut("label") {
                        *label_value = Value::String(new_label.to_string());
                    }
                }
            }
        }
    }

    // Update code sample labels in the confidential completions endpoint
    if let Some(completions_endpoint) = openapi
        .get_mut("paths")
        .and_then(|v| v.get_mut("/v1/confidential/completions"))
        .and_then(|v| v.get_mut("post"))
    {
        if let Some(samples) = completions_endpoint
            .get_mut("x-codeSamples")
            .and_then(|v| v.as_sequence_mut())
        {
            for sample in samples {
                if let Some(label) = sample.get_mut("label").and_then(|v| v.as_str()) {
                    let new_label = match label {
                        "confidential_completions_create" => "default",
                        "confidential_completions_create_stream" => "streaming",
                        _ => continue,
                    };

                    if let Some(label_value) = sample.get_mut("label") {
                        *label_value = Value::String(new_label.to_string());
                    }
                }
            }
        }
    }

    // Write the patched result back to the file using a custom serializer that preserves order
    let mut writer = Vec::new();
    let mut serializer = serde_yaml::Serializer::new(&mut writer);
    openapi
        .serialize(&mut serializer)
        .context("Failed to serialize YAML")?;
    fs::write(openapi_path, writer).context("Failed to write output file")?;

    println!("Successfully applied monkey patches to {}", openapi_path);
    Ok(())
}

fn merge_sdk_samples(
    openapi_path: &str,
    ts_samples_path: &str,
    py_samples_path: &str,
) -> Result<()> {
    // Read the OpenAPI spec
    let openapi_content =
        fs::read_to_string(openapi_path).context("Failed to read OpenAPI file")?;
    let mut openapi: Value =
        serde_yaml::from_str(&openapi_content).context("Failed to parse OpenAPI YAML")?;

    // Read the TypeScript code samples
    let ts_samples_content = fs::read_to_string(ts_samples_path)
        .context("Failed to read TypeScript code samples file")?;
    let ts_samples: Value = serde_yaml::from_str(&ts_samples_content)
        .context("Failed to parse TypeScript code samples YAML")?;

    // Read the Python code samples
    let py_samples_content =
        fs::read_to_string(py_samples_path).context("Failed to read Python code samples file")?;
    let py_samples: Value = serde_yaml::from_str(&py_samples_content)
        .context("Failed to parse Python code samples YAML")?;

    // Apply the TypeScript overlay actions first
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

    // Then apply the Python overlay actions
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

    // Sort all code samples in the OpenAPI spec to ensure TypeScript comes first
    sort_code_samples(&mut openapi);

    // Write the merged result back to the original file
    let output = serde_yaml::to_string(&openapi).context("Failed to serialize merged YAML")?;
    fs::write(openapi_path, output).context("Failed to write output file")?;

    println!(
        "Successfully merged TypeScript and Python code samples into {}",
        openapi_path
    );
    Ok(())
}

fn sort_code_samples(value: &mut Value) {
    match value {
        Value::Mapping(map) => {
            // Sort x-codeSamples if present
            if let Some(samples) = map.get_mut("x-codeSamples") {
                if let Some(samples_array) = samples.as_sequence_mut() {
                    samples_array.sort_by(|a, b| {
                        let a_lang = a.get("lang").and_then(|v| v.as_str()).unwrap_or("");
                        let b_lang = b.get("lang").and_then(|v| v.as_str()).unwrap_or("");
                        let a_label = a.get("label").and_then(|v| v.as_str()).unwrap_or("");
                        let b_label = b.get("label").and_then(|v| v.as_str()).unwrap_or("");

                        // First, compare languages (TypeScript comes before Python)
                        let lang_order = match (a_lang, b_lang) {
                            ("typescript", "python") => std::cmp::Ordering::Less,
                            ("python", "typescript") => std::cmp::Ordering::Greater,
                            _ => a_lang.cmp(b_lang),
                        };

                        if lang_order != std::cmp::Ordering::Equal {
                            return lang_order;
                        }

                        // Within the same language, sort by streaming vs non-streaming
                        let a_is_stream = a_label.contains("stream");
                        let b_is_stream = b_label.contains("stream");

                        match (a_is_stream, b_is_stream) {
                            (true, false) => std::cmp::Ordering::Greater,
                            (false, true) => std::cmp::Ordering::Less,
                            _ => a_label.cmp(b_label),
                        }
                    });
                }
            }

            // Recursively sort code samples in nested objects
            for (_, v) in map.iter_mut() {
                sort_code_samples(v);
            }
        }
        Value::Sequence(seq) => {
            // Recursively sort code samples in array elements
            for item in seq.iter_mut() {
                sort_code_samples(item);
            }
        }
        _ => {}
    }
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

fn is_duplicate_sample(existing: &Value, new_sample: &Value) -> bool {
    let existing_lang = existing.get("lang").and_then(|v| v.as_str());
    let existing_label = existing.get("label").and_then(|v| v.as_str());
    let new_lang = new_sample.get("lang").and_then(|v| v.as_str());
    let new_label = new_sample.get("label").and_then(|v| v.as_str());

    match (existing_lang, existing_label, new_lang, new_label) {
        (Some(el), Some(elbl), Some(nl), Some(nlbl)) => el == nl && elbl == nlbl,
        _ => false,
    }
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
                            'new_sample: for new_sample in new_array {
                                // Check if this sample already exists
                                for existing_sample in samples_array.iter() {
                                    if is_duplicate_sample(existing_sample, new_sample) {
                                        continue 'new_sample;
                                    }
                                }
                                // If we get here, this is a new sample
                                samples_array.push(new_sample.clone());
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
