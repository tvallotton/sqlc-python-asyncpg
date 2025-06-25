use core::fmt;

use crate::{options, response};

pub trait Render {
    fn render(&self, options: options::Options, file: &mut Vec<u8>);
}
