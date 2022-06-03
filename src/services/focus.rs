#![allow(dead_code)]

use crate::{
    elements::{CanvasElement, Element},
    foundation::Slot,
    foundation::{KeyEvent, MouseEvent, TextEvent},
};

pub struct Focus {
    pub canvas: CanvasElement,
    onmousemove: Option<Slot>,
    onmousedown: Option<Slot>,
    onmouseup: Option<Slot>,
    onmousewheel: Option<Slot>,
    onkeydown: Option<Slot>,
    onkeyup: Option<Slot>,
    ontextinput: Option<Slot>,
}

impl Focus {
    pub fn new(canvas: CanvasElement) -> Self {
        let instance = Self {
            canvas,
            onkeydown: None,
            onkeyup: None,
            onmousedown: None,
            onmousemove: None,
            onmouseup: None,
            onmousewheel: None,
            ontextinput: None,
        };

        // let component = instance.canvas.as_ref();
        // instance.onmousemove = Some(component.onmousemove.listen(box |e| instance.mousemove(e)));
        // instance.onmousedown = Some(component.onmousedown.listen(box |e| instance.mousedown(e)));
        // instance.onmouseup = Some(component.onmouseup.listen(box |e| instance.mouseup(e)));
        // instance.onmousewheel = Some(component.onmousewheel.listen(box |e| instance.mousewheel(e)));

        // instance.onkeydown = Some(component.onkeydown.listen(box |e| instance.keydown(e)));
        // instance.onkeyup = Some(component.onkeyup.listen(box |e| instance.keyup(e)));
        // instance.ontextinput = Some(component.ontextinput.listen(box |e| instance.textinput(e)));

        instance
    }

    pub fn destroy(&self) {
        // let component = self.canvas.as_ref().borrow();

        // if let Some(slot) = self.onmousemove {
        //     component.onmousemove.remove(slot);
        // }

        // if let Some(slot) = self.onmousedown {
        //     component.onmousedown.remove(slot);
        // }

        // if let Some(slot) = self.onmouseup {
        //     component.onmouseup.remove(slot);
        // }

        // if let Some(slot) = self.onmousewheel {
        //     component.onmousewheel.remove(slot);
        // }

        // if let Some(slot) = self.onkeydown {
        //     component.onkeydown.remove(slot);
        // }

        // if let Some(slot) = self.onkeyup {
        //     component.onkeyup.remove(slot);
        // }

        // if let Some(slot) = self.ontextinput {
        //     component.ontextinput.remove(slot);
        // }
    }

    fn keydown(&self, e: &mut KeyEvent) {
        if let Some(focused) = self.canvas.focused() {
            if focused.key_input() {
                focused.keydown(e);
            }
        }
    }

    fn keyup(&self, e: &mut KeyEvent) {
        if let Some(focused) = self.canvas.focused() {
            if focused.key_input() {
                focused.keyup(e);
            }
        }
    }

    fn textinput(&self, e: &mut TextEvent) {
        if let Some(focused) = self.canvas.focused() {
            if focused.key_input() {
                focused.textinput(e);
            }
        }
    }

    fn mousedown(&self, e: &mut MouseEvent) {
        if let Some(focused) = self.canvas.focused() {
            if focused.mouse_input() {
                focused.mousedown(e);
            }
        }

        if self.marked_mouse() {
            if let Some(marked) = self.canvas.marked() {
                marked.mousedown(e);
            }
        }
    }

    fn mouseup(&self, e: &mut MouseEvent) {
        if let Some(focused) = self.canvas.focused() {
            if focused.mouse_input() {
                focused.mouseup(e);
            }
        }

        if self.marked_mouse() {
            if let Some(marked) = self.canvas.marked() {
                marked.mouseup(e);
            }
        }
    }

