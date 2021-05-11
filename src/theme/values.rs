use super::units::{AbsoluteSize, GenericFontFamily, Length, RelativeSize};
use crate::Color;
use std::fmt;
use url::Url;

#[derive(Clone)]
pub enum CssValue {
    None,
    String(String)
}

impl Default for CssValue {
    fn default() -> Self {
        CssValue::None
    }
}

/// A partial CSS value, before inheritance has been resolved
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSValue<T> {
    Inherit,
    Specified(T),
}

// CSS 2.1, Section 8 - Box model

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSMargin {
    Length(Length),
    Percentage(f64),
    Auto,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSPadding {
    Length(Length),
    Percentage(f64),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSBorderWidth {
    Thin,
    Medium,
    Thick,
    Length(Length),
}

impl fmt::Display for CSSBorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Length(v) => fmt::Display::fmt(v, f),
            Self::Thin => write!(f, "thin"),
            Self::Medium => write!(f, "medium"),
            Self::Thick => write!(f, "thick"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSBorderColor {
    Color(Color),
    Transparent,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSBorderStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl fmt::Display for CSSBorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Hidden => write!(f, "hidden"),
            Self::Dotted => write!(f, "dotted"),
            Self::Dashed => write!(f, "dashed"),
            Self::Solid => write!(f, "solid"),
            Self::Double => write!(f, "double"),
            Self::Groove => write!(f, "groove"),
            Self::Ridge => write!(f, "ridge"),
            Self::Inset => write!(f, "inset"),
            Self::Outset => write!(f, "outset"),
        }
    }
}

// CSS 2.1, Section 9 - Visual formatting model

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSDisplay {
    // <display-outside> values
    Block,
    Inline,
    RunIn,

    // <display-inside> values
    Flow,
    FlowRoot,
    Table,
    Flex,
    Grid,
    Ruby,

    // <display-outside> plus <display-inside> values
    // Block_Flow,
    // Inline_Table,
    // Flex_RunIn,

    // <display-listitem> values
    ListItem,
    // ListItem_Block,
    // ListItem_Inline,
    // ListItem_Flow,
    // ListItem_FlowRoot,
    // ListItem_Block_Flow,
    // ListItem_Block_FlowRoot,
    // Flow_ListItem_Block,

    // <display-internal> values
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,

    // <display-box> values
    Contents,
    None,

    // <display-legacy> values
    InlineBlock,
    InlineTable,
    InlineFlex,
    InlineGrid,

    // Global values
    Inherit,
    Initial,
    Unset,
}

impl fmt::Display for CSSDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // <display-outside> values
            Self::Block => f.write_str("block"),
            Self::Inline => f.write_str("inline"),
            Self::RunIn => f.write_str("run-in"),

            // <display-inside> values
            Self::Flow => f.write_str("flow"),
            Self::FlowRoot => f.write_str("flow-root"),
            Self::Table => f.write_str("table"),
            Self::Flex => f.write_str("flex"),
            Self::Grid => f.write_str("grid"),
            Self::Ruby => f.write_str("ruby"),

            // <display-outside> plus <display-inside> values
            // Self::Block_Flow => f.write_str("block flow"),
            // Self::Inline_Table => f.write_str("inline table"),
            // Self::Flex_RunIn => f.write_str("flex run-in"),

            // <display-listitem> values
            Self::ListItem => f.write_str("list-item"),
            // Self::ListItem_Block => f.write_str("list-item block"),
            // Self::ListItem_Inline => f.write_str("list-item inline"),
            // Self::ListItem_Flow => f.write_str("list-item flow"),
            // Self::ListItem_FlowRoot => f.write_str("list-item flow-root"),
            // Self::ListItem_Block_Flow => f.write_str("list-item block flow"),
            // Self::ListItem_Block_FlowRoot => f.write_str("list-item block flow-root"),
            // Self::Flow_ListItem_Block => f.write_str("flow list-item block"),

            // <display-internal> values
            Self::TableRowGroup => f.write_str("table-row-group"),
            Self::TableHeaderGroup => f.write_str("table-header-group"),
            Self::TableFooterGroup => f.write_str("table-footer-group"),
            Self::TableRow => f.write_str("table-row"),
            Self::TableCell => f.write_str("table-cell"),
            Self::TableColumnGroup => f.write_str("table-column-group"),
            Self::TableColumn => f.write_str("table-column"),
            Self::TableCaption => f.write_str("table-caption"),
            Self::RubyBase => f.write_str("ruby-base"),
            Self::RubyText => f.write_str("ruby-text"),
            Self::RubyBaseContainer => f.write_str("ruby-base-container"),
            Self::RubyTextContainer => f.write_str("ruby-text-container"),

            // <display-box> values
            Self::Contents => f.write_str("contents"),
            Self::None => f.write_str("none"),

            // <display-legacy> values
            Self::InlineBlock => f.write_str("inline-block"),
            Self::InlineTable => f.write_str("inline-table"),
            Self::InlineFlex => f.write_str("inline-flex"),
            Self::InlineGrid => f.write_str("inline-grid"),

            // Global values
            Self::Inherit => f.write_str("inherit"),
            Self::Initial => f.write_str("initial"),
            Self::Unset => f.write_str("unset"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSPosition {
    Static,
    Relative,
    Absolute,
    Fixed,
}

impl fmt::Display for CSSPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Static => write!(f, "static"),
            Self::Relative => write!(f, "relative"),
            Self::Absolute => write!(f, "absolute"),
            Self::Fixed => write!(f, "fixed"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSTop {
    Length(Length),
    Percentage(f64),
    Auto,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSRight {
    Length(Length),
    Percentage(f64),
    Auto,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSBottom {
    Length(Length),
    Percentage(f64),
    Auto,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSLeft {
    Length(Length),
    Percentage(f64),
    Auto,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSFloat {
    None,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

impl fmt::Display for CSSFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::InlineStart => f.write_str("inline-start"),
            Self::InlineEnd => f.write_str("inline-end"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSClear {
    None,
    Left,
    Right,
    Both,
    InlineStart,
    InlineEnd,
}

impl fmt::Display for CSSClear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::Both => f.write_str("both"),
            Self::InlineStart => f.write_str("inline-start"),
            Self::InlineEnd => f.write_str("inline-end"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSDirection {
    Ltr,
    Rtl,
}

// CSS 2.1, Section 10 - Visual formatting model details

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSWidth {
    Length(Length),
    Percentage(f64),
    Auto,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSHeight {
    Length(Length),
    Percentage(f64),
    Auto,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSLineHeight {
    Normal,
    Number(f64),
    Length(Length),
    Percentage(f64),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSVerticalAlign {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
    Percentage(f64),
    Length(Length),
}

// CSS 2.1, Section 11 - Visual effects

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSOverflow {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}

impl fmt::Display for CSSOverflow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Visible => write!(f, "visible"),
            Self::Hidden => write!(f, "hidden"),
            Self::Clip => write!(f, "clip"),
            Self::Scroll => write!(f, "scroll"),
            Self::Auto => write!(f, "auto"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSVisibility {
    Visible,
    Hidden,
    Collapse,
}

// CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

// CSS 2.1, Section 13 - Paged media

// CSS 2.1, Section 14 - Colors and Backgrounds

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSColor {
    Color(Color),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSBackgroundColor {
    Color(Color),
    Transparent,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CSSBackgroundImage {
    Url(Url),
    None,
}

impl fmt::Display for CSSBackgroundImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Url(url) => write!(f, "url({})", url),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSBackgroundRepeat {
    Repeat,
    RepeatX,
    RepeatY,
    Space,
    Round,
    NoRepeat,
    // dual values: horisontal | vertical
    // background-repeat: repeat space;
    // background-repeat: repeat repeat;
    // background-repeat: round space;
    // background-repeat: no-repeat round;
    Inherit,
    Initial,
    Unset,
}

impl fmt::Display for CSSBackgroundRepeat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Repeat => write!(f, "repeat"),
            Self::RepeatX => write!(f, "repeat-x"),
            Self::RepeatY => write!(f, "repeat-y"),
            Self::Space => write!(f, "space"),
            Self::Round => write!(f, "round"),
            Self::NoRepeat => write!(f, "no-repeat"),
            Self::Inherit => write!(f, "inherit"),
            Self::Initial => write!(f, "initial"),
            Self::Unset => write!(f, "unset"),
        }
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-attachment
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSBackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

impl fmt::Display for CSSBackgroundAttachment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Scroll => write!(f, "scroll"),
            Self::Fixed => write!(f, "fixed"),
            Self::Local => write!(f, "local"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSBackgroundPosition {
    Percentage(f64),
    Length(Length),
    Left,
    Center,
    Right,
    Top,
    Bottom,
}

impl fmt::Display for CSSBackgroundPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Top => write!(f, "top"),
            Self::Left => write!(f, "left"),
            Self::Bottom => write!(f, "bottom"),
            Self::Right => write!(f, "right"),
            Self::Center => write!(f, "center"),
            Self::Percentage(v) => fmt::Display::fmt(v, f),
            Self::Length(v) => fmt::Display::fmt(v, f),
        }
    }
}

// CSS 2.1, Section 15 - Fonts

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CSSFontFamily {
    FamilyName(String),
    GenericFamily(GenericFontFamily),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSFontStyle {
    Normal,
    Italic,
    Oblique,
}

impl fmt::Display for CSSFontStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Normal => f.write_str("normal"),
            Self::Italic => f.write_str("italic"),
            Self::Oblique => f.write_str("oblique"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSFontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    /// Between 1 and 1000
    Number(f64),
}

impl fmt::Display for CSSFontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Normal => f.write_str("normal"),
            Self::Bold => f.write_str("bold"),
            Self::Lighter => f.write_str("lighter"),
            Self::Bolder => f.write_str("bolder"),
            Self::Number(v) => fmt::Display::fmt(v, f),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CSSFontSize {
    AbsoluteSize(AbsoluteSize),
    RelativeSize(RelativeSize),
    Length(Length),
    Percentage(f64),
}

// CSS 2.1, Section 16 - Text

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSTextAlign {
    Left,
    Right,
    Center,
    Justify,
}

impl fmt::Display for CSSTextAlign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Center => write!(f, "center"),
            Self::Justify => write!(f, "justify"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSTextDecoration {
    None,
    Underline,
    Overline,
    LineThrough,
    Blink,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CSSTextTransform {
    Capitalize,
    Uppercase,
    Lowercase,
    None,
}

// CSS 2.1, Section 17 - Tables

// CSS 2.1, Section 18 - User interface

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSCursor {
    // todo url
    Auto,
    Default,
    None,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NEResize,
    NWResize,
    SResize,
    SEResize,
    SWResize,
    WResize,
    EWResize,
    NSResize,
    NESWResize,
    NWSEResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
}

impl fmt::Display for CSSCursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Auto => f.write_str("auto"),
            Self::Default => f.write_str("default"),
            Self::None => f.write_str("none"),
            Self::ContextMenu => f.write_str("context-menu"),
            Self::Help => f.write_str("help"),
            Self::Pointer => f.write_str("pointer"),
            Self::Progress => f.write_str("progress"),
            Self::Wait => f.write_str("wait"),
            Self::Cell => f.write_str("cell"),
            Self::Crosshair => f.write_str("crosshair"),
            Self::Text => f.write_str("text"),
            Self::VerticalText => f.write_str("vertical-text"),
            Self::Alias => f.write_str("alias"),
            Self::Copy => f.write_str("copy"),
            Self::Move => f.write_str("move"),
            Self::NoDrop => f.write_str("no-drop"),
            Self::NotAllowed => f.write_str("not-allowed"),
            Self::Grab => f.write_str("grab"),
            Self::Grabbing => f.write_str("grabbing"),
            Self::EResize => f.write_str("e-resize"),
            Self::NResize => f.write_str("n-resize"),
            Self::NEResize => f.write_str("ne-resize"),
            Self::NWResize => f.write_str("nw-resize"),
            Self::SResize => f.write_str("s-resize"),
            Self::SEResize => f.write_str("se-resize"),
            Self::SWResize => f.write_str("sw-resize"),
            Self::WResize => f.write_str("w-resize"),
            Self::EWResize => f.write_str("ew-resize"),
            Self::NSResize => f.write_str("ns-resize"),
            Self::NESWResize => f.write_str("nesw-resize"),
            Self::NWSEResize => f.write_str("nwse-resize"),
            Self::ColResize => f.write_str("col-resize"),
            Self::RowResize => f.write_str("row-resize"),
            Self::AllScroll => f.write_str("all-scroll"),
            Self::ZoomIn => f.write_str("zoom-in"),
            Self::ZoomOut => f.write_str("zoom-out"),
        }
    }
}

/// https://www.w3.org/TR/css-flexbox-1/#propdef-justify-content
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSAlignContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    Stretch,
}

impl Default for CSSAlignContent {
    fn default() -> Self {
        Self::Stretch
    }
}

impl fmt::Display for CSSAlignContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FlexStart => write!(f, "flex-start"),
            Self::Center => write!(f, "center"),
            Self::FlexEnd => write!(f, "flex-end"),
            Self::SpaceAround => write!(f, "space-around"),
            Self::SpaceBetween => write!(f, "space-between"),
            Self::Stretch => write!(f, "stretch"),
        }
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/align-items
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSAlignItems {
    Normal,
    Stretch,
    Center,
    Start,
    End,
    FlexStart,
    FlexEnd,
    Baseline,
    FirstBaseline,
    LastBaseline,
    SafeCenter,
    UnsafeCenter,
}

impl fmt::Display for CSSAlignItems {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Stretch => write!(f, "stretch"),
            Self::Center => write!(f, "center"),
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::FlexStart => write!(f, "flex-start"),
            Self::FlexEnd => write!(f, "flex-end"),
            Self::Baseline => write!(f, "baseline"),
            Self::FirstBaseline => write!(f, "first baseline"),
            Self::LastBaseline => write!(f, "last baseline"),
            Self::SafeCenter => write!(f, "safe center"),
            Self::UnsafeCenter => write!(f, "unsafe center"),
        }
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/align-self
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSAlignSelf {
    Auto,
    Normal,
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    FlexStart,
    FlexEnd,
    Baseline,
    FirstBaseline,
    LastBaseline,
    Stretch,
    SafeCenter,
    UnsafeCenter,
}

impl fmt::Display for CSSAlignSelf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Normal => write!(f, "normal"),
            Self::Center => write!(f, "center"),
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::SelfStart => write!(f, "self-start"),
            Self::SelfEnd => write!(f, "self-end"),
            Self::FlexStart => write!(f, "flex-start"),
            Self::FlexEnd => write!(f, "flex-end"),
            Self::Baseline => write!(f, "baseline"),
            Self::FirstBaseline => write!(f, "first baseline"),
            Self::LastBaseline => write!(f, "last baseline"),
            Self::Stretch => write!(f, "stretch"),
            Self::SafeCenter => write!(f, "safe center"),
            Self::UnsafeCenter => write!(f, "unsafe center"),
        }
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-blend-mode
#[derive(Debug, Clone, PartialEq)]
pub enum CSSBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl fmt::Display for CSSBlendMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Multiply => write!(f, "multiply"),
            Self::Screen => write!(f, "screen"),
            Self::Overlay => write!(f, "overlay"),
            Self::Darken => write!(f, "darken"),
            Self::Lighten => write!(f, "lighten"),
            Self::ColorDodge => write!(f, "color-dodge"),
            Self::ColorBurn => write!(f, "color-burn"),
            Self::HardLight => write!(f, "hard-light"),
            Self::SoftLight => write!(f, "soft-light"),
            Self::Difference => write!(f, "difference"),
            Self::Exclusion => write!(f, "exclusion"),
            Self::Hue => write!(f, "hue"),
            Self::Saturation => write!(f, "saturation"),
            Self::Color => write!(f, "color"),
            Self::Luminosity => write!(f, "luminosity"),
        }
    }
}

/// https://www.w3.org/TR/css-flexbox-1/#propdef-justify-content
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSJustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
}

impl fmt::Display for CSSJustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FlexStart => write!(f, "flex-start"),
            Self::Center => write!(f, "center"),
            Self::FlexEnd => write!(f, "flex-end"),
            Self::SpaceAround => write!(f, "space-around"),
            Self::SpaceBetween => write!(f, "space-between"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSBoxSizing {
    BorderBox,
    ContentBox,
}

impl fmt::Display for CSSBoxSizing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BorderBox => f.write_str("border-box"),
            Self::ContentBox => f.write_str("content-box"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSColumnCount {
    Auto,
    Fixed(u32),
}

impl fmt::Display for CSSColumnCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Auto => f.write_str("auto"),
            Self::Fixed(inner) => write!(f, "{}", inner),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSFlexDirection {
    Row,
    Column,
}

impl fmt::Display for CSSFlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Row => f.write_str("row"),
            Self::Column => f.write_str("column"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSFlexWrap {
    Wrap,
    Nowrap,
}

impl fmt::Display for CSSFlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Wrap => write!(f, "wrap"),
            Self::Nowrap => write!(f, "nowrap"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CSSBorderCollapse {
    Collapse,
    Separate,
}

impl fmt::Display for CSSBorderCollapse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Collapse => write!(f, "collapse"),
            Self::Separate => write!(f, "separate"),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum CSSBackgroundBox {
    BorderBox,
    PaddingBox,
    ContentBox,
}

impl fmt::Display for CSSBackgroundBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BorderBox => write!(f, "border-box"),
            Self::PaddingBox => write!(f, "padding-box"),
            Self::ContentBox => write!(f, "content-box"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSListStyleType {
    Disc,
    Circle,
    Square,
    Decimal,
    DecimalLeadingZero,
    LowerRoman,
    UpperRoman,
    LowerGreek,
    UpperGreek,
    LowerLatin,
    UpperLatin,
    Armenian,
    Georgian,
    LowerAlpha,
    UpperAlpha,
    None,
}

impl fmt::Display for CSSListStyleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Disc => write!(f, "disc"),
            Self::Circle => write!(f, "circle"),
            Self::Square => write!(f, "square"),
            Self::Decimal => write!(f, "decimal"),
            Self::DecimalLeadingZero => write!(f, "decimal-leading-zero"),
            Self::LowerRoman => write!(f, "lower-roman"),
            Self::UpperRoman => write!(f, "upper-roman"),
            Self::LowerGreek => write!(f, "lower-greek"),
            Self::UpperGreek => write!(f, "upper-greek"),
            Self::LowerLatin => write!(f, "lower-latin"),
            Self::UpperLatin => write!(f, "upper-latin"),
            Self::Armenian => write!(f, "armenian"),
            Self::Georgian => write!(f, "georgian"),
            Self::LowerAlpha => write!(f, "lower-alpha"),
            Self::UpperAlpha => write!(f, "upper-alpha"),
            Self::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSResize {
    None,
    Both,
    Horizontal,
    Vertical,
}

impl fmt::Display for CSSResize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Both => write!(f, "both"),
            Self::Horizontal => write!(f, "horizontal"),
            Self::Vertical => write!(f, "vertical"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CSSWhiteSpace {
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    PreLine,
}

impl fmt::Display for CSSWhiteSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Pre => write!(f, "pre"),
            Self::Nowrap => write!(f, "nowrap"),
            Self::PreWrap => write!(f, "pre-wrap"),
            Self::PreLine => write!(f, "pre-line"),
        }
    }
}
