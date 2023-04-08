use super::*;

/// A geometric shape with optional fill and stroke.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Shape {
    /// The shape's geometry.
    pub geometry: Geometry,
    /// The shape's background fill.
    pub fill: Option<Paint>,
    /// The shape's border stroke.
    pub stroke: Option<Stroke>,
}

/// A shape's geometry.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Geometry {
    /// A line to a point (relative to its position).
    Line(Point),
    /// A rectangle with its origin in the topleft corner.
    Rect(Size),
    /// A bezier path.
    Path(Path),
}

impl Geometry {
    /// Fill the geometry without a stroke.
    pub fn filled(self, fill: Paint) -> Shape {
        Shape { geometry: self, fill: Some(fill), stroke: None }
    }

    /// Stroke the geometry without a fill.
    pub fn stroked(self, stroke: Stroke) -> Shape {
        Shape { geometry: self, fill: None, stroke: Some(stroke) }
    }

    /// Calculate the geometries axis-aligned bounding-box
    pub fn bounds(&self) -> Size {
        match self {
            Self::Line(point) => Size::new(point.x, point.y),
            Self::Rect(size) => size.clone(),
            Self::Path(path) => path.bounds(),
        }
    }
}
