#![allow(dead_code)]
use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Length {
    Em(f64), // normalized to 'em'
    Ex(f64),
    In(f64),
    Cm(f64),
    Mm(f64),
    Pt(f64),
    Pc(f64),
    Px(f64), // normalized to 'px'
    Zero,
}

impl Length {
    pub fn rel(self) -> f64 {
        match self {
            Self::Em(x) => x,
            _ => panic!("attempted to access relative unit of an absolute length")
        }
    }
    pub fn abs(self) -> f64 {
        match self {
            Self::Em(x) => x,
            _ => panic!("attempted to access relative unit of an absolute length")
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Length::Em(val) => write!(f, "{}em", val),
            Length::Ex(val) => write!(f, "{}ex", val),
            Length::In(val) => write!(f, "{}in", val),
            Length::Cm(val) => write!(f, "{}cm", val),
            Length::Mm(val) => write!(f, "{}mm", val),
            Length::Pt(val) => write!(f, "{}pt", val),
            Length::Pc(val) => write!(f, "{}pc", val),
            Length::Px(val) => write!(f, "{}px", val),
            Length::Zero => write!(f, "0"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BoxSizing { // used by width, height, top, left, etc
    BoxLength(Length),
    BoxPercent(f64),
    BoxAuto
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AbsoluteSize {
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
    Larger,
    Smaller,
    // LengthPercentage(Calc),
}

impl fmt::Display for AbsoluteSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::XXSmall => f.write_str("xx-small"),
            Self::XSmall => f.write_str("x-small"),
            Self::Small => f.write_str("small"),
            Self::Medium => f.write_str("medium"),
            Self::Large => f.write_str("large"),
            Self::XLarge => f.write_str("x-large"),
            Self::XXLarge => f.write_str("xx-large"),
            Self::XXXLarge => f.write_str("xxx-large"),
            Self::Larger => f.write_str("larger"),
            Self::Smaller => f.write_str("smaller"),
            // Self::LengthPercentage(v) => fmt::Display::fmt(v, f),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum RelativeSize {
    Larger,
    Smaller
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum GenericFontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    // Named(String),
}

impl fmt::Display for GenericFontFamily {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Serif => write!(f, "serif"),
            Self::SansSerif => write!(f, "sans-serif"),
            Self::Cursive => write!(f, "cursive"),
            Self::Fantasy => write!(f, "fantasy"),
            Self::Monospace => write!(f, "monospace"),
            // Self::Named(inner) => write!(f, "\"{}\"", inner),
        }
    }
}