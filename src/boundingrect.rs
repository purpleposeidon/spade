// Copyright 2017 The Spade Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::misc::max_inline;
use crate::point_traits::{PointN, PointNExtensions};
use crate::traits::SpatialObject;
use num::{one, zero, Signed};

/// An axis aligned minimal bounding rectangle.
///
/// An axis aligned minimal bounding rectangle is the smallest rectangle that completely
/// surrounds an object and is aligned along all axes. The vector type `V`'s dimension
/// determines if this is a rectangle, a box or a higher dimensional volume.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde_serialize", derive(Serialize, Deserialize))]
pub struct BoundingRect<V: PointN> {
    lower: V,
    upper: V,
}

impl<V> BoundingRect<V>
where
    V: PointN,
{
    /// Creates a bounding rectangle that contains exactly one point.
    ///
    /// This will create a bounding rectangle with `lower == upper == point`.
    pub fn from_point(point: V) -> BoundingRect<V> {
        BoundingRect {
            lower: point.clone(),
            upper: point,
        }
    }

    /// Create a bounding rectangle from a set of points
    ///
    /// # Panics
    /// Panics if the given iterator is empty.
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator<Item = V>,
    {
        let mut iter = points.into_iter();
        let first = iter.next();
        let mut result = Self::from_point(first.expect("Expected at least one point"));
        for p in iter {
            result.add_point(p);
        }
        result
    }

    /// Creates a bounding rectangle that contains two points.
    pub fn from_corners(corner1: &V, corner2: &V) -> BoundingRect<V> {
        BoundingRect {
            lower: corner1.min_point(&corner2),
            upper: corner1.max_point(&corner2),
        }
    }

    /// Returns the lower corner of the bounding rectangle.
    ///
    /// The lower corner has the smaller coordinates.
    pub fn lower(&self) -> V {
        self.lower.clone()
    }

    /// Returns the upper corner of the bounding rectangle.
    ///
    /// The upper corner has the larger coordinates.
    pub fn upper(&self) -> V {
        self.upper.clone()
    }

    /// Checks if a point is contained within the bounding rectangle.
    ///
    /// A point lying exactly on the bounding rectangle's border is also contained.
    #[inline]
    pub fn contains_point(&self, point: &V) -> bool {
        self.lower.all_comp_wise(&point, |l, r| l <= r)
            && self.upper.all_comp_wise(&point, |l, r| l >= r)
    }

    /// Checks if another bounding rectangle is completley contained withing this rectangle.
    ///
    /// A rectangle is contained if and only if all four corner are contained (see `contains_point`).
    #[inline]
    pub fn contains_rect(&self, rect: &BoundingRect<V>) -> bool {
        self.lower.all_comp_wise(&rect.lower(), |l, r| l <= r)
            && self.upper.all_comp_wise(&rect.upper(), |l, r| l >= r)
    }

    /// Enlarges this bounding rectangle to contain a point.
    ///
    /// If the point is already contained, nothing will be changed.
    /// Otherwise, this will enlarge `self` to be just large enough
    /// to contain the new point.
    #[inline]
    pub fn add_point(&mut self, point: V) {
        self.lower = self.lower.min_point(&point);
        self.upper = self.upper.max_point(&point);
    }

    /// Enlarges this bounding rectangle to contain a rectangle.
    ///
    /// If the rectangle is already contained, nothing will be changed.
    /// Otherwise, this will enlarge `self` to be just large enough
    /// to contain the new rectangle.
    #[inline]
    pub fn add_rect(&mut self, rect: &BoundingRect<V>) {
        self.lower = self.lower.min_point(&rect.lower);
        self.upper = self.upper.max_point(&rect.upper);
    }

    /// Returns the rectangle's area.
    pub fn area(&self) -> V::Scalar {
        let diag = self.upper().sub(&self.lower());
        diag.fold(one(), |acc, value| max_inline(acc * value, zero()))
    }

    /// Returns half of the rectangle's margin, thus `width + height`.
    pub fn half_margin(&self) -> V::Scalar {
        let diag = self.upper().sub(&self.lower());
        diag.fold(zero(), |acc, value| max_inline(acc + value, zero()))
    }

    /// Returns the rectangle's center.
    pub fn center(&self) -> V {
        let two = one::<V::Scalar>() + one::<V::Scalar>();
        self.lower()
            .add(&(self.upper().sub(&self.lower()).div(two)))
    }

    /// Returns the intersection of this and another bounding rectangle.
    ///
    /// If the rectangles do not intersect, a bounding rectangle with an area and
    /// margin of zero is returned.
    pub fn intersect(&self, other: &BoundingRect<V>) -> BoundingRect<V> {
        BoundingRect {
            lower: self.lower.max_point(&other.lower),
            upper: self.upper.min_point(&other.upper),
        }
    }

    /// Returns true if this and another bounding rectangle intersect each other.
    /// If the rectangles just "touch" each other at one side, true is returned.
    pub fn intersects(&self, other: &BoundingRect<V>) -> bool {
        self.lower.all_comp_wise(&other.upper(), |l, r| l <= r)
            && self.upper.all_comp_wise(&other.lower(), |l, r| l >= r)
    }

    #[doc(hidden)]
    pub fn min_point(&self, point: &V) -> V {
        self.upper.min_point(&self.lower.max_point(&point))
    }

    #[doc(hidden)]
    pub fn min_dist2(&self, point: &V) -> V::Scalar {
        self.min_point(point).sub(point).length2()
    }

    #[doc(hidden)]
    pub fn max_dist2(&self, point: &V) -> V::Scalar {
        let l = self.lower();
        let u = self.upper();
        let d1: V = l.sub(point).map(|v| v.abs());
        let d2: V = u.sub(point).map(|v| v.abs());
        let max_delta = d1.max_point(&d2);
        max_delta.length2()
    }

    #[doc(hidden)]
    pub fn min_max_dist2(&self, point: &V) -> V::Scalar {
        let l = self.lower().sub(point);
        let u = self.upper().sub(point);
        let (mut min, mut max) = (V::new(), V::new());
        for i in 0..V::dimensions() {
            if l.nth(i).abs() < u.nth(i).abs() {
                *min.nth_mut(i) = l.nth(i).clone();
                *max.nth_mut(i) = u.nth(i).clone();
            } else {
                *min.nth_mut(i) = u.nth(i).clone();
                *max.nth_mut(i) = l.nth(i).clone();
            }
        }
        let mut result = zero();
        for i in 0..V::dimensions() {
            let mut p = min.clone();
            // Only set one component to the maximum distance
            *p.nth_mut(i) = max.nth(i).clone();
            let new_dist = p.length2();
            if new_dist < result || i == 0 {
                result = new_dist
            }
        }
        result
    }
}

