use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const USAGE: &str = "Usage:
  Helper <english.properties> <target.properties>
  Helper --all <english.properties> [directory]

Examples:
  Helper English.properties German.properties
  Helper --all English.properties
  Helper --all English.properties translations/

The --all mode updates every .properties file in the selected directory and
skips the English file. If directory is omitted, the English file's directory
is used. --check-all and check-all are aliases for --all.";

#[derive(Debug, Clone)]
struct PropertyEntry {
    key: Option<String>,
    raw: String,
}

#[derive(Debug, Default)]
struct SyncResult {
    content: String,
    added: Vec<String>,
    removed: Vec<String>,
}

impl SyncResult {
    fn changed(&self, original: &str) -> bool {
        self.content != original
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!("{USAGE}");
        return;
    }

    if let Err(error) = run(&args) {
        eprintln!("Error: {error}");
        eprintln!("\n{USAGE}");
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err(Box::new(invalid_input("missing arguments")));
    }

    if is_all_mode(args[0].as_str()) {
        if args.len() < 2 || args.len() > 3 {
            return Err(Box::new(invalid_input(
                "--all expects an English file and an optional directory",
            )));
        }

        let english_path = Path::new(&args[1]);
        let directory = args
            .get(2)
            .map(PathBuf::from)
            .or_else(|| english_path.parent().map(Path::to_path_buf))
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| PathBuf::from("."));

        update_all(english_path, &directory)
    } else {
        if args.len() != 2 {
            return Err(Box::new(invalid_input(
                "expected an English file and a target file",
            )));
        }

        update_file(Path::new(&args[0]), Path::new(&args[1]))
    }
}

fn is_all_mode(argument: &str) -> bool {
    matches!(argument, "--all" | "--check-all" | "check-all")
}

fn update_file(english_path: &Path, target_path: &Path) -> Result<(), Box<dyn Error>> {
    let english_content = read_file(english_path)?;

    if !target_path.exists() {
        fs::copy(english_path, target_path).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("unable to create {}: {error}", target_path.display()),
            )
        })?;
        println!(
            "Created {} from {}",
            target_path.display(),
            english_path.display()
        );
        return Ok(());
    }

    let target_content = read_file(target_path)?;
    let result = synchronize(&english_content, &target_content);
    write_result(target_path, &target_content, result)
}

fn update_all(english_path: &Path, directory: &Path) -> Result<(), Box<dyn Error>> {
    let english_content = read_file(english_path)?;
    let english_path = fs::canonicalize(english_path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("unable to resolve {}: {error}", english_path.display()),
        )
    })?;

    if !directory.is_dir() {
        return Err(Box::new(invalid_input(format!(
            "{} is not a directory",
            directory.display()
        ))));
    }

    let mut target_paths = Vec::new();
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();

        if !path.is_file() || !is_properties_file(&path) {
            continue;
        }

        let resolved_path = fs::canonicalize(&path)?;
        if resolved_path != english_path {
            target_paths.push(path);
        }
    }

    target_paths.sort();

    if target_paths.is_empty() {
        println!(
            "No .properties target files found in {}",
            directory.display()
        );
        return Ok(());
    }

    for target_path in target_paths {
        let target_content = read_file(&target_path)?;
        let result = synchronize(&english_content, &target_content);
        write_result(&target_path, &target_content, result)?;
    }

    Ok(())
}

fn write_result(
    target_path: &Path,
    original: &str,
    result: SyncResult,
) -> Result<(), Box<dyn Error>> {
    if !result.changed(original) {
        println!("{} is already up to date", target_path.display());
        return Ok(());
    }

    let added_count = result.added.len();
    let removed_count = result.removed.len();
    fs::write(target_path, result.content)?;

    println!(
        "Updated {} (added {}, removed {})",
        target_path.display(),
        added_count,
        removed_count
    );

    if added_count > 0 {
        println!("  added: {}", result.added.join(", "));
    }
    if removed_count > 0 {
        println!("  removed: {}", result.removed.join(", "));
    }

    Ok(())
}

fn read_file(path: &Path) -> Result<String, io::Error> {
    fs::read_to_string(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("unable to read {}: {error}", path.display()),
        )
    })
}

fn is_properties_file(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map(|extension| extension.eq_ignore_ascii_case("properties"))
        .unwrap_or(false)
}

