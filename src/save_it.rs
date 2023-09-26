// Import required std libraries
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub fn determine_output_path(domain: &str, output: Option<&str>) -> PathBuf {
    match output {
        Some(outfile) => {
            // Use provided output path
            if outfile.contains("/") {
                PathBuf::from(outfile)
            } else {
                PathBuf::from(format!("output/{}", outfile))
            }
        },
        None => {
            // Default to domain name
            PathBuf::from(format!("output/{}.txt", domain))
        },
    }
}

// Save final URIs
pub fn save_func(final_uris: &Vec<String>, outfile: &str, domain: &str) -> std::io::Result<()> {
    // Determine filename
    let filename = if !outfile.is_empty() {
        if outfile.contains("/") {
            outfile.to_string()
        } else {
            format!("output/{}", outfile)
        }
    } else {
        format!("output/{}.txt", domain)
    };
    
    // Creates path object
    let path = Path::new(&filename);
    
    // Existing file check & removal
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    
    // Create parent directory
    let parent_dir = path.parent().unwrap();
    std::fs::create_dir_all(parent_dir)?;
    
    // Open file to write
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
        
    // Write URIs to file
    for uri in final_uris {
        writeln!(file, "{}", uri)?;
    }
    
    Ok(())
}
