use super::PathFillType;

#[derive(Default)]
pub struct Path {
    pub fill_type: PathFillType,
}

impl Path {
    // Create a new empty Path object. 
    pub fn new() -> Self {
        Self {
            fill_type: Default::default(),
        }
    }
    
    // Path.from(Path source)
    // Creates a copy of another Path.


    // addArc(Rect oval, double startAngle, double sweepAngle) → void
    // Adds a new sub-path with one arc segment that consists of the arc that follows the edge of the oval bounded by the given rectangle, from startAngle radians around the oval up to startAngle + sweepAngle radians around the oval, with zero radians being the point on the right hand side of the oval that crosses the horizontal line that intersects the center of the rectangle and with positive angles going clockwise around the oval. 
    // addOval(Rect oval) → void
    // Adds a new sub-path that consists of a curve that forms the ellipse that fills the given rectangle. 
    // addPath(Path path, Offset offset, {Float64List? matrix4}) → void
    // Adds the sub-paths of path, offset by offset, to this path. 
    // addPolygon(List<Offset> points, bool close) → void
    // Adds a new sub-path with a sequence of line segments that connect the given points. 
    // addRect(Rect rect) → void
    // Adds a new sub-path that consists of four lines that outline the given rectangle. 
    // addRRect(RRect rrect) → void
    // Adds a new sub-path that consists of the straight lines and curves needed to form the rounded rectangle described by the argument. 
    // arcTo(Rect rect, double startAngle, double sweepAngle, bool forceMoveTo) → void
    // If the forceMoveTo argument is false, adds a straight line segment and an arc segment. 
    // arcToPoint(Offset arcEnd, {Radius radius = Radius.zero, double rotation = 0.0, bool largeArc = false, bool clockwise = true}) → void
    // Appends up to four conic curves weighted to describe an oval of radius and rotated by rotation. 
    // close() → void
    // Closes the last sub-path, as if a straight line had been drawn from the current point to the first point of the sub-path. 
    // computeMetrics({bool forceClosed = false}) → PathMetrics
    // Creates a PathMetrics object for this path, which can describe various properties about the contours of the path. 
    // conicTo(double x1, double y1, double x2, double y2, double w) → void
    // Adds a bezier segment that curves from the current point to the given point (x2,y2), using the control points (x1,y1) and the weight w. If the weight is greater than 1, then the curve is a hyperbola; if the weight equals 1, it's a parabola; and if it is less than 1, it is an ellipse. 
    // contains(Offset point) → bool
    // Tests to see if the given point is within the path. (That is, whether the point would be in the visible portion of the path if the path was used with Canvas.clipPath.) 
    // cubicTo(double x1, double y1, double x2, double y2, double x3, double y3) → void
    // Adds a cubic bezier segment that curves from the current point to the given point (x3,y3), using the control points (x1,y1) and (x2,y2). 
    // extendWithPath(Path path, Offset offset, {Float64List? matrix4}) → void
    // Adds the sub-paths of path, offset by offset, to this path. The current sub-path is extended with the first sub-path of path, connecting them with a lineTo if necessary. 
    // getBounds() → Rect
    // Computes the bounding rectangle for this path. 
    // lineTo(double x, double y) → void
    // Adds a straight line segment from the current point to the given point. 
    // moveTo(double x, double y) → void
    // Starts a new sub-path at the given coordinate. 
    // noSuchMethod(Invocation invocation) → dynamic
    // Invoked when a non-existent method or property is accessed.
    // inherited
    // quadraticBezierTo(double x1, double y1, double x2, double y2) → void
    // Adds a quadratic bezier segment that curves from the current point to the given point (x2,y2), using the control point (x1,y1). 
    // relativeArcToPoint(Offset arcEndDelta, {Radius radius = Radius.zero, double rotation = 0.0, bool largeArc = false, bool clockwise = true}) → void
    // Appends up to four conic curves weighted to describe an oval of radius and rotated by rotation. 
    // relativeConicTo(double x1, double y1, double x2, double y2, double w) → void
    // Adds a bezier segment that curves from the current point to the point at the offset (x2,y2) from the current point, using the control point at the offset (x1,y1) from the current point and the weight w. If the weight is greater than 1, then the curve is a hyperbola; if the weight equals 1, it's a parabola; and if it is less than 1, it is an ellipse. 
    // relativeCubicTo(double x1, double y1, double x2, double y2, double x3, double y3) → void
    // Adds a cubic bezier segment that curves from the current point to the point at the offset (x3,y3) from the current point, using the control points at the offsets (x1,y1) and (x2,y2) from the current point. 
    // relativeLineTo(double dx, double dy) → void
    // Adds a straight line segment from the current point to the point at the given offset from the current point. 
    // relativeMoveTo(double dx, double dy) → void
    // Starts a new sub-path at the given offset from the current point. 
    // relativeQuadraticBezierTo(double x1, double y1, double x2, double y2) → void
    // Adds a quadratic bezier segment that curves from the current point to the point at the offset (x2,y2) from the current point, using the control point at the offset (x1,y1) from the current point. 
    // reset() → void
    // Clears the Path object of all sub-paths, returning it to the same state it had when it was created. The current point is reset to the origin. 
    // shift(Offset offset) → Path
    // Returns a copy of the path with all the segments of every sub-path translated by the given offset. 
    // toString() → String
    // A string representation of this object.
    // inherited
    // transform(Float64List matrix4) → Path
    // Returns a copy of the path with all the segments of every sub-path transformed by the given matrix. 
}