impl<V> SpatialObject for BoundingRect<V>
where
    V: PointN,
{
    type Point = V;

    fn mbr(&self) -> BoundingRect<V> {
        self.clone()
    }

    fn distance2(&self, point: &Self::Point) -> V::Scalar {
        self.min_dist2(point)
    }

    fn contains(&self, point: &Self::Point) -> bool {
        self.contains_point(point)
    }
}

#[cfg(test)]
mod test {
    use super::BoundingRect;
    use crate::traits::SpatialObject;

    #[test]
    fn test_add_points() {
        let points = [[0.0, 1.0f32], [1.0, 0.5], [2.0, -2.0], [0.0, 0.0]];
        let bb = BoundingRect::from_points(points.iter().cloned());
        assert_eq!(
            bb,
            BoundingRect {
                lower: [0.0, -2.0],
                upper: [2.0, 1.0],
            }
        );
    }

    #[test]
    fn test_bounding_rect_distance2() {
        let rect = BoundingRect {
            lower: [0.0, 0.0],
            upper: [1.0, 1.0],
        };
        assert_eq!(rect.distance2(&[0.0, 0.5]), 0.0);
        assert_eq!(rect.distance2(&[0.0, -1.0]), 1.0);
        assert_eq!(rect.distance2(&[0.2, 0.7]), 0.0);
        assert_eq!(rect.distance2(&[2.0, 2.0]), 2.0);
        assert_eq!(rect.distance2(&[2.0, 0.5]), 1.0);
    }
}
