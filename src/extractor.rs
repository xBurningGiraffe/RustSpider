use regex::Regex;
use std::collections::HashSet;

pub fn param_extract<F>(
    response: &str,
    black_list: &[String],
    placeholder: &str,
    level: &str,
    mut f: F,
) where
    F: FnMut(String),
{
    let re = Regex::new(r".*?:\/\/.*\?.*\=[^$]").unwrap();
    let parsed: HashSet<_> = re.captures_iter(response).map(|cap| cap[0].to_string()).collect();
    let mut final_uris = HashSet::new();

    for i in parsed {
        let delim = i.find('=').unwrap_or(0);
        let second_delim = i[(delim + 1)..].find('=').unwrap_or(0) + delim + 1;

        let uri_blacklisted = black_list.iter().any(|ext| i.contains(ext));

        if !uri_blacklisted {
            let first_param = format!("{}{}", &i[..delim + 1], placeholder);
            final_uris.insert(first_param.clone());
            f(first_param); // Print or save the first parameter immediately
            
            if level == "high" {
                let second_param = format!("{}{}", &i[..second_delim + 1], placeholder);
                final_uris.insert(second_param.clone());
                f(second_param); // Print or save the second parameter immediately
            }
        }
    }
}
