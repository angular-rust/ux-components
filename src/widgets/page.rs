use crate::foundation::LocalKey;

// canUpdate(Page other) -> bool
// Whether this page can be updated with the other page.
//
// createRoute(BuildContext context) -> Route<T>
// Creates the Route that corresponds to this page.

pub struct Page {
    key: LocalKey,
    name: String,
    arguments: String, // TODO: String generic should be fixed
    restoration_id: String,
}
