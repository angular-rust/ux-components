use once_cell::sync::OnceCell;
use std::cell::RefCell;
use stretch::{
    geometry::Size,
    node::{MeasureFunc, Node, Stretch},
    number::Number,
    result::Layout,
    style::Style,
    Error,
};

pub struct LayoutSystem {
    system: RefCell<Stretch>,
}

unsafe impl std::marker::Send for LayoutSystem {}
unsafe impl std::marker::Sync for LayoutSystem {}

impl LayoutSystem {
    fn new() -> Self {
        Self {
            system: RefCell::new(Stretch::new()),
        }
    }

    fn global() -> &'static Self {
        static LAYOUT_SYSTEM_INSTANCE: OnceCell<LayoutSystem> = OnceCell::new();
        LAYOUT_SYSTEM_INSTANCE.get_or_init(Self::new)
    }

    pub fn new_leaf(style: Style, measure: MeasureFunc) -> Result<Node, Error> {
        let mut system = Self::global().system.borrow_mut();
        system.new_leaf(style, measure)
    }

    pub fn new_node(style: Style, children: Vec<Node>) -> Result<Node, Error> {
        let mut system = Self::global().system.borrow_mut();
        system.new_node(style, children)
    }

    /// Removes all nodes.
    ///
    /// All associated nodes will be invalid.
    pub fn clear() {
        let mut system = Self::global().system.borrow_mut();
        system.clear()
    }

    /// Remove nodes.
    pub fn remove(node: Node) {
        let mut system = Self::global().system.borrow_mut();
        system.remove(node)
    }

    pub fn set_measure(node: Node, measure: Option<MeasureFunc>) -> Result<(), Error> {
        let mut system = Self::global().system.borrow_mut();
        system.set_measure(node, measure)
    }

    pub fn add_child(node: Node, child: Node) -> Result<(), Error> {
        let mut system = Self::global().system.borrow_mut();
        system.add_child(node, child)
    }

    pub fn set_children(node: Node, children: Vec<Node>) -> Result<(), Error> {
        let mut system = Self::global().system.borrow_mut();
        system.set_children(node, children)
    }

    pub fn remove_child(node: Node, child: Node) -> Result<Node, Error> {
        let mut system = Self::global().system.borrow_mut();
        system.remove_child(node, child)
    }

    pub fn remove_child_at_index(node: Node, index: usize) -> Result<Node, Error> {
        let mut system = Self::global().system.borrow_mut();
        system.remove_child_at_index(node, index)
    }

    pub fn replace_child_at_index(node: Node, index: usize, child: Node) -> Result<Node, Error> {
        let mut system = Self::global().system.borrow_mut();
        system.replace_child_at_index(node, index, child)
    }

    pub fn children(node: Node) -> Result<Vec<Node>, Error> {
        let system = Self::global().system.borrow();
        system.children(node)
    }

    pub fn child_count(node: Node) -> Result<usize, Error> {
        let system = Self::global().system.borrow();
        system.child_count(node)
    }

    pub fn set_style(node: Node, style: Style) -> Result<(), Error> {
        let mut system = Self::global().system.borrow_mut();
        system.set_style(node, style)
    }

    pub fn style(node: Node) -> Result<Style, Error> {
        let system = Self::global().system.borrow();
        system.style(node).map(Clone::clone)
    }

    pub fn layout(node: Node) -> Result<Layout, Error> {
        let system = Self::global().system.borrow();
        system.layout(node).map(Clone::clone)
    }

    pub fn mark_dirty(node: Node) -> Result<(), Error> {
        let mut system = Self::global().system.borrow_mut();
        system.mark_dirty(node)
    }

    pub fn dirty(node: Node) -> Result<bool, Error> {
        let system = Self::global().system.borrow();
        system.dirty(node)
    }

    pub fn compute_layout(node: Node, size: Size<Number>) -> Result<(), Error> {
        let mut system = Self::global().system.borrow_mut();
        system.compute_layout(node, size)
    }
}
