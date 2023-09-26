// Module to extract params from response text
use regex::Regex;
use std::collections::HashSet;

/// # Arguments
///
/// * `response` - The response text to be parsed for URIs.
/// * `black_list` - A list of URI extensions to be excluded.
/// * `placeholder` - The placeholder to append to the URI parameters.
/// * `level` - The level of nested parameters to process ('high' or 'medium').
/// * `f` - A function or closure to process the extracted URIs.

pub fn param_extract<F>(
    response: &str,
    black_list: &[String],
    placeholder: &str,
    level: &str,
    mut f: F,
)
where
    F: FnMut(String),
{
   // Regex for URI matching
    let re = Regex::new(r".*?:\/\/.*\?.*\=[^$]").unwrap();
    
    // Captures and stores URIs into a HashSet to remove avoid duplication
    let parsed: HashSet<_> = re.captures_iter(response).map(|cap| cap[0].to_string()).collect();
    
    // Initialize a HashSet to store the final processed URIs
    let mut final_uris = HashSet::new();

    // Loops through captured URIs and processes
    for i in parsed {
        // Find the position of the first and second '=' delimiter in the URI
        let delim = i.find('=').unwrap_or(0);
        let second_delim = i[(delim + 1)..].find('=').unwrap_or(0) + delim + 1;

        // Check if the current URI is blacklisted
        let uri_blacklisted = black_list.iter().any(|ext| i.contains(ext));

        // If the URI is not blacklisted, process it
        if !uri_blacklisted {
            // Append placeholder to the first parameter and store it
            let first_param = format!("{}{}", &i[..delim + 1], placeholder);
            final_uris.insert(first_param.clone());

            // Invoke the provided function/closure on the first parameter
            f(first_param);
            
            // If level is set to 'high', process the second parameter
            if level == "high" {
                // Append placeholder to the second parameter and store it
                let second_param = format!("{}{}", &i[..second_delim + 1], placeholder);
                final_uris.insert(second_param.clone());

                // Invoke the provided function/closure on the second parameter
                f(second_param);
            }
        }
    }
}
