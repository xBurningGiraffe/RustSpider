
use colored::*;

pub fn display_help_menu() {
    println!("{}", "=====================".bright_white());
    println!();
    println!("{}", "Usage:".bright_yellow());
    println!("  RustSpider [OPTIONS]");
    println!();
    println!("{}", "Options:".bright_yellow());
    println!("  -d, --domain DOMAIN     {}", "Domain name of the target [ex: hackerone.com]".bright_white());
    println!("  -s, --subs SUBS         {}", "Set False for no subs [ex: --subs False] (default: True)".bright_white());
    println!("  -l, --level LEVEL       {}", "For nested parameters [ex: --level high] (default: medium)".bright_white());
    println!("  -e, --exclude EXCLUDE   {}", "Extensions to exclude [ex: --exclude php,aspx]".bright_white());
    println!("  -o, --output OUTPUT     {}", "Output file name [by default it is 'domain.txt']".bright_white());
    println!("  -O, --output_path       {}", "Path to output the results".bright_white());
    println!("  -r, --rate_limit        {}", "Rate limit in milliseconds between requests".bright_white());
    println!("  -p, --placeholder PH    {}", "Placeholder string to add after the parameter name (default: FUZZ)".bright_white());
    println!("  -h, --help HELP         {}", "Print out this help menu".bright_white());
    println!();
    println!("{}", "Examples:".bright_yellow());
    println!("  RustSpider -d hackerone.com -s True -l high");
    println!("  RustSpider -d example.com -e php,aspx -o output.txt");
    println!();
}
