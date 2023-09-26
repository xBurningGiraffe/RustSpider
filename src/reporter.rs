// Reporter module for displaying results

use std::path::Path;
use std::time::Instant;

// Display final results and metrics
pub fn display_results(
    final_uris: &[String],  // Final list of URIs
    start_time: &Instant,   // Start time for performance measurement
    output_path: &Path,     // Output file path
    domain: &str,           // Target domain
) {
    // Display total execution time
    println!("Total execution time: {}s", start_time.elapsed().as_secs());

    // Check for discovered URLs
    if final_uris.is_empty() {
        println!("No URLs with parameters found.");
    } else {
        println!("Unique URLs with parameters:");
        for uri in final_uris {
            println!("{}", uri);
        }
    }

    // Display output location
    if let Some(out_file) = output_path.to_str() {
        println!("Output is saved here: {}", out_file);
    } else {
        println!("Output is saved here: output/{}.txt", domain);
    }
}