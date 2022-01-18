use super::State;

// createElement() → StatefulElement
// Creates a StatefulElement to manage this widget's location in the tree. [...]
// override
//
// createState() → State<StatefulWidget>
// Creates the mutable state for this widget at a given location in the tree. [...]
// @factory, @protected

pub trait StatefulWidget {
    type Out;

    fn create_state(&self) -> Box<dyn State<Self::Out>>;
}
