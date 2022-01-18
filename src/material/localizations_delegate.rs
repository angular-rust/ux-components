// isSupported(Locale locale) -> bool
// Whether resources for the given locale can be loaded by this delegate.
//
// load(Locale locale) -> Future<T>
// Start loading the resources for locale. The returned future completes when the resources have finished loading.
//
// shouldReload(covariant LocalizationsDelegate<T> old) -> bool
// Returns true if the resources for this delegate should be loaded again by calling the load method.
//
// toString() -> String
// A string representation of this object. [...]
pub struct LocalizationsDelegate;
