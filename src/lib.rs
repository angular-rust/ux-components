#![allow(unused_imports)]

#[macro_use]
extern crate glib;

#[macro_use]
extern crate bitflags;

mod backend;
pub use backend::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
