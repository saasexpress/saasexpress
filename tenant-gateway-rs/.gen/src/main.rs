use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Pattern {
    category: String,
    name: String,
    short_description: String,
    long_description: String,
    reference_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Patterns {
    patterns: Vec<Pattern>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the YAML file
    let yaml_content = fs::read_to_string("eip2.yaml")?;
    let patterns: Patterns = serde_yaml::from_str(&yaml_content)?;

    // Create a directory to store the categorized patterns
    let output_dir = "src/patterns_by_category";
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir)?;
    }

    println!("Pattern Count = {}", patterns.patterns.len());

    // Iterate through each pattern
    for pattern in patterns.patterns {
        // Create a directory for the category if it doesn't exist
        let category_dir = format!(
            "{}/{}",
            output_dir,
            pattern.category.replace(" ", "_").to_lowercase()
        );
        if !Path::new(&category_dir).exists() {
            fs::create_dir(&category_dir)?;
        }

        // Create a file for the pattern
        let file_name = format!(
            "{}/{}.rs",
            category_dir,
            pattern.name.replace(" ", "_").to_lowercase()
        );
        let mut file = File::create(&file_name)?;

        // Write the pattern details as comments in the file
        writeln!(file, "/*");
        writeln!(file, " * Pattern: {}", pattern.name)?;
        writeln!(file, " * Short Description: {}", pattern.short_description)?;
        writeln!(file, " * Description: {}", pattern.long_description)?;
        writeln!(file, " * Reference URL: {}", pattern.reference_url)?;
        writeln!(file, "*/");
        writeln!(file, "\n// Your Rust code for the pattern goes here...")?;
    }

    println!(
        "Pattern files generated successfully in the '{}' directory.",
        output_dir
    );
    Ok(())
}
