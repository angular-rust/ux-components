// use super::{State, StatefulWidget};

// TODO: String generic should be fixed
// pub struct GlobalKey<T: State<dyn StatefulWidget>>(T);

pub struct GlobalKey<T: Default>(T);

impl<T: Default> Default for GlobalKey<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
