#[derive(Debug, Clone)]
pub struct NeverEqual<T>(pub T);

impl<T> NeverEqual<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> PartialEq for NeverEqual<T> {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
