#![allow(dead_code)]
#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

#[macro_use]
extern crate bounded_integer;

use std::fmt::Debug;

trait AssertImplDebug: Debug { }

bounded_integer! {
    /// Documentation.
    #[derive(Debug)]
    enum A: u8 { 0...1 }
}
impl AssertImplDebug for A { }
