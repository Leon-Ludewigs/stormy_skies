mod always_equal;
pub use always_equal::AlwaysEqual;

mod never_equal;
pub use never_equal::NeverEqual;

pub type CloneableStr = leptos::Oco<'static, str>;
