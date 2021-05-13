/// The complete list of standard properties belonging to the latest CSS3 specifications
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum CssProperty {
    /// Specifies the alignment of flexible container's items within the flex container: `align-content`
    AlignContent = 0x1,
    /// Specifies the default alignment for items within the flex container: `align-items`
    AlignItems,
    /// Specifies the alignment for selected items within the flex container: `align-self`
    AlignSelf,
    /// Specifies the keyframe-based animations: `animation`
    Animation,
    /// Specifies when the animation will start: `animation-delay`
    AnimationDelay,
    /// Specifies whether the animation should play in reverse on alternate cycles or not: `animation-direction`
    AnimationDirection,
    /// Specifies the number of seconds or milliseconds an animation should take to complete one cycle: `animation-duration`
    AnimationDuration,
    /// Specifies how a CSS animation should apply styles to its target before and after it is executing: `animation-fill-mode`
    AnimationFillMode,
    /// Specifies the number of times an animation cycle should be played before stopping: `animation-iteration-count`
    AnimationIterationCount,
    /// Specifies the name of @keyframes defined animations that should be applied to the selected element: `animation-name`
    AnimationName,
    /// Specifies whether the animation is running or paused: `animation-play-state`
    AnimationPlayState,
    /// Specifies how a CSS animation should progress over the duration of each cycle: `animation-timing-function`
    AnimationTimingFunction,
    /// Specifies whether or not the "back" side of a transformed element is visible when facing the user: `backface-visibility`
    BackfaceVisibility,
    /// Defines a variety of background properties within one declaration: `background`
    Background,
    /// Specify whether the background image is fixed in the viewport or scrolls: `background-attachment`
    BackgroundAttachment,
    /// Specifies the painting area of the background: `background-clip`
    BackgroundClip,
    /// Defines an element's background color: `background-color`
    BackgroundColor,
    /// Defines an element's background image: `background-image`
    BackgroundImage,
    /// Specifies the positioning area of the background images: `background-origin`
    BackgroundOrigin,
    /// Defines the origin of a background image: `background-position`
    BackgroundPosition,
    /// Specify whether/how the background image is tiled: `background-repeat`
    BackgroundRepeat, 	
    /// Specifies the size of the background images: `background-size`
    BackgroundSize,
    /// Sets the width, style, and color for all four sides of an element's border: `border`
    Border,
    /// Sets the width, style, and color of the bottom border of an element: `border-bottom`
    BorderBottom,
    /// Sets the color of the bottom border of an element: `border-bottom-color`
    BorderBottomColor,
    /// Defines the shape of the bottom-left border corner of an element: `border-bottom-left-radius`
    BorderBottomLeftRadius,
    /// Defines the shape of the bottom-right border corner of an element: `border-bottom-right-radius`
    BorderBottomRightRadius,
    /// Sets the style of the bottom border of an element: `border-bottom-style`
    BorderBottomStyle,
    /// Sets the width of the bottom border of an element: `border-bottom-width`
    BorderBottomWidth,
    /// Specifies whether table cell borders are connected or separated: `border-collapse`
    BorderCollapse,
    /// Sets the color of the border on all the four sides of an element: `border-color`	
    BorderColor,
    /// Specifies how an image is to be used in place of the border styles: `border-image`
    BorderImage,
    /// Specifies the amount by which the border image area extends beyond the border box: `border-image-outset`
    BorderImageOutset,
    /// Specifies whether the image-border should be repeated, rounded or stretched: `border-image-repeat`	
    BorderImageRepeat,
    /// Specifies the inward offsets of the image-border: `border-image-slice`
    BorderImageSlice,
    /// Specifies the location of the image to be used as a border: `border-image-source`
    BorderImageSource,
    /// Specifies the width of the image-border: `border-image-width`
    BorderImageWidth,
    /// Sets the width, style, and color of the left border of an element: `border-left`
    BorderLeft,
    /// Sets the color of the left border of an element: `border-left-color`
    BorderLeftColor,
    /// Sets the style of the left border of an element: `border-left-style`
    BorderLeftStyle,
    /// Sets the width of the left border of an element: `border-left-width`
    BorderLeftWidth,
    /// Defines the shape of the border corners of an element: `border-radius`
    BorderRadius,
    /// Sets the width, style, and color of the right border of an element: `border-right`
    BorderRight,
    /// Sets the color of the right border of an element: `border-right-color`
    BorderRightColor,	
    /// Sets the style of the right border of an element: `border-right-style`
    BorderRightStyle,
    /// Sets the width of the right border of an element: `border-right-width`
    BorderRightWidth,
    /// Sets the spacing between the borders of adjacent table cells: `border-spacing`
    BorderSpacing,
    /// Sets the style of the border on all the four sides of an element: `border-style`
    BorderStyle,
    /// Sets the width, style, and color of the top border of an element: `border-top`
    BorderTop,
    /// Sets the color of the top border of an element: `border-top-color`
    BorderTopColor,
    /// Defines the shape of the top-left border corner of an element: `border-top-left-radius`
    BorderTopLeftRadius,
    /// Defines the shape of the top-right border corner of an element: `border-top-right-radius`
    BorderTopRightRadius,
    /// Sets the style of the top border of an element: `border-top-style`
    BorderTopStyle,
    /// Sets the width of the top border of an element: `border-top-width`
    BorderTopWidth,
    /// Sets the width of the border on all the four sides of an element: `border-width`
    BorderWidth,
    /// Specify the location of the bottom edge of the positioned element: `bottom`
    Bottom,
    /// Applies one or more drop-shadows to the element's box: `box-shadow`
    BoxShadow,
    /// Alter the default CSS box model: `box-sizing`
    BoxSizing,
    /// Specify the position of table's caption: `caption-side`
    CaptionSide,
    /// Specifies the placement of an element in relation to floating elements: `clear`
    Clear,
    /// Defines the clipping region: `clip`
    Clip,
    /// Specify the color of the text of an element: `color`
    Color, 	
    /// Specifies the number of columns in a multi-column element: `column-count`
    ColumnCount,
    /// Specifies how columns will be filled: `column-fill`
    ColumnFill,	
    /// Specifies the gap between the columns in a multi-column element: `column-gap`
    ColumnGap,
    /// Specifies a straight line, or "rule", to be drawn between each column in a multi-column element: `column-rule`
    ColumnRule,	
    /// Specifies the color of the rules drawn between columns in a multi-column layout: `column-rule-color`
    ColumnRuleColor,
    /// Specifies the style of the rule drawn between the columns in a multi-column layout: `column-rule-style`
    ColumnRuleStyle,
    /// Specifies the width of the rule drawn between the columns in a multi-column layout: `column-rule-width`
    ColumnRuleWidth,
    /// Specifies how many columns an element spans across in a multi-column layout: `column-span`
    ColumnSpan,
    /// Specifies the optimal width of the columns in a multi-column element: `column-width`
    ColumnWidth,
    /// A shorthand property for setting column-width and column-count properties: `columns`
    Columns,
    /// Inserts generated content: `content`
    Content,
    /// Increments one or more counter values: `counter-increment`
    CounterIncrement,
    /// Creates or resets one or more counters: `counter-reset`
    CounterReset,
    /// Specify the type of cursor: `cursor`
    Cursor,
    /// Define the text direction/writing direction: `direction`
    Direction,
    /// Specifies how an element is displayed onscreen: `display`
    Display,
    /// Show or hide borders and backgrounds of empty table cells: `empty-cells`
    EmptyCells,
    /// Specifies the components of a flexible length: `flex`
    Flex,
    /// Specifies the initial main size of the flex item: `flex-basis`
    FlexBasis,
    /// Specifies the direction of the flexible items: `flex-direction`
    FlexDirection,
    /// A shorthand property for the flex-direction and the flex-wrap properties: `flex-flow`
    FlexFlow,
    /// Specifies how the flex item will grow relative to the other items inside the flex container: `flex-grow`
    FlexGrow,
    /// Specifies how the flex item will shrink relative to the other items inside the flex container: `flex-shrink`
    FlexShrink,
    /// Specifies whether the flexible items should wrap or not: `flex-wrap`
    FlexWrap,
    /// Specifies whether or not a box should float: `float`
    Float,
    /// Defines a variety of font properties within one declaration: `font`
    Font,	
    /// Defines a list of fonts for element: `font-family`
    FontFamily,
    /// Defines the font size for the text: `font-size`
    FontSize,
    /// Preserves the readability of text when font fallback occurs: `font-size-adjust`
    FontSizeAdjust,
    /// Selects a normal, condensed, or expanded face from a font: `font-stretch`
    FontStretch,
    /// Defines the font style for the text: `font-style`
    FontStyle,
    /// Specify the font variant: `font-variant`
    FontVariant,
    /// Specify the font weight of the text: `font-weight`
    FontWeight,
    /// Specify the height of an element: `height`
    Height,
    /// Specifies how flex items are aligned along the main axis of the flex container 
    /// after any flexible lengths and auto margins have been resolved: `justify-content`
    JustifyContent,
    /// Specify the location of the left edge of the positioned element: `left`
    Left,
    /// Sets the extra spacing between letters: `letter-spacing`
    LetterSpacing,
    /// Sets the height between lines of text: `line-height`
    LineHeight,
    /// Defines the display style for a list and list elements: `list-style`
    ListStyle,
    /// Specifies the image to be used as a list-item marker: `list-style-image`
    ListStyleImage,
    /// Specifies the position of the list-item marker: `list-style-position`
    ListStylePosition,
    /// Specifies the marker style for a list-item: `list-style-type`
    ListStyleType,
    /// Sets the margin on all four sides of the element: `margin`
    Margin,
    /// Sets the bottom margin of the element: `margin-bottom`
    MarginBottom,
    /// Sets the left margin of the element: `margin-left`
    MarginLeft,
    /// Sets the right margin of the element: `margin-right`
    MarginRight,
    /// Sets the top margin of the element: `margin-top` 	
    MarginTop,
    /// Specify the maximum height of an element: `max-height`
    MaxHeight,
    /// Specify the maximum width of an element: `max-width`
    MaxWidth,
    /// Specify the minimum height of an element: `min-height`
    MinHeight,
    /// Specify the minimum width of an element: `min-width`
    MinWidth,
    /// Specifies the transparency of an element: `opacity`
    Opacity, 	
    /// Specifies the order in which a flex items are displayed and laid out within a flex container: `order`
    Order, 	
    /// Sets the width, style, and color for all four sides of an element's outline: `outline`
    Outline,
    /// Sets the color of the outline: `outline-color`
    OutlineColor,
    /// Set the space between an outline and the border edge of an element: `outline-offset`
    OutlineOffset,
    /// Sets a style for an outline: `outline-style`
    OutlineStyle,
    /// Sets the width of the outline: `outline-width`
    OutlineWidth,
    /// Specifies the treatment of content that overflows the element's box: `overflow`
    Overflow, 	
    /// Specifies the treatment of content that overflows the element's box horizontally: `overflow-x`
    OverflowX, 	
    /// Specifies the treatment of content that overflows the element's box vertically: `overflow-y`
    OverflowY, 	
    /// Sets the padding on all four sides of the element: `padding`
    Padding,
    /// Sets the padding to the bottom side of an element: `padding-bottom`
    PaddingBottom,
    /// Sets the padding to the left side of an element: `padding-left`
    PaddingLeft,
    /// Sets the padding to the right side of an element: `padding-right`
    PaddingRight,
    /// Sets the padding to the top side of an element: `padding-top`
    PaddingTop,
    /// Insert a page breaks after an element: `page-break-after`
    PageBreakAfter,
    /// Insert a page breaks before an element: `page-break-before`
    PageBreakBefore,
    /// Insert a page breaks inside an element: `page-break-inside`
    PageBreakInside,
    /// Defines the perspective from which all child elements of the object are viewed: `perspective`
    Perspective, 	
    /// Defines the origin (the vanishing point for the 3D space) for the perspective property: `perspective-origin`
    PerspectiveOrigin,
    /// Specifies how an element is positioned: `position`
    Position, 	
    /// Specifies quotation marks for embedded quotations: `quotes`
    Quotes, 	
    /// Specifies whether or not an element is resizable by the user: `resize`
    Resize,
    /// Specify the location of the right edge of the positioned element: `right`
    Right,
    /// Specifies the length of the tab character: `tab-size`
    TabSize,
    /// Specifies a table layout algorithm: `table-layout`
    TableLayout,
    /// Sets the horizontal alignment of inline content: `text-align`
    TextAlign,
    /// Specifies how the last line of a block or a line right before a forced line break 
    /// is aligned when text-align is justify: `text-align-last`
    TextAlignLast,
    /// Specifies the decoration added to text: `text-decoration`
    TextDecoration,
    /// Specifies the color of the text-decoration-line: `text-decoration-color`
    TextDecorationColor,
    /// Specifies what kind of line decorations are added to the element: `text-decoration-line`
    TextDecorationLine,
    /// Specifies the style of the lines specified by the 
    /// text-decoration-line property: `text-decoration-style`
    TextDecorationStyle,
    /// Indent the first line of text: `text-indent`
    TextIndent,
    /// Specifies the justification method to use when the text-align property is set to justify: `text-justify`
    TextJustify,
    /// Specifies how the text content will be displayed, when it overflows the block containers: `text-overflow`
    TextOverflow,
    /// Applies one or more shadows to the text content of an element: `text-shadow`
    TextShadow,
    /// Transforms the case of the text: `text-transform`
    TextTransform,
    /// Specify the location of the top edge of the positioned element: `top`
    Top,
    /// Applies a 2D or 3D transformation to an element: `transform`
    Transform,
    /// Defines the origin of transformation for an element: `transform-origin`
    TransformOrigin,
    /// Specifies how nested elements are rendered in 3D space: `transform-style`
    TransformStyle,
    /// Defines the transition between two states of an element: `transition`
    Transition, 	
    /// Specifies when the transition effect will start: `transition-delay`
    TransitionDelay,
    /// Specifies the number of seconds or milliseconds a transition effect should take to complete: `transition-duration`
    TransitionDuration,
    /// Specifies the names of the CSS properties to which a transition effect should be applied: `transition-property`
    TransitionProperty,
    /// Specifies the speed curve of the transition effect: `transition-timing-function`
    TransitionTimingFunction,
    /// Sets the vertical positioning of an element relative to the current text baseline: `vertical-align`
    VerticalAlign,
    /// Specifies whether or not an element is visible: `visibility`
    Visibility,
    /// Specifies how white space inside the element is handled: `white-space`
    WhiteSpace,
    /// Specify the width of an element: `width`
    Width,
    /// Specifies how to break lines within words: `word-break`
    WordBreak,
    /// Sets the spacing between words: `word-spacing`
    WordSpacing,
    /// Specifies whether to break words when the content overflows the boundaries of its container: `word-wrap`
    WordWrap,
    /// Specifies a layering or stacking order for positioned elements: `z-index`
    ZIndex 	
}

impl Into<u8> for CssProperty {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<u64> for CssProperty {
    fn into(self) -> u64 {
        self as u64
    }
}