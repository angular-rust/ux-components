#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use crate::{elements::Element, foundation::Id};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i32)]
pub enum AnchorType {
    CenterX = 1,
    CenterY = 2,
    Left = 3,
    Right = 4,
    Top = 5,
    Bottom = 6,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i32)]
pub enum MarginType {
    Percent = 1,
    Fixed = 2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i32)]
pub enum SizeTarget {
    Width = 1,
    Height = 2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i32)]
pub enum MarginTarget {
    Left = 1,
    Right = 2,
    Top = 3,
    Bottom = 4,
}

#[derive(Debug, Clone, Copy)]
pub struct Margin {
    primary: Id,
    other: Id,
    target: MarginTarget,
    margin_type: MarginType,
    value: f32,
}

impl Margin {
    pub fn update(&self) {
        // let primary = self.primary.widget().unwrap();
        // let other = self.other.widget().unwrap();
        // let value = self.value;
        // let target = self.target;

        // match self.margin_type {
        //     MarginType::Fixed => match target {
        //         MarginTarget::Left => primary.set_x((other.x() + value).abs()),
        //         MarginTarget::Top => primary.set_y((other.y() + value).abs()),
        //         MarginTarget::Right => primary.set_w(((other.right() - value) - primary.x()).abs()),
        //         MarginTarget::Bottom => {
        //             primary.set_h(((other.bottom() - value) - primary.y()).abs())
        //         }
        //     },
        //     MarginType::Percent => {
        //         let per = value / 100.0;
        //         match target {
        //             MarginTarget::Left => primary.set_x((other.x() + (other.w() * per)).abs()),
        //             MarginTarget::Top => primary.set_y((other.y() + (other.h() * per)).abs()),
        //             MarginTarget::Right => {
        //                 primary.set_w(((other.right() - (other.w() * per)) - primary.x()).abs())
        //             }
        //             MarginTarget::Bottom => {
        //                 primary.set_h(((other.bottom() - (other.h() * per)) - primary.y()).abs())
        //             }
        //         }
        //     }
        // }
    }
}

// Hashing Margin except margin_type and value
impl Hash for Margin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.primary.hash(state);
        self.other.hash(state);
        self.target.hash(state);
    }
}

impl PartialEq for Margin {
    fn eq(&self, other: &Self) -> bool {
        self.primary == other.primary && self.other == other.other && self.target == other.target
    }
}

impl Eq for Margin {}

#[derive(Debug, Clone, Copy)]
struct Sizer {
    primary: Id,
    other: Id,
    target: SizeTarget,
    value: f32,
}

impl Sizer {
    pub fn update(&self) {
        // let primary = self.primary.widget().unwrap();
        // let other = self.other.widget().unwrap();
        // let per = self.value / 100.0;

        // match self.target {
        //     SizeTarget::Width => primary.set_w(other.w() * per),
        //     SizeTarget::Height => primary.set_h(other.h() * per),
        // }
    }
}

// Hashing Sizer except value
impl Hash for Sizer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.primary.hash(state);
        self.other.hash(state);
        self.target.hash(state);
    }
}

impl PartialEq for Sizer {
    fn eq(&self, other: &Self) -> bool {
        self.primary == other.primary && self.other == other.other && self.target == other.target
    }
}

impl Eq for Sizer {}

#[derive(Debug, Clone, Copy)]
struct Anchor {
    primary: Id,
    other: Id,
    primary_anchor: AnchorType,
    other_anchor: AnchorType,
    offset: Option<i32>,
}

impl Anchor {
    pub fn update(&self) {
        // let other = self.other.widget().unwrap();
        // let primary = self.primary.widget().unwrap();

        // let mut reference = match self.other_anchor {
        //     AnchorType::CenterX => other.x() + (other.w() / 2.0),
        //     AnchorType::CenterY => other.y() + (other.h() / 2.0),
        //     AnchorType::Right => other.right(),
        //     AnchorType::Bottom => other.bottom(),
        //     AnchorType::Left => other.x(),
        //     AnchorType::Top => other.y(),
        // };

        // reference += self.offset.unwrap_or_default() as f32;

        // match self.primary_anchor {
        //     AnchorType::CenterX => primary.set_x(reference - (primary.w() / 2.0)),
        //     AnchorType::CenterY => primary.set_y(reference - (primary.h() / 2.0)),
        //     AnchorType::Right => primary.set_x(reference - primary.w()),
        //     AnchorType::Bottom => primary.set_y(reference - primary.h()),
        //     AnchorType::Left => primary.set_x(reference),
        //     AnchorType::Top => primary.set_y(reference),
        // }
    }
}

