use anyhow::Result;
use std::time::Instant;

mod requester;
mod extractor;
mod save_it;
mod helper;
mod args;
mod banner;
mod reporter;
mod processor;
mod common_types;

use processor::process_url;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Display the application banner
    banner::display_banner();

    // Parse and validate command-line arguments
    let parsed_args_opt = args::get_parsed_args()?;
    if let Some(parsed_args) = parsed_args_opt {
        // Determines output file path
        let output_path = save_it::determine_output_path(&parsed_args.domain, parsed_args.output.as_deref());
        
        // Start time for performance measurement
        let start_time = Instant::now();

        // Final URIs list initialization
        let mut final_uris: Vec<String> = Vec::new();

        // Defining printing URIs and processing closures
        let collect_and_print = |uri: String| {
            if !parsed_args.quiet {
                println!("{}", uri);
            }
            final_uris.push(uri);
        };

        // URL processing and collecting results
        process_url(&parsed_args, collect_and_print).await?;

        // Save the collected URIs to a file
        if let Err(err) = save_it::save_func(&final_uris, &output_path.to_string_lossy().into_owned(), &parsed_args.domain) {
            println!("Failed to save data: {}", err);
        }

        // Display summary and results
        reporter::display_results(&final_uris, &start_time, &output_path, &parsed_args.domain);

        Ok(())

    } else {
        // Displays help menu and exits
        Ok(())
    }
}
