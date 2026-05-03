use std::collections::BTreeMap;

use crate::error::QuicktermError;

pub fn env_map() -> BTreeMap<String, String> {
    std::env::vars()
        .map(|(k, v)| (format!("${k}"), v))
        .collect()
}

pub fn expand_template(
    template: &str,
    replacements: &BTreeMap<String, String>,
) -> Result<String, QuicktermError> {
    let mut out = String::with_capacity(template.len());
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '{' {
            if let Some(rel_end) = chars[i + 1..].iter().position(|ch| *ch == '}') {
                let end = i + 1 + rel_end;
                let key: String = chars[i + 1..end].iter().collect();
                let value = replacements.get(&key).ok_or_else(|| {
                    QuicktermError::Expansion(format!("missing placeholder {{{key}}}"))
                })?;
                out.push_str(value);
                i = end + 1;
                continue;
            }
        }

        out.push(chars[i]);
        i += 1;
    }

    Ok(out)
}

pub fn expand_string(
    template: &str,
    replacements: &BTreeMap<String, String>,
) -> Result<String, QuicktermError> {
    expand_template(template, replacements)
}

pub fn expand_command(
    template: &str,
    replacements: &BTreeMap<String, String>,
) -> Result<Vec<String>, QuicktermError> {
    let expanded = expand_template(template, replacements)?;
    shlex::split(&expanded)
        .ok_or_else(|| QuicktermError::Expansion(format!("could not parse command: {expanded}")))
}
