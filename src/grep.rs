use std::{fs::OpenOptions, io::{BufReader, BufRead}};

use anyhow::Error;
use clap::{App, Arg};

const LINE_NUMBER: &str = "line_number";
const FILE_NAME_ONLY: &str = "file_name_only";
const CASE_INSENSITIVE: &str = "case_insensitive";
const INVERT_MODE: &str = "invert_mode";
const ENTIRE_LINE_ONLY: &str = "entire_line_only";

const FILES_TO_SEARCH: &str = "files_to_search";
const PATTERN_TO_SEARCH: &str = "pattern";

#[derive(Debug)]
pub struct Flags {
    pub show_line_number: bool,// Print the line numbers of each matching line
    pub file_name_only: bool,// Print only the names of files that contain at least one matching line.
    pub case_insensitie: bool,// Match line using a case-insensitive comparison.
    pub invert_mode: bool,// Invert the program -- collect all lines that fail to match the pattern.
    pub entire_line_only: bool,// Only match entire lines, instead of lines that contain a match.
    pub pattern: String,// pattern to search
    pub files: Vec<String>,// files to search
}

impl Flags {
    pub fn new() -> Self {
        let mut flags = Flags {
            show_line_number: false,
            file_name_only: false,
            case_insensitie: false,
            invert_mode: false,
            entire_line_only: false,
            pattern: String::new(),
            files: Vec::new(),
        };
        get_args(&mut flags);
        flags
    }    
}

fn get_args(flags: &mut Flags) {
    let args = App::new("grep")
        .arg(Arg::with_name(LINE_NUMBER).short("-n"))
        .arg(Arg::with_name(FILE_NAME_ONLY).short("-l"))
        .arg(Arg::with_name(CASE_INSENSITIVE).short("-i"))
        .arg(Arg::with_name(INVERT_MODE).short("-v"))
        .arg(Arg::with_name(ENTIRE_LINE_ONLY).short("-x"))
        .arg(
            Arg::with_name(PATTERN_TO_SEARCH)
                .short("-p")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name(FILES_TO_SEARCH)
                .short("-f")
                .required(true)
                .index(2)
                .multiple(true),
        )
        .get_matches();

    let mut show_line_number = false;
    if args.is_present(LINE_NUMBER) {
        show_line_number = true;
    }

    let mut file_name_only = false;
    if args.is_present(FILE_NAME_ONLY) {
        file_name_only = true;
    }

    let mut case_insensitie = false;
    if args.is_present(CASE_INSENSITIVE) {
        case_insensitie = true;
    }

    let mut invert_mode = false;
    if args.is_present(INVERT_MODE) {
        invert_mode = true;
    }

    let mut entire_line_only = false;
    if args.is_present(ENTIRE_LINE_ONLY) {
        entire_line_only = true;
    }

    let pattern = args.value_of(PATTERN_TO_SEARCH).unwrap();

    let mut files_to_search = Vec::new();
    let files = args.values_of(FILES_TO_SEARCH).unwrap();
    for f in files {
        files_to_search.push(f.to_string());
    }

    flags.show_line_number = show_line_number;
    flags.file_name_only = file_name_only;
    flags.case_insensitie = case_insensitie;
    flags.invert_mode = invert_mode;
    flags.entire_line_only = entire_line_only;
    flags.pattern = pattern.to_string();
    flags.files = files_to_search;
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();

    for &f in files.iter() {
        let ret = handle_file(f, pattern, flags);
        if ret.is_ok() {
            let mut items = ret.unwrap();
            result.append(&mut items);
        }
    }
    return Ok(result);
}

fn handle_file(path: &str, _pattern: &str, flags: &Flags) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();

    let pattern = if flags.case_insensitie {
        _pattern.to_ascii_lowercase()
    } else {
        _pattern.to_string()
    };

    let ret = OpenOptions::new().read(true).open(path);
    if ret.is_ok() {
        let file = ret.unwrap();
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut line_number = 0;

        while let Ok(ret) = reader.read_line(&mut line) {
            if ret == 0 {
                break;
            }

            line = line.trim_end_matches('\n').to_string();
            line_number += 1;
            let mut hit = false;

            if flags.case_insensitie {
                if flags.entire_line_only {
                    if line.eq_ignore_ascii_case(&pattern) {
                        hit = true;
                    }
                } else {
                    let s = line.to_ascii_lowercase();
                    if s.contains(&pattern) {
                        hit = true;
                    }
                }
            } else {
                if flags.entire_line_only {
                    if line.eq(&pattern) {
                        hit = true;
                    }
                } else {
                    if line.contains(&pattern) {
                        hit = true;
                    }
                }
            }

            if hit {
                if flags.file_name_only {
                    result.push(path.to_string());
                    break;
                }

                let mut final_s = line.clone();
                if flags.show_line_number {
                    let mut s = line_number.to_string();
                    s.push(':');
                    s.push_str(&line);
                    final_s = s;
                } 
                result.push(final_s);
            } else {
                if flags.invert_mode {
                    result.push(line.clone());                    
                }
            }
            line.clear();
        }
    }

    return Ok(result);
}