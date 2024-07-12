use std::alloc::Allocator;

mod inline_tokens;
mod md_inline;
///
/// After Blocks have been parsed we come here
/// to find Inline Elements
///
pub mod md_string;

pub(crate) trait VecLastMutIfMatch<T, P>
where
    P: FnOnce(&T) -> bool,
{
    fn last_mut_if(&mut self, f: P) -> Option<&mut T>;
}

impl<T, A, P> VecLastMutIfMatch<T, P> for Vec<T, A>
where
    A: Allocator,
    P: FnOnce(&T) -> bool,
{
    fn last_mut_if(&mut self, f: P) -> Option<&mut T> {
        if let Some(x) = self.last_mut() {
            if f(&x) {
                return Some(x);
            }
        }
        None
    }
}
