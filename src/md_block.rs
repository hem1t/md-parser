use crate::{md_inline_parser::md_string::MdString, md_line_purifier::PurifiedMdLine};

#[derive(Debug)]
pub(crate) enum TableRow {
    Heading,
    Data,
}

///
/// here we create blocks
///
/// blocks will hold multiple lines at once whereever the lines are related
/// blocks will also convert all the strings to MdString.
///
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
    Parah(Vec<MdString>),
    HR,
    EmptyLine,
}


///
/// takes `PurifiedMdLine`s and converts it to Vec of `MdBlock`
///
pub(crate) struct MdBlockParser {
    blocks: Vec<MdBlock>,
}

impl MdBlockParser {
    pub fn new() -> Self {
        MdBlockParser {
            blocks: Vec::new()
        }
    }

    pub fn parse(lines: Vec<PurifiedMdLine>) -> Self {
        unimplemented!()
    }
}
