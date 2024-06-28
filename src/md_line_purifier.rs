use crate::md_line_reader::MdLine;

///
/// Here lies implimentations for MdLine
/// to refine itself and become PurifiedMdLine
///
/// After passing `MdLine` through here they will selected by their
/// validity and will be refined into furthere seperated data or else
/// they will drop to being a simple Text.
///

enum PurifiedMdLine {
    Head {
        title: String,
        level: u8,
        id: String,
    },
}

impl PurifiedMdLine {
    pub fn purify(&self, md_line: MdLine) -> PurifiedMdLine {
        match md_line {
            MdLine::Head(s) => self.purify_head(s),
            MdLine::Quote(_) => todo!(),
            MdLine::OList(_) => todo!(),
            MdLine::UList(_) => todo!(),
            MdLine::Image(_) => todo!(),
            MdLine::Table(_) => todo!(),
            MdLine::CodeStart => todo!(),
            MdLine::CodeEnd => todo!(),
            MdLine::Definition(_) => todo!(),
            MdLine::TaskLine(_) => todo!(),
            MdLine::TabbedLine(_) => todo!(),
            MdLine::HR => todo!(),
            MdLine::Text(_) => todo!(),
            MdLine::EmptyLine => todo!(),
        }
    }

    pub fn purify_head(&self, data: String) -> PurifiedMdLine {
        todo!()
    }

    pub fn purify_quote(&self, data: String) -> PurifiedMdLine {
        todo!()
    }

    pub fn purify_olist(&self, data: String) -> PurifiedMdLine {
        todo!()
    }

    pub fn purify_ulist(&self, data: String) -> PurifiedMdLine {
        todo!()
    }

    pub fn purify_image(&self, data: String) -> PurifiedMdLine {
        todo!()
    }

    pub fn purify_table(&self, data: String) -> PurifiedMdLine {
        todo!()
    }

    pub fn purify_definition(&self, data: String) -> PurifiedMdLine {
        todo!()
    }

    pub fn purify_taskline(&self, data: String) -> PurifiedMdLine {
        todo!()
    }

    pub fn purify_tabbedline(&self, data: String) -> PurifiedMdLine {
        todo!()
    }

    pub fn purify_text(&self, data: String) -> PurifiedMdLine {
        todo!()
    }
}
