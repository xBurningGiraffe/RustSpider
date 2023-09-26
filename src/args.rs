use clap::{Arg, App, ArgMatches};
use anyhow::Result;
use crate::helper;
pub struct ParsedArgs {
    pub domain: String,
    pub subs: String,
    pub level: String,
    pub exclude: String,
    pub output: Option<String>,
    pub placeholder: String,
    pub quiet: bool,
    pub rate_limit: u64,
}

pub fn get_cli_args() -> ArgMatches<'static> {
    App::new("RustSpider")
    .version("1.0")
    .author("xBurningGiraffe")
    .about("A parameter discovery suite")
    .arg(Arg::with_name("domain")
        .short("d")
        .long("domain")
        .value_name("DOMAIN")
        .help("Domain name of the target [ex: hackerone.com]")
        .takes_value(true))
    .arg(Arg::with_name("url")
        .short("u")
        .long("url")
        .value_name("URL")
        .help("URL of the target [ex: https://example.com")
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
        .help("Sets the rate limit for requests per second")
        .takes_value(true)
        .default_value("20"))
    
    .get_matches()
}

pub fn parse_and_validate_args(matches: &ArgMatches) -> Result<ParsedArgs> {
    let domain = matches.value_of("domain")
        .ok_or_else(|| anyhow::anyhow!("Domain not provided"))?
        .to_string();  // Convert &str to String
    let subs = matches.value_of("subs").unwrap_or("True").to_string();
    let level = matches.value_of("level").unwrap_or("medium").to_string();
    let exclude = matches.value_of("exclude").unwrap_or("").to_string();
    let output = matches.value_of("output").map(|s| s.to_string());  // Convert Option<&str> to Option<String>
    let placeholder = matches.value_of("placeholder").unwrap_or("FUZZ").to_string();
    let quiet = matches.is_present("quiet");
    let rate_limit = matches.value_of("rate_limit").unwrap_or("20").parse::<u64>()?;

    Ok(ParsedArgs {
        domain,
        subs,
        level,
        exclude,
        output,  // This is now Option<String>
        placeholder,
        quiet,
        rate_limit,
    })
}

pub fn get_parsed_args() -> Result<Option<ParsedArgs>, anyhow::Error> {
    let matches = get_cli_args();
    if matches.is_present("help") {
        helper::display_help_menu();
        return Ok(None);  // This is fine now.
    }
    let parsed_args = parse_and_validate_args(&matches)?;
    Ok(Some(parsed_args))
}

