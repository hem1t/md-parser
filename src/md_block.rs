use crate::md_inline_parser::MdString;

///
/// here we create blocks
///
/// blocks will hold multiple lines at once whereever the lines are related
/// blocks will also convert all the strings to MdString.
///

#[derive(Debug)]
pub(crate) enum TableRow {
    Heading,
    Data,
}

#[derive(Debug)]
pub(crate) enum MdBlock {
    Head {
        level: u8,
        id: String,
        data: MdString,
    },
    BlockQuote {
        data: Vec<Box<MdBlock>>,
    },
    OList {
        data: Vec<MdString>,
    },
    UList {
        data: Vec<MdString>,
    },
    Image {
        alt_text: String,
        link_text: String,
    },
    Table {
        data: Vec<(TableRow, Vec<MdString>)>
    },
    Definition {
        term: String,
        defs: Vec<MdString>,
    },
    TaskedLine {
        done: bool,
        tast_text: MdString,
    },
    CodeBlock(String),
    Parah(MdString),
    HR,
    EmptyLine,
}
