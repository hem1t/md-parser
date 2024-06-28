use crate::md_line_reader::MdLine;

///
/// Here lies implimentations for MdLine
/// to refine itself and become PurifiedMdLine
///
/// After passing `MdLine` through here they will selected by their
/// validity and will be refined into furthere seperated data or else
/// they will drop to being a simple Text.
///

#[derive(Debug, PartialEq)]
enum PurifiedMdLine {
    Head {
        title: String,
        level: u8,
        id: String,
    },
    Quote {
        nest_level: u8,
        inside_md: String,
    },
    FailedText(String),
}

impl PurifiedMdLine {
    pub fn purify(md_line: MdLine) -> PurifiedMdLine {
        match md_line {
            MdLine::Head(s) => PurifiedMdLine::purify_head(s),
            MdLine::Quote(s) => PurifiedMdLine::purify_quote(s),
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

    pub fn purify_head(data: String) -> PurifiedMdLine {
        // #head_1 count the hashes (should be between 1 and 6 inclusive)
        let mut hash_count = 0;
        for &ch in data.as_bytes().iter() {
            // count `#` until Space
            if ch == b'#' {
                hash_count = hash_count + 1;
            } else if ch == b' ' {
                break;
            } else {
                // found something in between then break;
                return PurifiedMdLine::FailedText(data);
            }
        }
        if hash_count > 6 && hash_count < 1 {
            // if out of bounds break;
            return PurifiedMdLine::FailedText(data);
        }

        // #head_2 should have space between hashes and head_text
        let mut head_text = String::new();
        let mut custom_id_part = String::new(); // can find here.
        for text in data.split(' ').skip(1) {
            if text.starts_with("{#") && text.ends_with("}") {
                custom_id_part.push_str(text);
                break;
            } else {
                head_text.push_str(text);
                head_text.push(' ');
            }
        }

        // OPTIONAL: after head_text may have space and then a custom_id in curly braces
        let mut custom_id = String::new();
        if !custom_id_part.is_empty() {
            unsafe {
                custom_id.push_str(&custom_id_part.get_unchecked(2..(custom_id_part.len() - 1)))
            }
        }

        PurifiedMdLine::Head {
            title: head_text.trim_end().to_string(),
            level: hash_count,
            id: custom_id,
        }
    }

    pub fn purify_quote(mut quotes: String) -> PurifiedMdLine {
        // count ">" and that is the level
        // everything after is text
        // series of ">" & after_text is divided by the "Space"
        if let Some(space_position) = quotes.find(' ') {
            let data = quotes.split_off(space_position);
            unsafe {
                PurifiedMdLine::Quote {
                    nest_level: quotes.len() as u8,
                    inside_md: data.get_unchecked(1..).to_string(),
                }
            }
        } else {
            PurifiedMdLine::FailedText(quotes)
        }
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

#[cfg(test)]
mod purifier_testing {
    use super::*;

    #[test]
    fn head_purifier_test() {
        // test with proper spacing
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Head(String::from("## head 2 {#head-2}"))),
            PurifiedMdLine::Head {
                title: String::from("head 2"),
                level: 2,
                id: String::from("head-2")
            }
        );

        // test with failed hash spacing
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Head(String::from("##head 2 {#head-2}"))),
            PurifiedMdLine::FailedText(String::from("##head 2 {#head-2}"))
        );

        // test with failed head_text && custom_id spacing
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Head(String::from("## head 2{#head-2}"))),
            PurifiedMdLine::Head {
                title: String::from("head 2{#head-2}"),
                level: 2,
                id: String::new()
            }
        );

        // test with failed custom_id spacing
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Head(String::from("## head 2 {# head-2}"))),
            PurifiedMdLine::Head {
                title: String::from("head 2 {# head-2}"),
                level: 2,
                id: String::new()
            }
        );
    }

    #[test]
    fn quote_purifier_test() {
        // test ok
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Quote(String::from("> blockquote"))),
            PurifiedMdLine::Quote {
                nest_level: 1,
                inside_md: String::from("blockquote")
            }
        );


        // should only trim one space from inside_md
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Quote(String::from("> \t blockquote"))),
            PurifiedMdLine::Quote {
                nest_level: 1,
                inside_md: String::from("\t blockquote")
            }
        );

        // test different level and more words
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Quote(String::from(">>> blockquote lask"))),
            PurifiedMdLine::Quote {
                nest_level: 3,
                inside_md: String::from("blockquote lask")
            }
        );
    }
}
