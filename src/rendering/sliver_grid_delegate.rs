use super::{SliverConstraints, SliverGridLayout};

pub trait SliverGridDelegate {
    // Returns information about the size and position of the tiles in the grid.
    fn get_layout(&self, constraints: SliverConstraints) -> Box<dyn SliverGridLayout> {
        todo!()
    }
    
    // Override this method to return true when the children need to be laid out. [...]
    fn should_relayout(&self, old_delegate: Box<dyn SliverGridDelegate>) -> bool {
        false
    }
}

pub struct NoneSliverGridDelegate;

impl SliverGridDelegate for NoneSliverGridDelegate {
    
}