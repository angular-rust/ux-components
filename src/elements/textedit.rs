use cgmath::Point2;
use std::{
    cell::RefCell,
    char,
    default::default,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};
use stretch::{node::Node, style};

use crate::prelude::Singleton;

use crate::{
    foundation::{
        properties::{LabelProperties, TextEditProperties},
        KeyCode, KeyEvent, MouseEvent, Signal, TextEditChangeEvent, TextEditCommitEvent, TextEvent,
        TextEventType,
    },
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, LabelElement, WidgetComponent};

#[derive(Default, Debug, Clone)]
struct TextEditState {
    index: i32,              // = 0,
    composition_start: i32,  // = 0;
    composition_length: i32, // = 0;
    composition: String,     // = "";
    edit: String,            // = "";
    display: String,         // = "";
    // pub display_text (get, never) -> String,
    display_char: Option<char>,
    text: String,
}

/// A simple text edit control
/// Additional Signals: none
pub struct TextEditElement {
    component: Rc<RefCell<WidgetComponent>>,
    pub label: LabelElement,
    pub filter: Option<Box<dyn Fn(String, String, String) -> bool>>,
    state: RefCell<TextEditState>,
    /// Emitted whenever the index is changed.
    pub onchangeindex: Signal<i32>,
    /// Emitted whenever the text or display text is changed.
    /// `text:String, display_text:String, from_typing: bool`
    pub onchange: Signal<TextEditChangeEvent>,

    /// Emitted whenever the return key is pressed.
    /// `text:String, display_text:String`
    pub oncommit: Signal<TextEditCommitEvent>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for TextEditElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("TextEditElement").finish()
    }
}

impl TextEditElement {
    pub fn new(widget: &TextEditProperties) -> Self {
        let id = widget.key.id();

        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let component = WidgetComponent::get(widget.key.id());

        // let filter = options.filter;

        let label = LabelElement::new(&LabelProperties {
            parent: Some(id),
            x: 2.0,
            y: 0.0,
            // w: component.w,
            // h: component.h,
            // options: options.options.label,
            mouse_input: false,
            depth: widget.depth + 0.001,
            internal_visible: widget.visible,
            text: widget.text.clone(),
            text_size: widget.text_size,
            align: widget.align,
            align_vertical: widget.align_vertical,
            bounds_wrap: widget.bounds_wrap,
            ..Default::default()
        });

        let edit = widget.text.clone();
        let index = edit.len() as i32;

        let display_char = widget.display_char;

        let instance = Self {
            component,
            filter: None, // TODO: DV
            state: RefCell::new(TextEditState {
                index,
                composition_length: 0,
                composition_start: 0,
                composition: String::new(),
                edit: edit.clone(),
                display: String::new(),
                display_char,
                text: String::new(),
            }),
            label,
            onchange: Signal::new(),
            onchangeindex: Signal::new(),
            oncommit: Signal::new(),
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        };

        instance.refresh(edit, false, true);

        instance
    }

    #[inline]
    // _from_typing: bool=true, _emit: bool=true
    fn refresh(&self, str: String, from_typing: bool, emit: bool) -> String {
        let mut state = self.state.borrow_mut();
        state.edit = str.clone();

        match state.display_char {
            Some(ch) => {
                state.display = String::new();
                for i in 0..str.len() {
                    state.display.push(ch);
                }
            }
            None => {
                let state = self.state.borrow();
                let index = state.index as usize;
                self.state.borrow_mut().display = format!(
                    "{}{}{}",
                    self.before(index),
                    state.composition,
                    self.after(index)
                );
            }
        }

        let state = self.state.borrow();
        self.label.set_text(state.display.clone());
        self.update_cur();

        if emit {
            self.onchange.emit(&TextEditChangeEvent {
                text: state.edit.clone(),
                display_text: state.display.clone(),
                from_typing,
            });
        }

        state.edit.clone()
    }

    pub fn index(&self) -> i32 {
        self.state.borrow().index
    }

    pub fn text(&self) -> String {
        self.state.borrow().edit.clone()
    }

    #[inline]
    pub fn display_text(&self) -> String {
        self.state.borrow().display.clone()
    }

    #[inline]
    pub fn display_char(&self) -> Option<char> {
        self.state.borrow().display_char
    }

    #[inline]
    pub fn set_text(&self, value: String) {
        self.state.borrow_mut().index = value.len() as i32;
        self.refresh(value, false, true);
    }

    #[inline]
    pub fn set_display_char(&mut self, value: Option<char>) {
        let mut state = self.state.borrow_mut();
        state.display_char = value;

        self.refresh(state.edit.clone(), false, true);
        self.update_cur();
    }