// Hashing Sizer except offset
// TODO: need additional deal with logic i think
impl Hash for Anchor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.primary.hash(state);
        self.other.hash(state);
        self.primary_anchor.hash(state);
        self.other_anchor.hash(state);
    }
}

impl PartialEq for Anchor {
    fn eq(&self, other: &Self) -> bool {
        self.primary == other.primary
            && self.other == other.other
            && self.primary_anchor == other.primary_anchor
            && self.other_anchor == other.other_anchor
    }
}

impl Eq for Anchor {}

#[derive(Default, Debug, Clone)]
pub struct Layout {
    margins: HashSet<Margin>,
    anchors: HashSet<Anchor>,
    sizers: HashSet<Sizer>,
}

pub struct Margins {
    observed: HashMap<Id, Layout>,
}

impl Margins {
    pub fn new() -> Self {
        Self {
            observed: HashMap::new(),
        }
    }

    // ?other:WidgetComponent, target:SizeTarget, value: f32
    pub fn size(
        &mut self,
        primary: &dyn Element,
        other: Option<&dyn Element>,
        target: SizeTarget,
        value: f32,
    ) {
        if let Some(other) = other.or(primary.parent()) {
            let layout = self.get(other);

            let sizer: Sizer = Sizer {
                target: target,
                value: value,
                primary: primary.id(),
                other: other.id(),
            };

            sizer.update();

            layout.sizers.insert(sizer);

            // primary.ondestroy.listen(box |_| {
            //     layout.sizers.remove(&sizer);
            // });
        } else {
            log::info!("No other for sizers")
        }
    }

    // ?other:WidgetComponent, self_anchor:AnchorType, other_anchor:AnchorType, offset:i32=0
    pub fn anchor(
        &mut self,
        primary: &dyn Element,
        other: Option<&dyn Element>,
        self_anchor: AnchorType,
        other_anchor: AnchorType,
        offset: Option<i32>,
    ) {
        if let Some(other) = other.or(primary.parent()) {
            let layout = self.get(other);

            let anchor: Anchor = Anchor {
                primary_anchor: self_anchor,
                other_anchor: other_anchor,
                primary: primary.id(),
                other: other.id(),
                offset,
            };

            anchor.update();

            layout.anchors.insert(anchor);

            // primary.ondestroy.listen(box |_| {
            //     layout.anchors.remove(&anchor);
            // });
        } else {
            log::info!("No other for anshors")
        }
    }

    // ?other:WidgetComponent, target:MarginTarget, type:MarginType, value: f32
    pub fn margin(
        &mut self,
        primary: &dyn Element,
        other: Option<&dyn Element>,
        target: MarginTarget,
        margin_type: MarginType,
        value: f32,
    ) {
        if let Some(other) = other.or(primary.parent()) {
            let layout = self.get(other);

            let margin: Margin = Margin {
                target: target,
                margin_type,
                value: value,
                primary: primary.id(),
                other: other.id(),
            };

            margin.update();

            layout.margins.insert(margin);

            // primary.ondestroy.listen(box |_| {
            //     layout.margins.remove(&margin);
            // });
        } else {
            log::info!("No other for margins")
        }
    }

    //Internal

    fn get(&mut self, other: &dyn Element) -> &mut Layout {
        if !self.observed.contains_key(&other.id()) {
            let layout = Layout::default();

            // other.ondestroy.listen(box |_| {
            //     self.observed.remove(&other.id());
            //     layout.anchors.clear();
            //     layout.margins.clear();
            //     layout.sizers.clear();
            // });

            // let other_id = other.id();
            // other.onbounds.listen(box |_| self.update_layout(other_id));

            self.observed.insert(other.id(), layout);
        }

        self.observed.get_mut(&other.id()).unwrap()
    }

    // fn update_layout(&self, other: WidgetId) {
    //     if let Some(layout) = self.observed.get(&other) {
    //         for margin in layout.margins.iter() {
    //             margin.update();
    //         }

    //         for anchor in layout.anchors.iter() {
    //             anchor.update();
    //         }

    //         for sizer in layout.sizers.iter() {
    //             sizer.update();
    //         }
    //     }
    // }
}