fn synchronize(english_content: &str, target_content: &str) -> SyncResult {
    let english_entries = parse_properties(english_content);
    synchronize_entries(&english_entries, english_content, target_content)
}

fn synchronize_entries(
    english_entries: &[PropertyEntry],
    english_content: &str,
    target_content: &str,
) -> SyncResult {
    let target_entries = parse_properties(target_content);
    let english_keys: HashSet<&str> = english_entries
        .iter()
        .filter_map(|entry| entry.key.as_deref())
        .collect();
    let target_keys: HashSet<&str> = target_entries
        .iter()
        .filter_map(|entry| entry.key.as_deref())
        .collect();
    let (english_comments, trailing_english_comments) = comments_by_key(english_entries);
    let newline = preferred_newline(target_content, english_content);

    let mut removed = Vec::new();
    let mut removed_keys = HashSet::new();
    let mut content = String::new();

    for (target_index, entry) in target_entries.iter().enumerate() {
        let should_remove = entry
            .key
            .as_deref()
            .map(|key| !english_keys.contains(key))
            .unwrap_or(false);

        if should_remove {
            if let Some(key) = &entry.key {
                if removed_keys.insert(key.clone()) {
                    removed.push(key.clone());
                }
            }
        } else {
            if let Some(key) = entry.key.as_deref() {
                if let Some(comments) = english_comments.get(key) {
                    let comments_to_add: Vec<String> = comments
                        .iter()
                        .filter(|comment| {
                            !has_comment_before_key(&target_entries, target_index, comment)
                        })
                        .cloned()
                        .collect();
                    append_comments(&mut content, &comments_to_add, newline);
                }
            }
            content.push_str(&entry.raw);
        }
    }

    let mut added = Vec::new();
    let mut added_keys = HashSet::new();

    for entry in english_entries {
        let Some(key) = entry.key.as_deref() else {
            continue;
        };

        if target_keys.contains(key) || !added_keys.insert(key) {
            continue;
        }

        if let Some(comments) = english_comments.get(key) {
            append_comments(&mut content, comments, newline);
        }
        append_source_entry(&mut content, &entry.raw, newline);
        added.push(key.to_string());
    }

    for comment in &trailing_english_comments {
        if !has_trailing_comment(&target_entries, comment) {
            append_source_entry(&mut content, comment, newline);
        }
    }

    SyncResult {
        content,
        added,
        removed,
    }
}

fn comments_by_key(entries: &[PropertyEntry]) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let mut comments = HashMap::new();
    let mut pending_comments = Vec::new();

    for entry in entries {
        if let Some(key) = &entry.key {
            if !pending_comments.is_empty() {
                comments.insert(key.clone(), std::mem::take(&mut pending_comments));
            }
        } else if is_comment_entry(entry) {
            pending_comments.push(entry.raw.clone());
        }
    }

    (comments, pending_comments)
}

fn append_comments(content: &mut String, comments: &[String], newline: &str) {
    for comment in comments {
        append_source_entry(content, comment, newline);
    }
}

fn has_comment_before_key(
    entries: &[PropertyEntry],
    key_index: usize,
    source_comment: &str,
) -> bool {
    let source_signature = comment_signature(source_comment);

    for entry in entries[..key_index].iter().rev() {
        if entry.key.is_some() {
            break;
        } else if is_comment_entry(entry) {
            if comment_signature(&entry.raw) == source_signature {
                return true;
            }
        }
    }

    false
}

fn has_trailing_comment(entries: &[PropertyEntry], source_comment: &str) -> bool {
    let source_signature = comment_signature(source_comment);

    for entry in entries.iter().rev() {
        if entry.key.is_some() {
            break;
        } else if is_comment_entry(entry) && comment_signature(&entry.raw) == source_signature {
            return true;
        }
    }

    false
}

fn append_source_entry(content: &mut String, raw: &str, newline: &str) {
    if !content.is_empty() && !ends_with_line_ending(content) {
        content.push_str(newline);
    }
    content.push_str(&normalize_line_endings(raw, newline));
}

fn is_comment_entry(entry: &PropertyEntry) -> bool {
    let line = strip_line_ending(&entry.raw);
    let line = trim_property_whitespace_start(line);
    line.starts_with('#') || line.starts_with('!')
}

