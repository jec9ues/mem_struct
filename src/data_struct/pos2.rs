use std::ops::{Add, AddAssign, Sub, SubAssign};
use serde::{Deserialize, Serialize};


/// A position on screen.
///
/// Normally given in points (logical pixels).
///
/// Mathematically this is known as a "point", but the term position was chosen so not to
/// conflict with the unit (one point = X physical pixels).
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Pos2 {
    /// How far to the right.
    pub x: f32,

    /// How far down.
    pub y: f32,
    // implicit w = 1
}

/// `pos2(x, y) == Pos2::new(x, y)`
#[inline(always)]
pub const fn pos2(x: f32, y: f32) -> Pos2 {
    Pos2 { x, y }
}

// ----------------------------------------------------------------------------
// Compatibility and convenience conversions to and from [f32; 2]:

impl From<[f32; 2]> for Pos2 {
    #[inline(always)]
    fn from(v: [f32; 2]) -> Self {
        Self { x: v[0], y: v[1] }
    }
}

impl From<&[f32; 2]> for Pos2 {
    #[inline(always)]
    fn from(v: &[f32; 2]) -> Self {
        Self { x: v[0], y: v[1] }
    }
}

impl From<Pos2> for [f32; 2] {
    #[inline(always)]
    fn from(v: Pos2) -> Self {
        [v.x, v.y]
    }
}

impl From<&Pos2> for [f32; 2] {
    #[inline(always)]
    fn from(v: &Pos2) -> Self {
        [v.x, v.y]
    }
}

// ----------------------------------------------------------------------------
// Compatibility and convenience conversions to and from (f32, f32):

impl From<(f32, f32)> for Pos2 {
    #[inline(always)]
    fn from(v: (f32, f32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl From<&(f32, f32)> for Pos2 {
    #[inline(always)]
    fn from(v: &(f32, f32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl From<Pos2> for (f32, f32) {
    #[inline(always)]
    fn from(v: Pos2) -> Self {
        (v.x, v.y)
    }
}

impl From<&Pos2> for (f32, f32) {
    #[inline(always)]
    fn from(v: &Pos2) -> Self {
        (v.x, v.y)
    }
}

impl Pos2 {
    /// The zero position, the origin.
    /// The top left corner in a GUI.
    /// Same as `Pos2::default()`.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }


    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        self.x.hypot(self.y)
    }

    #[inline(always)]
    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y
    }


    #[inline]
    pub fn distance_sq(self, other: Self) -> f32 {
        (self - other).length_sq()
    }

    #[inline(always)]
    pub fn floor(self) -> Self {
        pos2(self.x.floor(), self.y.floor())
    }

    #[inline(always)]
    pub fn round(self) -> Self {
        pos2(self.x.round(), self.y.round())
    }

    #[inline(always)]
    pub fn ceil(self) -> Self {
        pos2(self.x.ceil(), self.y.ceil())
    }

    /// True if all members are also finite.
    #[inline(always)]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// True if any member is NaN.
    #[inline(always)]
    pub fn any_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    #[must_use]
    #[inline]
    pub fn min(self, other: Self) -> Self {
        pos2(self.x.min(other.x), self.y.min(other.y))
    }

    #[must_use]
    #[inline]
    pub fn max(self, other: Self) -> Self {
        pos2(self.x.max(other.x), self.y.max(other.y))
    }

    #[must_use]
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}

impl std::ops::Index<usize> for Pos2 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Pos2 index out of bounds: {}", index),
        }
    }
}

impl std::ops::IndexMut<usize> for Pos2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Pos2 index out of bounds: {}", index),
        }
    }
}



impl AddAssign<Pos2> for Pos2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Pos2) {
        *self = Pos2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl SubAssign<Pos2> for Pos2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Pos2) {
        *self = Pos2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Add<Pos2> for Pos2 {
    type Output = Pos2;

    #[inline(always)]
    fn add(self, rhs: Pos2) -> Pos2 {
        Pos2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos2 {
    type Output = Pos2;

    #[inline(always)]
    fn sub(self, rhs: Pos2) -> Pos2 {
        Pos2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


impl std::fmt::Debug for Pos2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {:.1} y: {:.1})", self.x, self.y)
    }
}

