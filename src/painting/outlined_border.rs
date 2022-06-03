#![allow(unused_imports)]
use super::{BorderSide, ShapeBorder};

pub trait OutlinedBorder: ShapeBorder {
    // pub side: BorderSide,

    // Returns a copy of this OutlinedBorder that draws its outline with the specified side, if side is non-null. 
    // fn copy_with(&self, side: Option<BorderSide>) -> Box<dyn OutlinedBorder>;
}

#[derive(Default)]
pub struct NoneOutlinedBorder;

impl OutlinedBorder for NoneOutlinedBorder {

}

impl ShapeBorder for NoneOutlinedBorder {

}

// impl Default for OutlinedBorder {
//     fn default() -> Self {
//         Self {
//             side: Default::default(),
//         }
//     }
// }
