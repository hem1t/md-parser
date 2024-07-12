use super::md_inline::MdInline;

#[derive(Debug, PartialEq)]
pub(crate) struct MdString {
    string: Vec<MdInline>,
}

impl MdString {
    pub fn new() -> MdString {
        MdString {
            string: vec![]
        }
    }

    pub fn from_vec(data: Vec<MdInline>) -> Self {
        MdString {
            string: data
        }
    }

    pub fn from_string(data: String) -> MdString {
        unimplemented!()
    }

    pub fn push(&mut self, val: MdInline) {
        self.string.push(val);
    }

    pub fn last_mut(&mut self) -> Option<&mut MdInline> {
        self.string.last_mut()
    }
}
