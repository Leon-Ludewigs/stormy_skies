#[derive(Debug, Clone)]
pub struct AlwaysEqual<T>(pub T);

impl<T> AlwaysEqual<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> PartialEq for AlwaysEqual<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
