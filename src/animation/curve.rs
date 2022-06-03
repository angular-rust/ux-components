pub trait Curve {
    // // Returns a new curve that is the reversed inversion of this one. [...]
    // flipped: Curve,

    // Returns the value of the curve at point t.
    fn transform(&self, t: f32) -> f32;
}

#[derive(Default)]
pub struct NoneCurve;

impl Curve for NoneCurve {
    fn transform(&self, t: f32) -> f32 {
        todo!()
    }
}
