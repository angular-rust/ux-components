/*
getAnimationRestart(double previousValue) -> double
Gets the progress value to restart a motion animation from when the animation is interrupted.
getOffset({required Offset begin, required Offset end, required double progress}) -> Offset
Gets the FloatingActionButton's position relative to the origin of the Scaffold based on progress.
getRotationAnimation({required Animation<double> parent}) -> Animation<double>
Animates the rotation of Scaffold.floatingActionButton.
getScaleAnimation({required Animation<double> parent}) -> Animation<double>
Animates the scale of the FloatingActionButton.
*/
pub struct FloatingActionButtonAnimator;

impl Default for FloatingActionButtonAnimator {
    fn default() -> Self {
        Self {}
    }
}
