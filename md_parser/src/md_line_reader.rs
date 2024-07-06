#[derive(Debug, Clone)]
pub(crate) enum MdRawLine {
    /// Any line which starts with # will be transfered here.
    Head(String),
    /// Line which starts with `>`
    Quote(String),
    /// Line with `n. `
    OList(String),
    /// Line with `- ` TODO: add for "* " "+ "
    UList(String),
    /// Line starting with !
    Image(String),
    /// Table lines start with |
    Table(String),
    /// make code skip until finds another `CodeEnd`
    CodeBlock,
    /// starts with `: `
    Definition(String),
    /// starts with `- [X] ` or `- [ ]`
    TaskLine(String),
    /// start with `\t`
    TabbedLine(String),
    HR,
    Text(String),
    /// start with '\n'
    EmptyLine,
}

pub fn to_mdline(line: String) -> MdRawLine {
    if line.starts_with("#") {
        MdRawLine::Head(line.clone())
    } else if line.starts_with("> ") {
        MdRawLine::Quote(line)
    } else if line.starts_with("- [ ] ") || line.starts_with("- [X] ") {
        MdRawLine::TaskLine(line)
    } else if starts_with_ordered_list_pattern(&line) {
        MdRawLine::OList(line)
    } else if line.starts_with("- ") {
        MdRawLine::UList(line)
    } else if line.starts_with("---") {
        MdRawLine::HR
    } else if line.starts_with("![") {
        MdRawLine::Image(line)
    } else if line.starts_with("|") {
        MdRawLine::Table(line)
    } else if line.starts_with("```") {
        MdRawLine::CodeBlock
    } else if line.starts_with(": ") {
        MdRawLine::Definition(line)
    } else if line.starts_with('\t') {
        MdRawLine::TabbedLine(line)
    } else if line.starts_with("\n") {
        MdRawLine::EmptyLine
    } else {
        MdRawLine::Text(line.clone())
    }
}

pub fn to_mdlines(lines: Vec<String>) -> Vec<MdRawLine> {
    lines
        .into_iter()
        .map(|line| to_mdline(line))
        .collect::<Vec<MdRawLine>>()
}

fn starts_with_ordered_list_pattern(line: &String) -> bool {
    let mut divs = line.split('.');
    let before_dots = divs.next().unwrap();
    if before_dots.parse::<u32>().is_ok() {
        return true;
    }
    false
}

#[test]
fn test_ordered_list_check() {
    assert!(starts_with_ordered_list_pattern(&String::from("1. jsdf")));
    assert!(!starts_with_ordered_list_pattern(&String::from(" 1jsdf")));
    assert!(!starts_with_ordered_list_pattern(&String::from(" 1jsdf.")));
    assert!(starts_with_ordered_list_pattern(&String::from("1.jsdf.")));
    assert!(starts_with_ordered_list_pattern(&String::from("1. ")));
}
