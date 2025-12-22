// Path Utilities
//
// Path manipulation utilities.

/// Check if identifier is valid.
pub fn is_valid_identifier(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    
    let first = name.chars().next().unwrap();
    if !first.is_alphabetic() && first != '_' && first != '$' {
        return false;
    }
    
    name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}

/// Convert to camel case.
pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for c in input.chars() {
        if c == '-' || c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert to kebab case.
pub fn to_kebab_case(input: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                result.push('-');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert to pascal case.
pub fn to_pascal_case(input: &str) -> String {
    let camel = to_camel_case(input);
    let mut chars: Vec<char> = camel.chars().collect();
    if !chars.is_empty() {
        chars[0] = chars[0].to_ascii_uppercase();
    }
    chars.into_iter().collect()
}

/// Get basename from path.
pub fn get_basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

/// Get directory name from path.
pub fn get_dirname(path: &str) -> &str {
    match path.rfind('/') {
        Some(pos) => &path[..pos],
        None => ".",
    }
}

/// Get file extension.
pub fn get_extension(path: &str) -> Option<&str> {
    let basename = get_basename(path);
    basename.rfind('.').map(|pos| &basename[pos + 1..])
}

/// Remove file extension.
pub fn remove_extension(path: &str) -> &str {
    match path.rfind('.') {
        Some(pos) => &path[..pos],
        None => path,
    }
}