    // amount:i32 = -1
    pub fn move_cursor(&self, amount: i32) {
        let state = self.state.borrow();
        let mut index = state.index;
        index += amount;
        self.state.borrow_mut().index = index.clamp(0, state.edit.len() as i32);

        // println!("index {} / {}", index, edit.len());
        // println!(before(index) + "|" + after(index));

        self.update_cur();
    }

    #[inline]
    // start: i32 = 0, count: i32 = 1
    pub fn cut(&self, start: usize, count: usize) -> String {
        let text: String = self.after(start).chars().skip(count).collect();
        self.refresh(format!("{}{}", self.before(start), text), true, true)
    }

    #[inline]
    // cur: i32 = 0
    pub fn after(&self, cur: usize) -> String {
        self.state.borrow().edit.chars().skip(cur).collect()
    }

    #[inline]
    // cur: i32 = 0
    pub fn before(&self, cur: usize) -> String {
        self.state.borrow().edit.chars().take(cur).collect()
    }

    #[inline]
    // cur: i32 = 0
    pub fn after_display(&self, cur: usize) -> String {
        self.state.borrow().display.chars().skip(cur).collect()
    }

    #[inline]
    // cur: i32 = 0
    pub fn before_display(&self, cur: usize) -> String {
        self.state.borrow().display.chars().take(cur).collect()
    }

    #[inline]
    pub fn update_cur(&self) {
        let index = self.state.borrow().index;
        self.onchangeindex.emit(&index);
    }
}

impl AsRef<RefCell<WidgetComponent>> for TextEditElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for TextEditElement {
    fn destroy(&self) {
        // self.base.destroy();

        self.onchange.clear();
        self.onchangeindex.clear();
        self.oncommit.clear();
    }

    fn mousedown(&self, event: &mut MouseEvent) {
        // self.base.mousedown(event);

        if self.contains(event.x as f32, event.y as f32) {
            self.focus();
        } else {
            self.unfocus();
        }
    }

    fn unfocus(&self) {
        // self.base.unfocus();
        let mut state = self.state.borrow_mut();
        state.composition = String::new();
        state.composition_start = 0;
        state.composition_length = 0;
    }

    fn textinput(&self, event: &mut TextEvent) {
        // self.base.textinput(event);

        let index = self.state.borrow().index;
        let bef_str = self.before(index as usize);
        let aft_str = self.after(index as usize);
        let new_str = format!("{}{}{}", bef_str, event.text, aft_str);

        match event.event_type {
            TextEventType::Edit => {
                let mut state = self.state.borrow_mut();
                state.composition = event.text.clone();
                state.composition_start = event.start;
                state.composition_length = event.length;

                self.refresh(state.edit.clone(), true, false);
            }
            TextEventType::Input => {
                let mut state = self.state.borrow_mut();
                state.composition = String::new();
                state.composition_start = 0;
                state.composition_length = 0;

                if let Some(ref filter) = self.filter {
                    if !filter(event.text.clone(), new_str.clone(), state.edit.clone()) {
                        return;
                    }
                }

                let index = state.index;
                self.state.borrow_mut().index = index + event.text.len() as i32;
                self.refresh(new_str, true, true);
            }
            _ => {}
        }
    }

    fn keydown(&self, event: &mut KeyEvent) {
        let state = self.state.borrow();
        // self.base.keydown(event);

        match event.key {
            KeyCode::Backspace => {
                self.move_cursor(-1);
                let index = self.state.borrow().index;
                self.cut(index as usize, 1);
            }
            KeyCode::Delete => {
                let index = self.state.borrow().index;
                self.cut(index as usize, 1);
            }
            KeyCode::Left => self.move_cursor(-1),
            KeyCode::Right => self.move_cursor(1),
            KeyCode::Enter => self.oncommit.emit(&TextEditCommitEvent {
                display_text: state.display.clone(),
                text: state.edit.clone(),
            }),
            KeyCode::Escape | KeyCode::Tab | KeyCode::Unknown | KeyCode::Down | KeyCode::Up => {}
        }
    }

    fn node(&self) -> Option<Node> {
        Some(self.node)
    }

    fn relayout(&self, origin: Point2<f32>) {
        let update_childs = match LayoutSystem::layout(self.node) {
            Ok(layout) => {
                let mut comp = self.as_ref().borrow_mut();
                comp.x = layout.location.x + origin.x;
                comp.y = layout.location.y + origin.y;
                comp.w = layout.size.width;
                comp.h = layout.size.height;

                true
            }
            Err(e) => {
                log::error!("{}", e);
                false
            }
        };

        if update_childs {
            // self.leading.relayout();
            // self.title.relayout();
            // self.flexible_space.relayout();
        }
    }
}
