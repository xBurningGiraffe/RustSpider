
use anyhow::Result;
use reqwest;
use tokio::task;
use std::collections::HashSet;
use std::num::NonZeroU32;

use crate::args::ParsedArgs;
use crate::requester::connector;
use crate::extractor::param_extract;
use crate::common_types::create_rate_limiter;


pub async fn process_url<F>(parsed_args: &ParsedArgs, f: F) -> Result<HashSet<String>>
where
    F: FnMut(String),  // Generic closure
{  
    // Defines target URL
    let url = if parsed_args.subs == "True" {
        format!("https://web.archive.org/cdx/search/cdx?url=*.{}/&output=txt&fl=original&collapse=urlkey&page=/", parsed_args.domain)
    } else {
        format!("https://web.archive.org/cdx/search/cdx?url={}/&output=txt&fl=original&collapse=urlkey&page=/", parsed_args.domain)
    };

    // Create the RateLimiter
    let rate = NonZeroU32::new(parsed_args.rate_limit as u32).unwrap();
    let rate_limiter = create_rate_limiter(rate);

    let rate_limiter_clone = rate_limiter.clone();

    // Spawn a new task for HTTP request
    let response: reqwest::Response = task::spawn(async move {
        connector(url.clone(), &rate_limiter_clone).await
    }).await??;

    
    // creates blacklist from parsing args
    let black_list: Vec<String> = if !parsed_args.exclude.is_empty() {
        parsed_args.exclude.split(",").map(|s| format!(".{}", s)).collect()
    } else {
        Vec::new()
    };

    // Extract response    
    let response_text: String = response.text().await?;

    let final_uris = HashSet::new();

    // Extracting params from response
    param_extract(&response_text, &black_list, &parsed_args.placeholder, &parsed_args.level, f);

    Ok(final_uris)
}


