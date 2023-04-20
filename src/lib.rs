#![feature(generic_const_exprs)]

pub enum Check<const CONDITION: bool> {}

pub trait Passed {}
pub trait Failed {}

impl Passed for Check<true> {}
impl Failed for Check<false> {}

#[macro_export]
macro_rules! define_check {

    ($name:ident) => {
        // pub type $name = $crate::Check
    }

}

define_check!(Yester);

mod standard;
use standard::*;

fn test() {
}