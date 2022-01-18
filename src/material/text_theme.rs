use crate::painting::TextStyle;

/*
apply({String? fontFamily, double fontSizeFactor = 1.0, double fontSizeDelta = 0.0, Color? displayColor, Color? bodyColor, TextDecoration? decoration, Color? decorationColor, TextDecorationStyle? decorationStyle}) → TextTheme
Creates a copy of this text theme but with the given field replaced in each of the individual text styles. [...]

copyWith({TextStyle? headline1, TextStyle? headline2, TextStyle? headline3, TextStyle? headline4, TextStyle? headline5, TextStyle? headline6, TextStyle? subtitle1, TextStyle? subtitle2, TextStyle? bodyText1, TextStyle? bodyText2, TextStyle? caption, TextStyle? button, TextStyle? overline}) → TextTheme
Creates a copy of this text theme but with the given fields replaced with the new values. [...]

debugFillProperties(DiagnosticPropertiesBuilder properties) → void
Add additional properties associated with the node. [...]
override

merge(TextTheme? other) → TextTheme
Creates a new TextTheme where each text style from this object has been merged with the matching text style from the other object. [...]
*/

pub struct TextTheme {
    pub headline1: TextStyle,
    pub headline2: TextStyle,
    pub headline3: TextStyle,
    pub headline4: TextStyle,
    pub headline5: TextStyle,
    pub headline6: TextStyle,
    pub subtitle1: TextStyle,
    pub subtitle2: TextStyle,
    pub body_text1: TextStyle,
    pub body_text2: TextStyle,
    pub caption: TextStyle,
    pub button: TextStyle,
    pub overline: TextStyle,
}

impl Default for TextTheme {
    fn default() -> Self {
        Self {
            headline1: Default::default(),
            headline2: Default::default(),
            headline3: Default::default(),
            headline4: Default::default(),
            headline5: Default::default(),
            headline6: Default::default(),
            subtitle1: Default::default(),
            subtitle2: Default::default(),
            body_text1: Default::default(),
            body_text2: Default::default(),
            caption: Default::default(),
            button: Default::default(),
            overline: Default::default(),
        }
    }
}
