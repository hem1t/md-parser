use super::md_inline::MdInline;

#[derive(Debug)]
pub(crate) struct MdString {
    string: Vec<MdInline>,
}

impl MdString {
    pub fn from_string(data: String) -> MdString {
        unimplemented!()
    }

}
