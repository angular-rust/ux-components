// copyWith({String? name, Object? arguments}) → RouteSettings
// Creates a copy of this route settings object with the given fields replaced with the new values.
//
// toString() -> String
// A string representation of this object.

pub struct RouteSettings<T> {
    name: String,
    arguments: Option<T>,
}
