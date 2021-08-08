use na::RealField;

use crate::math::{Point, Vector};
use crate::procedural::TriMesh;

/// A sample point and its associated tangent.
pub enum PathSample<N: RealField + Copy> {
    /// A point that starts a new path.
    StartPoint(Point<N>, Vector<N>),
    /// A point that is inside of the path currently generated.
    InnerPoint(Point<N>, Vector<N>),
    /// A point that ends the path currently generated.
    EndPoint(Point<N>, Vector<N>),
    /// Used when the sampler does not have any other points to generate.
    EndOfSample,
}

/// A curve sampler.
pub trait CurveSampler<N: RealField + Copy> {
    /// Returns the next sample point.
    fn next(&mut self) -> PathSample<N>;
}

/// A pattern that is replicated along a path.
///
/// It is responsible of the generation of the whole mesh.
pub trait StrokePattern<N: RealField + Copy> {
    /// Generates the mesh using this pattern and the curve sampled by `sampler`.
    fn stroke<C: CurveSampler<N>>(&mut self, sampler: &mut C) -> TriMesh<N>;
}
