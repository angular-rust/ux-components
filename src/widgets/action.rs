use super::Intent;

/*
addActionListener(ActionListenerCallback listener) -> void
Register a callback to listen for changes to the state of this action. [...]
@mustCallSuper

consumesKey(covariant T intent) -> bool
Indicates whether this action should treat key events mapped to this action as being "handled" when it is invoked via the key event.

invoke(covariant T intent) -> Object?
Called when the action is to be performed. [...]
@protected

isEnabled(covariant T intent) -> bool
Returns true if the action is enabled and is ready to be invoked.

notifyActionListeners() -> void
Call all the registered listeners. [...]
@protected, @visibleForTesting

removeActionListener(ActionListenerCallback listener) -> void
Remove a previously registered closure from the list of closures that are notified when the object changes. [...]
*/

pub struct Action<T: Intent>(T);
