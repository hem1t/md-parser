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
    OList {
        list_number: u8,
        list_text: String,
    },
    UList {
        list_text: String,
    },
    Image {
        alt_text: String,
        link_text: String,
    },
    Table {
        row: Vec<String>,
    },
    Definition {
        def_text: String,
    },
    FailedText(String),
    EmptyLine,
    CodeStart,
    CodeEnd,
}

impl PurifiedMdLine {
    pub fn purify(md_line: MdLine) -> PurifiedMdLine {
        match md_line {
            MdLine::Head(s) => PurifiedMdLine::purify_head(s),
            MdLine::Quote(s) => PurifiedMdLine::purify_quote(s),
            MdLine::OList(s) => PurifiedMdLine::purify_olist(s),
            MdLine::UList(s) => PurifiedMdLine::purify_ulist(s),
            MdLine::Image(s) => PurifiedMdLine::purify_image(s),
            MdLine::Table(s) => PurifiedMdLine::purify_table(s),
            MdLine::CodeStart => PurifiedMdLine::CodeStart,
            MdLine::CodeEnd => PurifiedMdLine::CodeEnd,
            MdLine::Definition(s) => PurifiedMdLine::purify_definition(s),
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

    pub fn purify_olist(mut data: String) -> PurifiedMdLine {
        // find first ". ", which of length 2
        if let Some(dotspace_position) = data.find(". ") {
            let text = data.split_off(dotspace_position);
            PurifiedMdLine::OList {
                list_number: data.parse::<u8>().unwrap(),
                list_text: text.get(2..).unwrap().to_string(),
            }
        } else {
            PurifiedMdLine::FailedText(data)
        }
    }

    pub fn purify_ulist(mut data: String) -> PurifiedMdLine {
        // cut from first "Space"
        if let Some(space_position) = data.find(' ') {
            PurifiedMdLine::UList {
                list_text: data.split_off(space_position).get(1..).unwrap().to_string(),
            }
        } else {
            PurifiedMdLine::FailedText(data)
        }
    }

    pub fn purify_image(data: String) -> PurifiedMdLine {
        let mut image_text = data.trim().to_owned();
        // ![alt_text](link_text)
        if let Some(seperate_pos) = image_text.find("](") {
            let link_text = image_text.split_off(seperate_pos);
            PurifiedMdLine::Image {
                alt_text: image_text.get(2..).unwrap().trim().to_string(),
                link_text: link_text
                    .get(2..(link_text.len() - 1))
                    .unwrap()
                    .trim()
                    .to_string(),
            }
        } else {
            PurifiedMdLine::FailedText(data)
        }
    }

    pub fn purify_table(data: String) -> PurifiedMdLine {
        let table_data = data.trim().to_owned();
        if !(table_data.starts_with('|') && table_data.ends_with('|')) {
            // user can put spaces at end, if spaces then trim and check
            return PurifiedMdLine::FailedText(data);
        }

        let mut table_elems = Vec::with_capacity(5);
        // split and collect all strings
        for elems in table_data.split('|') {
            if !elems.is_empty() {
                table_elems.push(elems.to_owned());
            }
        }
        PurifiedMdLine::Table { row: table_elems }
    }

    pub fn purify_definition(data: String) -> PurifiedMdLine {
        // data is something that starts with ": "
        // take everything after ": "
        PurifiedMdLine::Definition {
            def_text: data.get(2..).unwrap().trim().to_owned(),
        }
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

    #[test]
    fn olist_purifier_test() {
        // test ok
        assert_eq!(
            PurifiedMdLine::purify(MdLine::OList(String::from("1. a list"))),
            PurifiedMdLine::OList {
                list_number: 1,
                list_text: String::from("a list")
            }
        );

        // take the space with you
        assert_eq!(
            PurifiedMdLine::purify(MdLine::OList(String::from("1. \ta list"))),
            PurifiedMdLine::OList {
                list_number: 1,
                list_text: String::from("\ta list")
            }
        );
        // test space missing between n. & list_text
        // this case should be filtered out before in MdLine
    }

    #[test]
    fn ulist_purifier_test() {
        // test ok
        assert_eq!(
            PurifiedMdLine::purify(MdLine::UList("- hello list is here".to_string())),
            PurifiedMdLine::UList {
                list_text: "hello list is here".to_string()
            }
        );

        // take the spaces with you
        assert_eq!(
            PurifiedMdLine::purify(MdLine::UList("-  hello list is here".to_string())),
            PurifiedMdLine::UList {
                list_text: " hello list is here".to_string()
            }
        );
    }

    #[test]
    fn image_purifier_test() {
        // test ok
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Image(String::from("![alt text](image.jpg)"))),
            PurifiedMdLine::Image {
                alt_text: "alt text".to_string(),
                link_text: "image.jpg".to_string()
            }
        );

        // spaces should be ignored
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Image(String::from("![alt text](image.jpg) "))),
            PurifiedMdLine::Image {
                alt_text: "alt text".to_string(),
                link_text: "image.jpg".to_string()
            }
        );

        // space in between braces and exclamation should fail
        // this will be filtered out in `MdLineReader`
        // assert_eq!(
        //     PurifiedMdLine::purify(MdLine::Image(String::from("! [alt text](image.jpg) "))),
        //     PurifiedMdLine::FailedText("! [alt text](image.jpg) ".to_string())
        // );

        // space in between [] () should fail
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Image(String::from("![alt text] (image.jpg) "))),
            PurifiedMdLine::FailedText("![alt text] (image.jpg) ".to_string())
        );
    }

    #[test]
    fn table_purifier_test() {
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Table("| hello | world |".to_string())),
            PurifiedMdLine::Table {
                row: vec![" hello ".to_string(), " world ".to_string()]
            }
        );

        // avoid preceding spaces
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Table("| hello | world | ".to_string())),
            PurifiedMdLine::Table {
                row: vec![" hello ".to_string(), " world ".to_string()]
            }
        );

        // should only end with |
        assert_eq!(
            PurifiedMdLine::purify(MdLine::Table("| hello | world  ".to_string())),
            PurifiedMdLine::FailedText("| hello | world  ".to_string())
        );
    }

    #[test]
    fn definition_purifier_test() {
       // test all trimmed and saved
       assert_eq!(
           PurifiedMdLine::purify(MdLine::Definition(": the definition   \t".to_string())),
           PurifiedMdLine::Definition { def_text: "the definition".to_string() }
       );
    }
}
