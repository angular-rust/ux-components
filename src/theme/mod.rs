#![allow(unused_imports, unused_variables)]
use animate::{Color, Gradient, Pattern};
use intmap::IntMap;
use once_cell::sync::OnceCell;
use std::{
    cell::RefCell,
    collections::HashMap,
    convert::{From, TryFrom},
    sync::Mutex,
};

mod style;

mod properties;
pub use properties::*;

mod rule;
pub use rule::*;

mod values;
pub use values::*;

mod units;
pub use units::*;

#[derive(Debug, Clone)]
pub enum Fill {
    Solid(Color),
    Gradient(Gradient),
    // Image(Pattern)
}

#[derive(Debug, Clone)]
pub enum StyleError {
    Poisoned,
}

#[derive(Default, Debug, Clone)]
pub struct StyleDefinition {
    pub background: Option<Fill>,
    pub color: Option<Color>,
    // height
    // padding
    pub fontfamily: Option<String>,
    pub fontsize: Option<f64>,
    pub fontweight: Option<String>,
    pub border_radius: Option<f64>,
}

#[derive(Debug, Copy, Clone)]
#[repr(u64)]
pub enum StyleClass {
    MdcButton = 1,
}

impl Into<u64> for StyleClass {
    fn into(self) -> u64 {
        self as u64
    }
}

#[derive(Debug)]
pub struct Theme {
    styles: Mutex<IntMap<StyleDefinition>>,
}

impl Theme {
    fn new() -> Self {
        println!("CREATE THEME INSTANCE");
        
        let mut definitions = IntMap::new();
        definitions.insert(StyleClass::MdcButton.into(), style::mdc_button());

        Self {
            styles: Mutex::new(definitions),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<Theme> = OnceCell::new();
        INSTANCE.get_or_init(Self::new)
    }

    pub fn get(&self, class: impl Into<u64>) -> Result<StyleDefinition, StyleError> {
        match self.styles.lock() {
            Ok(styles) => {
                let style = styles.get(class.into()).unwrap();
                Ok(style.clone())
            }
            Err(e) => Err(StyleError::Poisoned),
        }
    }
}

pub fn test_global() {
    Theme::global().get(StyleClass::MdcButton).unwrap();
}