    fn mousewheel(&self, e: &mut MouseEvent) {
        if let Some(focused) = self.canvas.focused() {
            if focused.mouse_input() {
                focused.mousewheel(e);
            }
        }

        if self.marked_mouse() {
            if let Some(marked) = self.canvas.marked() {
                marked.mousewheel(e);
            }
        }
    }

    fn marked_mouse(&self) -> bool {
        if let Some(marked) = self.canvas.marked() {
            if let Some(focused) = self.canvas.focused() {
                return (marked != focused) && marked.mouse_input();
            }
        }
        false
    }

    fn get_markable(&self, target: &dyn Element, x: f32, y: f32) -> Option<&dyn Element> {
        let highest = target.topmost_child_at_point(x, y);
        if let Some(current) = highest {
            if current.id() == target.id() {
                return None;
            } else {
                // let canvas = target.canvas.as_ref();
                // // now if we have the highest possible control,
                // // we have to walk backward up the tree,
                // // looking for the highest one with mouse input

                // let canvas = target.canvas.as_ref();

                // loop {
                //     if highest.is_none() || highest == canvas {
                //         return highest;
                //     }

                //     if current.mouse_input {
                //         return current;
                //     }

                //     if current.parent().is_none() {
                //         return highest;
                //     }

                //     highest = current.parent();
                // }
                todo!()
            }
        }

        None
    }

    fn mousemove(&self, e: &mut MouseEvent) {
        let handled = match self.canvas.captured() {
            Some(captured) => {
                if captured.mouse_input() {
                    if let Some(markable) =
                        self.get_markable(captured.as_ref(), e.x as f32, e.y as f32)
                    {
                        self.mark_control(markable, e);
                    }

                    if self.marked_mouse() {
                        if let Some(marked) = self.canvas.marked() {
                            marked.mousemove(e);
                        }
                    }
                    true
                } else {
                    false
                }
            }
            None => false,
        };

        let component = self.canvas.component.borrow();
        if !handled && component.ishovered {
            //if the mouse has left the current marked control, unmark
            if let Some(marked) = self.canvas.marked() {
                if !marked.contains(e.x as f32, e.y as f32) {
                    self.unmark_control(marked.as_ref(), e);
                }
            }

            // the focused control always gets the events
            if let Some(focused) = self.canvas.focused() {
                if focused.mouse_input() {
                    focused.mousemove(e);
                }
            }

            //get the marked control from the canvas
            if let Some(markable) = self.get_markable(&self.canvas, e.x as f32, e.y as f32) {
                self.mark_control(markable, e);
            }

            if self.marked_mouse() {
                if let Some(marked) = self.canvas.marked() {
                    marked.mousemove(e);
                }
            }
        }
    }

    fn reset_marked(&self, control: &dyn Element, e: &mut MouseEvent) {
        if control.is_marked() {
            self.unmark_control(control, e);
        }
    }

    fn get_focusable(&self, x: f32, y: f32) -> Option<&Box<dyn Element>> {
        if let Some(captured) = self.canvas.captured() {
            Some(captured)
        } else {
            self.canvas.topmost_at_point(x, y)
        }
    }

    fn find_marked(&self, e: &mut MouseEvent) -> Option<&Box<dyn Element>> {
        match self.get_focusable(e.x as f32, e.y as f32) {
            Some(marked) => {
                self.mark_control(marked.as_ref(), e);
                Some(marked)
            }
            None => None,
        }
    }

    fn unmark_control(&self, control: &dyn Element, e: &mut MouseEvent) {
        if control.is_marked() {
            control.unmark();

            if control.mouse_input() {
                control.mouseleave(e);
            }
        }
    }

    fn mark_control(&self, control: &dyn Element, e: &mut MouseEvent) {
        if !control.is_marked() {
            if let Some(marked) = self.canvas.marked() {
                self.reset_marked(marked.as_ref(), e);
            }
            control.mark();

            if control.mouse_input() {
                control.mouseenter(e);
            }
        }
    }
}
