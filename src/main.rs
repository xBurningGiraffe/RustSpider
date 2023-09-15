use anyhow::Result;
use clap::{Arg, App};
use std::time::Instant;
use std::path::PathBuf;
use colored::*;
use tokio::task;

mod requester;
mod extractor;
mod save_it;
mod helper;

use requester::connector;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("{}", r#"

    ██████╗ ██╗   ██╗███████╗████████╗███████╗██████╗ ██╗██████╗ ███████╗██████╗ 
    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔════╝██╔══██╗██║██╔══██╗██╔════╝██╔══██╗
    ██████╔╝██║   ██║███████╗   ██║   ███████╗██████╔╝██║██║  ██║█████╗  ██████╔╝
    ██╔══██╗██║   ██║╚════██║   ██║   ╚════██║██╔═══╝ ██║██║  ██║██╔══╝  ██╔══██╗
    ██║  ██║╚██████╔╝███████║   ██║   ███████║██║     ██║██████╔╝███████╗██║  ██║
    Made by xBurningGiraffe

    "#.green());

    // let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Unable to determine the user's home directory."))?;

    // let output_folder: PathBuf = home_dir.join("RustSpider");

    let start_time = Instant::now();

    let matches = App::new("RustSpider")
    .version("1.0")
    .author("Your Name")
    .about("A parameter discovery suite")
    .arg(Arg::with_name("domain")
        .short("d")
        .long("domain")
        .value_name("DOMAIN")
        .help("Domain name of the target [ex: hackerone.com]")
        .takes_value(true))
    .arg(Arg::with_name("subs")
        .short("s")
        .long("subs")
        .value_name("SUBS")
        .help("Set False for no subs [ex: --subs False]")
        .takes_value(true)
        .default_value("True"))
    .arg(Arg::with_name("level")
        .short("l")
        .long("level")
        .value_name("LEVEL")
        .help("For nested parameters [ex: --level high]")
        .takes_value(true)
        .default_value("medium"))
    .arg(Arg::with_name("exclude")
        .short("e")
        .long("exclude")
        .value_name("EXCLUDE")
        .help("Extensions to exclude [ex: --exclude php,aspx]")
        .takes_value(true))
    .arg(Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("OUTPUT")
        .help("Output file name or full path [ex: output.txt or /path/to/output.txt]")
        .takes_value(true))
    .arg(Arg::with_name("placeholder")
        .short("p")
        .long("placeholder")
        .value_name("PLACEHOLDER")
        .help("The string to add as a placeholder after the parameter name.")
        .takes_value(true)
        .default_value("FUZZ"))
    .arg(Arg::with_name("help")
        .short("h")
        .long("help")
        .help("Displays this help menu"))
    .arg(Arg::with_name("quiet")
        .short("q")
        .long("quiet")
        .help("Quiet mode, no output to terminal")
        .takes_value(false))
    .arg(Arg::with_name("rate_limit")
        .short("r")
        .long("rate_limit")
        .value_name("RATE")
        .help("Rate limit in milliseconds between requests")
        .takes_value(true)
        .default_value("1000"))
    
    .get_matches();

    // After the help check
    if matches.is_present("help") {
        helper::display_help_menu();
        return Ok(());
    }

    let domain = matches.value_of("domain").ok_or_else(|| anyhow::anyhow!("Domain not provided"))?;
    let subs = matches.value_of("subs").unwrap_or("True");
    let level = matches.value_of("level").unwrap_or("medium");
    let exclude = matches.value_of("exclude").unwrap_or("");
    let output = matches.value_of("output");
    let placeholder = matches.value_of("placeholder").unwrap_or("FUZZ");
    let quiet = matches.is_present("quiet");
    let rate_limit = matches.value_of("rate_limit").unwrap_or("1000").parse::<u64>()?;


    // Define mutable final_uris

    let output_path = match output {
        Some(outfile) => {
            if outfile.contains("/") {
                PathBuf::from(outfile)
            } else {
                PathBuf::from(format!("output/{}", outfile))
            }
        },
        None => PathBuf::from(format!("output/{}.txt", domain)),
    };
    let mut final_uris: Vec<String> = Vec::new();

    // Define the closure
    let collect_and_print = |uri: String| {
        if !quiet {
            println!("{}", uri);
        }
        final_uris.push(uri);
    };

    let url = if subs == "True" {
        format!("https://web.archive.org/cdx/search/cdx?url=*.{}/&output=txt&fl=original&collapse=urlkey&page=/", domain)
    } else {
        format!("https://web.archive.org/cdx/search/cdx?url={}/&output=txt&fl=original&collapse=urlkey&page=/", domain)
    };

    let black_list: Vec<String> = if !exclude.is_empty() {
        exclude.split(",").map(|s| format!(".{}", s)).collect()
    } else {
        Vec::new()
    };

    let response: reqwest::Response = task::spawn(connector(url.clone(), rate_limit)).await??;
    let response_text: String = response.text().await?;
    

    // Call param_extract
    extractor::param_extract(&response_text, &black_list, placeholder, level, collect_and_print);

    // At this point, you can call save_func to save the data
    if let Err(err) = save_it::save_func(&final_uris, &output_path.to_string_lossy().into_owned(), domain) {
        println!("Failed to save data: {}", err);
    }

    println!("Total execution time: {}s", start_time.elapsed().as_secs());

    if final_uris.is_empty() {
        println!("No URLs with parameters found.");
    } else {
        println!("Unique URLs with parameters:");
        for uri in &final_uris {
            println!("{}", uri);
        }
    }

    if let Some(out_file) = output_path.to_str() {
        println!("Output is saved here: {}", out_file);
    } else {
        println!("Output is saved here: output/{}.txt", domain);
    }

    Ok(())

}