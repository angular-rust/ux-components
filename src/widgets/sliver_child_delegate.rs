use crate::foundation::Key;

use super::{BuildContext, Widget};

pub trait SliverChildDelegate {
    // Returns an estimate of the number of children this delegate will build. [...]
    // estimatedChildCount → int?

    // Returns the child with the given index. [...]
    fn build(&self, context: Option<BuildContext>, index: usize) -> Option<Box<dyn Widget>> {
        todo!()
    }

    // Called at the end of layout to indicate that layout is now complete. [...]
    fn did_finish_layout(&self, first_index: usize, last_index: usize) {}

    // Returns an estimate of the max scroll extent for all the children. [...]
    fn estimate_max_scroll_offset(
        &self,
        first_index: usize,
        last_index: usize,
        leading_scroll_offset: f32,
        trailing_scroll_offset: f32,
    ) -> Option<f32> {
        None
    }

    // Find index of child element with associated key. [...]
    fn find_index_by_key(&self, key: Key) -> Option<usize> {
        None
    }

    // Called whenever a new instance of the child delegate class is provided to the sliver. [...]
    fn should_rebuild(&self, old_delegate: Box<dyn SliverChildDelegate>) -> bool {
        false
    }
}

pub struct NoneSliverChildDelegate;

impl SliverChildDelegate for NoneSliverChildDelegate {}
