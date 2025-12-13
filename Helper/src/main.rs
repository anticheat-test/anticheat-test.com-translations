use serde_json::{Value, Map};
use std::path::Path;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: english.json output_file.json");
        return;
    }

    let english_file_path = &args[1];
    let output_file = &args[2];

    println!("Input file: {}", english_file_path);
    println!("Output file: {}", output_file);

    let english_file_exists = fs::exists(&english_file_path).unwrap_or(false);
    if !english_file_exists {
        eprintln!("Unable to find input file (english.json)");
        return;
    }

    let output_exist = fs::exists(&output_file).unwrap_or(false);

    if !output_exist {
        println!("Unable to find input file, creating a clone of the input file");
        match fs::copy(english_file_path, output_file) {
            Ok(_) => println!("Copied input file to {}", output_file),
            Err(_) => eprintln!("Failed to copy input file to {}", output_file),
        }
        return;
    }

    println!("Both input and the output file exist, \
    merging contents of the input file into the output file...");

    let english_file = Path::new(english_file_path);
    let output_file = Path::new(output_file);
    read_files(english_file, output_file);
    println!("Files has been merged!")
}

fn read_files(input: &Path, output: &Path) {
    match fs::read_to_string(&input) {
        Ok(english_content) => {
            match fs::read_to_string(&output) {
                Ok(other_content) => {
                    // box not used for anything rn
                    let _ = merge_contents(output, english_content, other_content);
                }
                Err(e) => {
                    eprintln!("Failed to read english content: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("Unable to read input file: {}", e);
        }
    }
}

fn merge_contents(output: &Path, english_content: String, other_content: String)
                  -> Result<(), Box<dyn std::error::Error>> {
    let english: Value = serde_json::from_str(&english_content)?;
    let other: Value = serde_json::from_str(&other_content)?;

    let english_obj = english.as_object()
        .ok_or("Input JSON is not an object")?;
    let other_obj = other.as_object()
        .ok_or("Output JSON is not an object")?;

    println!("Keys in input file but missing in output file:");
    let mut added_count = 0;
    let mut missing_keys = Vec::new();

    for (key, value) in english_obj {
        if !other_obj.contains_key(key) {
            println!(" - Adding \"{}\": {:?}", key, value);
            missing_keys.push((key.clone(), value.clone()));
            added_count += 1;
        }
    }

    let mut updated_other = Map::new();

    for (key, value) in other_obj {
        updated_other.insert(key.clone(), value.clone());
    }

    for (key, value) in missing_keys {
        updated_other.insert(key, value);
    }

    if added_count == 0 {
        println!("No missing keys!");
        return Ok(())
    }

    let mut updated_other = Map::new();
    let mut credits_value = None;
    let mut texture_id_value = None;

    for (key, value) in other_obj {
        if key == "credits" {
            credits_value = Some(value.clone());
        } else if key == "textureID" {
            texture_id_value = Some(value.clone());
        } else {
            updated_other.insert(key.clone(), value.clone());
        }
    }

    for (key, value) in english_obj {
        if key != "credits" && key != "textureID" && !other_obj.contains_key(key) {
            updated_other.insert(key.clone(), value.clone());
        }
    }

    if texture_id_value.is_none() {
        if let Some(texture_id) = english_obj.get("textureID") {
            texture_id_value = Some(texture_id.clone());
        }
    }

    if credits_value.is_none() {
        if let Some(credits) = english_obj.get("credits") {
            credits_value = Some(credits.clone());
        }
    }

    if let Some(texture_id) = texture_id_value {
        updated_other.insert("textureID".to_string(), texture_id);
    }

    if let Some(credits) = credits_value {
        updated_other.insert("credits".to_string(), credits);
    }

    println!("Adding {} missing keys to output file from input file", added_count);
    let updated_content = serde_json::to_string_pretty(&updated_other)?;
    fs::write(output, updated_content)?;
    Ok(())
}