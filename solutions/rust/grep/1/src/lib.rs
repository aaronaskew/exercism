use anyhow::Error;
use std::io::Read;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug, Default)]
pub struct Flags {
    // -n Prepend the line number and a colon (':') to each line in the output, placing the number after the filename (if present).
    pub(crate) line_numbers: bool,
    // -l Output only the names of the files that contain at least one matching line.
    pub(crate) file_names_only: bool,
    // -i Match using a case-insensitive comparison.
    pub(crate) case_insensitive: bool,
    // -v Invert the program -- collect all lines that fail to match.
    pub(crate) invert: bool,
    // -x Search only for lines where the search string matches the entire line.
    pub(crate) match_entire_line_only: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut new_flags = Self::default();

        for &flag in flags {
            match flag {
                "-n" => new_flags.line_numbers = true,
                "-l" => new_flags.file_names_only = true,
                "-i" => new_flags.case_insensitive = true,
                "-v" => new_flags.invert = true,
                "-x" => new_flags.match_entire_line_only = true,
                _ => {
                    panic!("invalid flag")
                }
            }
        }

        new_flags
    }
}

#[derive(Debug)]
pub struct FileToSearch {
    file_name: String,
    contents: String,
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut files_to_search = vec![];

    for file_name in files {
        let mut file = std::fs::File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        files_to_search.push(FileToSearch {
            file_name: file_name.to_string(),
            contents,
        });
    }

    let prepend_file_names = files_to_search.len() > 1;

    let mut search_results = Vec::new();

    for file in files_to_search {
        for (i, line) in file.contents.lines().enumerate() {
            let mut line_matches = match (flags.case_insensitive, flags.match_entire_line_only) {
                (true, true) => line.to_uppercase() == pattern.to_uppercase(),
                (true, false) => line.to_uppercase().contains(&pattern.to_uppercase()),
                (false, true) => line == pattern,
                (false, false) => line.contains(pattern),
            };

            if flags.invert {
                line_matches = !line_matches;
            }

            if line_matches {
                let mut search_result_line = String::new();

                if flags.file_names_only {
                    search_results.push(file.file_name.clone());
                    break;
                }

                if prepend_file_names {
                    search_result_line.push_str(&format!("{}:", file.file_name));
                }

                if flags.line_numbers {
                    search_result_line.push_str(&format!("{}:", i + 1));
                }

                search_result_line.push_str(line);

                search_results.push(search_result_line);
            }
        }
    }

    Ok(search_results)
}
