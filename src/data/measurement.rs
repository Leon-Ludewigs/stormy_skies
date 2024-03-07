use std::fmt::{Debug, Display};

pub trait Measurement: Copy + Debug + Display + Into<f32> + PartialEq {
    fn unit(&self) -> &'static str;
}
