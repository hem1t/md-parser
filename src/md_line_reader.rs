use std::{fs::File, io::{BufRead, BufReader}, process::exit};

enum MdLine {
    /// Any line which starts with # will be transfered here.
    Hash(String),
    /// Line which starts with `>`
    Quote(String),
    /// Line with `n. `
    OList(String),
    /// Line with `- `
    UList(String),
    /// Line starting with !
    Image(String),
    /// Table lines start with |
    Table(String),
    /// make code skip until finds another `CodeEnd`
    CodeStart,
    CodeEnd,
    /// starts with `: `
    Definition(String),
    /// starts with `- [X] ` or `- [ ]`
    TaskLine(String),
    /// start with `\t` or 3 or more spaces
    TabbedLine(String),
    HR,
    Text(String),
}

struct MdLineReader {
    file: BufReader<File>
}

impl MdLineReader {
    pub fn to_mdlines(self) -> Vec<MdLine> {
        let mut in_code_block = false;

        self.file.buffer().lines().map(|line| {
            if let Ok(line) = line {
                if in_code_block {
                    // skip until not find `CodeEnd`
                    if line.starts_with("```") {
                        in_code_block = false;
                        MdLine::CodeEnd
                    } else {
                        MdLine::Text(line)
                    }
                }
                else if line.starts_with("#") {
                    MdLine::Hash(line.clone())
                }
                else if line.starts_with("> ") {
                    MdLine::Quote(line)
                }
                else if line.starts_with("- [ ] ") || line.starts_with("- [X] ") {
                    MdLine::TaskLine(line)
                }
                else if line.starts_with("- ") {
                    MdLine::UList(line)
                }
                else if line.starts_with("---") {
                    MdLine::HR
                }
                else if line.starts_with("!") {
                    MdLine::Image(line)
                }
                else if line.starts_with("|") {
                    MdLine::Table(line)
                }
                else if line.starts_with("```") {
                    in_code_block = true;
                    MdLine::CodeStart
                }
                else if line.starts_with(": ") {
                    MdLine::Definition(line)
                }
                else {
                    MdLine::Text(line.clone())
                }
            } else {
                eprintln!("Error occured while reading {:?}", line);
                exit(1);
            }
        }).collect::<Vec<MdLine>>()
    }
}

fn starts_with_ordered_list_pattern(line: &str) -> bool {
    unimplemented!();
}
