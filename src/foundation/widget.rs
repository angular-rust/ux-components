// use crate::elements::Element;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(pub u64);

impl Id {
    // pub fn widget(&self) -> Option<&Box<dyn Element>> {
    //     todo!()
    // }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ChildBounds {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    pub x_local: f32,
    pub y_local: f32,
}
