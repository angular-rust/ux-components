// The accelerate easing curve in the Material specification.
// Elements exiting a screen use acceleration easing, where they start at rest and end at peak velocity.

// const Curve accelerateEasing = Cubic(0.4, 0.0, 1.0, 1.0); // animation Curve

// The decelerate easing curve in the Material specification.
// Incoming elements are animated using deceleration easing, which starts a transition at peak velocity 
// (the fastest point of an element’s movement) and ends at rest.

// const Curve decelerateEasing = Cubic(0.0, 0.0, 0.2, 1.0);

// Used to annotate an instance or static method m. Indicates that m must either be abstract or must return 
// a newly allocated object or null. In addition, every method that either implements or overrides m is 
// implicitly annotated with this same annotation.
// Tools, such as the analyzer, can provide feedback if
//     the annotation is associated with anything other than a method, or
//     a method that has this annotation can return anything other than a newly allocated object or null.

// const _Factory factory = _Factory();

// Used to annotate a class C. Indicates that C and all subtypes of C must be immutable.
// A class is immutable if all of the instance fields of the class, whether defined directly or inherited, are final.
// Tools, such as the analyzer, can provide feedback if
//     the annotation is associated with anything other than a class, or
//     a class that has this annotation or extends, implements or mixes in a class that has this annotation is not immutable.

// const Immutable immutable = Immutable();

// An eyeballed value that moves the cursor slightly left of where it is rendered for text on Android so its 
// positioning more accurately matches the native iOS text cursor positioning.
// This value is in device pixels, not logical pixels as is typically used throughout the codebase.

// const int iOSHorizontalOffset = -2;

// An animation that is always complete.
// Using this constant involves less overhead than building an AnimationController with an initial value of 1.0. 
// This is useful when an API expects an animation but you don't actually want to animate anything.

// const Animation<double> kAlwaysCompleteAnimation = _AlwaysCompleteAnimation();

// An animation that is always dismissed.
// Using this constant involves less overhead than building an AnimationController with an initial value of 0.0. 
// This is useful when an API expects an animation but you don't actually want to animate anything.

// const Animation<double> kAlwaysDismissedAnimation = _AlwaysDismissedAnimation();

// The height of the bottom navigation bar.
pub const BOTTOM_NAVIGATION_BAR_HEIGHT: f32 = 56.0;

// Map of elevation offsets used by material design to BoxShadow definitions.
// The following elevations have defined shadows: 1, 2, 3, 4, 6, 8, 9, 12, 16, 24.
// Each entry has three shadows which must be combined to obtain the defined effect for that elevation.
// This is useful when simulating a shadow with a BoxDecoration or other class that uses a list of BoxShadow objects.
// See also:
//     Material, which takes an arbitrary double for its elevation and generates a shadow dynamically.
//     material.io/design/environment/elevation.html

// const Map<int, List<BoxShadow>> kElevationToShadow = _elevationToShadow;

// The margin that a FloatingActionButton should leave between it and the edge of the screen.
// FloatingActionButtonLocation.endFloat uses this to set the appropriate margin between the FloatingActionButton and the end of the screen.
pub const FLOATING_ACTION_BUTTON_MARGIN: f32 = 16.0;

// The amount of time the FloatingActionButton takes to transition in or out.
// The Scaffold uses this to set the duration of FloatingActionButton motion, entrance, and exit animations.

// const Duration kFloatingActionButtonSegue = Duration(milliseconds: 200);

// The fraction of a circle the FloatingActionButton should turn when it enters.
// Its value corresponds to 0.125 of a full circle, equivalent to 45 degrees or pi/4 radians.
pub const FLOATING_ACTION_BUTTON_TURN_INTERVAL: f32 = 0.125;

// The border radii used by the various kinds of material in material design.
// See also:
//     MaterialType
//     Material

// const Map<MaterialType, BorderRadius?> kMaterialEdges = <MaterialType, BorderRadius?>{
//     MaterialType.canvas: null,
//     MaterialType.card: BorderRadius.all(Radius.circular(2.0)),
//     MaterialType.circle: null,
//     MaterialType.button: BorderRadius.all(Radius.circular(2.0)),
//     MaterialType.transparency: null,
// };

//   The padding added around material list items.

// const EdgeInsets kMaterialListPadding = EdgeInsets.symmetric(vertical: 8.0);

// If a FloatingActionButton is used on a Scaffold in certain positions, it is moved kMiniButtonOffsetAdjustment pixels closer to the edge of the screen.
// This is intended to be used with FloatingActionButton.mini set to true, so that the floating action button appears 
// to align with CircleAvatars in the ListTile.leading slot of a ListTile in a ListView in the Scaffold.body.
// More specifically:
//     In the following positions, the FloatingActionButton is moved horizontally closer to the edge of the screen:
//         FloatingActionButtonLocation.miniStartTop
//         FloatingActionButtonLocation.miniStartFloat
//         FloatingActionButtonLocation.miniStartDocked
//         FloatingActionButtonLocation.miniEndTop
//         FloatingActionButtonLocation.miniEndFloat
//         FloatingActionButtonLocation.miniEndDocked
//     In the following positions, the FloatingActionButton is moved vertically closer to the bottom of the screen:
//         FloatingActionButtonLocation.miniStartFloat
//         FloatingActionButtonLocation.miniCenterFloat
//         FloatingActionButtonLocation.miniEndFloat
pub const MINI_BUTTON_OFFSET_ADJUSTMENT: f32 = 4.0;

// The minimum dimension of any interactive region according to Material guidelines.
// This is used to avoid small regions that are hard for the user to interact with. It applies to both dimensions of a region, 
// so a square of size kMinInteractiveDimension x kMinInteractiveDimension is the smallest acceptable region that should respond to gestures.
// See also:
//     kMinInteractiveDimensionCupertino
//     The Material spec on touch targets at material.io/design/usability/accessibility.html#layout-typography.
pub const MIN_INTERACTIVE_DIMENSION: f32 = 48.0;

// The value of the alpha channel to use when drawing a circular material ink response.
pub const RADIAL_REACTION_ALPHA: i32 = 0x1F;

// The amount of time a circular material ink response should take to expand to its full size.

// const Duration kRadialReactionDuration = Duration(milliseconds: 100);

// The default radius of a circular material ink response in logical pixels.
pub const RADIAL_REACTION_RADIUS: f32 = 20.0;

// The horizontal padding included by Tabs.

// const EdgeInsets kTabLabelPadding = EdgeInsets.symmetric(horizontal: 16.0);

// The duration of the horizontal scroll animation that occurs when a tab is tapped.

// const Duration kTabScrollDuration = Duration(milliseconds: 300);

// The height of a tab bar containing text.
pub const TEXT_TAB_BAR_HEIGHT: f32 = MIN_INTERACTIVE_DIMENSION;

// The duration over which theme changes animate by default.

// const Duration kThemeAnimationDuration = Duration(milliseconds: 200);

// The amount of time theme change animations should last.

// const Duration kThemeChangeDuration = Duration(milliseconds: 200);

// The height of the toolbar component of the AppBar.
pub const TOOLBAR_HEIGHT: f32 = 56.0;

// The standard easing curve in the Material specification.
// Elements that begin and end at rest use standard easing. They speed up quickly and slow down gradually, 
// in order to emphasize the end of the transition.
// See also:
//     material.io/design/motion/speed.html#easing

// const Curve standardEasing = Curves.fastOutSlowIn;

