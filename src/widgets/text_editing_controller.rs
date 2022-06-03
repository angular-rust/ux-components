use crate::services::{TextEditingValue, TextSelection};

#[derive(Default)]
pub struct TextEditingController {
    // Whether any listeners are currently registered.
    pub has_listeners: bool,

    // The currently selected text.
    pub selection: TextSelection,

    // The current string the user is editing.
    pub text: String,

    // The current value stored in this notifier.
    pub value: TextEditingValue,
}

impl TextEditingController {
    // addListener(VoidCallback listener) → void
    // Register a closure to be called when the object changes.
    // inherited
    // buildTextSpan({required BuildContext context, TextStyle? style, required bool withComposing}) → TextSpan
    // Builds TextSpan from current editing value.
    // clear() → void
    // Set the value to empty.
    // clearComposing() → void
    // Set the composing region to an empty range.
    // dispose() → void
    // Discards any resources used by the object. After this is called, the object is not in a usable state and should be discarded (calls to addListener and removeListener will throw after the object is disposed).
    // @mustCallSuper, inherited
    // isSelectionWithinTextBounds(TextSelection selection) → bool
    // Check that the selection is inside of the bounds of text.
    // noSuchMethod(Invocation invocation) → dynamic
    // Invoked when a non-existent method or property is accessed.
    // inherited
    // notifyListeners() → void
    // Call all the registered listeners.
    // @protected, @visibleForTesting, inherited
    // removeListener(VoidCallback listener) → void
    // Remove a previously registered closure from the list of closures that are notified when the object changes.
    // inherited
}