fn comment_signature(comment: &str) -> String {
    normalize_line_endings(comment, "\n")
        .lines()
        .map(|line| trim_property_whitespace_start(line).trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_properties(content: &str) -> Vec<PropertyEntry> {
    if content.is_empty() {
        return Vec::new();
    }

    let physical_lines: Vec<&str> = content.split_inclusive('\n').collect();
    let mut entries = Vec::new();
    let mut index = 0;

    while index < physical_lines.len() {
        let start = index;
        let first_line = strip_line_ending(physical_lines[index]);
        let mut end = index + 1;

        if !is_ignored_line(first_line) {
            while end < physical_lines.len()
                && has_continuation(strip_line_ending(physical_lines[end - 1]))
            {
                end += 1;
            }
        }

        let raw = physical_lines[start..end].concat();
        let logical_line = logical_line(&physical_lines[start..end]);
        let key = parse_property_key(&logical_line);

        entries.push(PropertyEntry { key, raw });
        index = end;
    }

    entries
}

fn logical_line(lines: &[&str]) -> String {
    let mut logical = String::new();

    for (index, line) in lines.iter().enumerate() {
        let mut part = strip_line_ending(line);

        if index > 0 {
            part = trim_property_whitespace_start(part);
        }

        if index + 1 < lines.len() && has_continuation(part) {
            part = &part[..part.len() - 1];
        }

        logical.push_str(part);
    }

    logical
}

fn parse_property_key(line: &str) -> Option<String> {
    let line = trim_property_whitespace_start(line);
    if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
        return None;
    }

    let mut key = String::new();
    let mut escaped = false;

    for character in line.chars() {
        if escaped {
            key.push('\\');
            key.push(character);
            escaped = false;
            continue;
        }

        if character == '\\' {
            escaped = true;
            key.push(character);
            continue;
        }

        if character == '=' || character == ':' || is_property_whitespace(character) {
            break;
        }

        key.push(character);
    }

    Some(unescape_property_key(&key))
}

fn unescape_property_key(raw_key: &str) -> String {
    let characters: Vec<char> = raw_key.chars().collect();
    let mut key = String::new();
    let mut index = 0;

    while index < characters.len() {
        if characters[index] != '\\' {
            key.push(characters[index]);
            index += 1;
            continue;
        }

        if index + 1 >= characters.len() {
            key.push('\\');
            break;
        }

        let escaped = characters[index + 1];
        match escaped {
            't' => key.push('\t'),
            'r' => key.push('\r'),
            'n' => key.push('\n'),
            'f' => key.push('\u{000c}'),
            'u' if index + 5 < characters.len() => {
                let digits: String = characters[index + 2..index + 6].iter().collect();
                if let Ok(code_point) = u32::from_str_radix(&digits, 16) {
                    if let Some(character) = char::from_u32(code_point) {
                        key.push(character);
                        index += 6;
                        continue;
                    }
                }
                key.push('u');
            }
            other => key.push(other),
        }

        index += 2;
    }

    key
}

fn is_ignored_line(line: &str) -> bool {
    let line = trim_property_whitespace_start(line);
    line.is_empty() || line.starts_with('#') || line.starts_with('!')
}

fn has_continuation(line: &str) -> bool {
    let mut slash_count = 0;

    for character in line.chars().rev() {
        if character != '\\' {
            break;
        }
        slash_count += 1;
    }

    slash_count % 2 == 1
}

fn is_property_whitespace(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\u{000c}')
}

fn trim_property_whitespace_start(value: &str) -> &str {
    value.trim_start_matches(is_property_whitespace)
}

fn strip_line_ending(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

fn preferred_newline(target: &str, english: &str) -> &'static str {
    if target.contains("\r\n") || (!target.contains('\n') && target.contains('\r')) {
        "\r\n"
    } else if target.contains('\n') {
        "\n"
    } else if english.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

fn ends_with_line_ending(value: &str) -> bool {
    value.ends_with('\n') || value.ends_with('\r')
}

fn normalize_line_endings(value: &str, newline: &str) -> String {
    let normalized = value.replace("\r\n", "\n").replace('\r', "\n");
    if newline == "\n" {
        normalized
    } else {
        normalized.replace('\n', newline)
    }
}

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}
