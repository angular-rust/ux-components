// resolve(Set<MaterialState> states) -> T
// Returns a value of type T that depends on states.
pub struct MaterialStateProperty<T: Default> {
    pub val: T,
}

impl<T: Default> Default for MaterialStateProperty<T> {
    fn default() -> Self {
        Self {
            val: Default::default(),
        }
    }
}
