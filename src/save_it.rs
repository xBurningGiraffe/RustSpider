use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn save_func(final_uris: &Vec<String>, outfile: &str, domain: &str) -> std::io::Result<()> {
    let filename = if !outfile.is_empty() {
        if outfile.contains("/") {
            outfile.to_string()
        } else {
            format!("output/{}", outfile)
        }
    } else {
        format!("output/{}.txt", domain)
    };
    
    let path = Path::new(&filename);
    
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    
    let parent_dir = path.parent().unwrap();
    std::fs::create_dir_all(parent_dir)?;
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
        
    for uri in final_uris {
        writeln!(file, "{}", uri)?;
    }
    
    Ok(())
}