#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StackFit {
    // The constraints passed to the stack from its parent are loosened.
    // For example, if the stack has constraints that force it to 350x600, then this would allow the 
    // non-positioned children of the stack to have any width from zero to 350 and any height from zero to 600.
    // See also:
    //     Center, which loosens the constraints passed to its child and then centers the child in itself.
    //     BoxConstraints.loosen, which implements the loosening of box constraints.
    Loose = 0,
    // The constraints passed to the stack from its parent are tightened to the biggest size allowed.
    // For example, if the stack has loose constraints with a width in the range 10 to 100 and a 
    // height in the range 0 to 600, then the non-positioned children of the stack 
    // would all be sized as 100 pixels wide and 600 high.
    Expand = 1,
    // The constraints passed to the stack from its parent are passed 
    // unmodified to the non-positioned children.
    // For example, if a Stack is an Expanded child of a Row, the horizontal 
    // constraints will be tight and the vertical constraints will be loose.
    Passthrough = 2,
}

impl Default for StackFit {
    fn default() -> Self {
        Self::Loose
    }
}
