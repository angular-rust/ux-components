use std::{cell::RefCell, fmt, rc::Rc};

use intmap::IntMap;

use super::{KeyEvent, MouseEvent, TextEvent};

#[derive(Default, Debug, Copy, Clone, PartialEq, Hash)]
pub struct Slot(pub u64);

impl Slot {
    pub fn id(&self) -> u64 {
        self.0
    }
}

/// A signal for mouse input events
pub type MouseSignal = Signal<MouseEvent>;
/// A signal for key input events
pub type KeySignal = Signal<KeyEvent>;
/// A signal for text input events
pub type TextSignal = Signal<TextEvent>;

#[derive(Clone)]
pub struct Signal<T: ?Sized> {
    listeners: Rc<RefCell<IntMap<Box<dyn Fn(&T)>>>>,
}

impl<T: ?Sized> Default for Signal<T> {
    fn default() -> Self {
        Self {
            listeners: Rc::new(RefCell::new(IntMap::new())),
        }
    }
}

impl<T: ?Sized> fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Signal")
    }
}

impl<T: ?Sized> Signal<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn listen(&self, handler: Box<dyn Fn(&T)>) -> Slot {
        let slot = Slot(rand::random());
        let mut listeners = self.listeners.borrow_mut();
        listeners.insert(slot.id(), handler);

        slot
    }

    pub fn remove(&self, slot: Slot) -> Option<Box<dyn Fn(&T)>> {
        let mut listeners = self.listeners.borrow_mut();
        listeners.remove(slot.id())
    }

    pub fn has(&self, slot: Slot) -> bool {
        let listeners = self.listeners.borrow();
        listeners.contains_key(slot.id())
    }

    pub fn clear(&self) {
        let mut listeners = self.listeners.borrow_mut();
        listeners.clear();
    }

    pub fn emit(&self, arg: &T) {
        let listeners = self.listeners.borrow();
        for (_, handler) in listeners.iter() {
            handler.as_ref()(arg);
        }
    }
}
