use std::ops::{Add, AddAssign, Sub, SubAssign};
use serde::{Deserialize, Serialize};


/// A position on screen.
///
/// Normally given in points (logical pixels).
///
/// Mathematically this is known as a "point", but the term position was chosen so not to
/// conflict with the unit (one point = X physical pixels).
#[derive(Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Pos3 {
    /// How far to the right.
    pub x: f32,

    /// How far down.
    pub y: f32,

    pub z: f32,
}

/// `Pos3(x, y) == Pos3::new(x, y)`
#[inline(always)]
pub const fn pos3(x: f32, y: f32, z: f32) -> Pos3 {
    Pos3 { x, y, z }
}

// ----------------------------------------------------------------------------
// Compatibility and convenience conversions to and from [f32; 3]:

impl From<[f32; 3]> for Pos3 {
    #[inline(always)]
    fn from(v: [f32; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }
}

impl From<&[f32; 3]> for Pos3 {
    #[inline(always)]
    fn from(v: &[f32; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }
}

impl From<Pos3> for [f32; 3] {
    #[inline(always)]
    fn from(v: Pos3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl From<&Pos3> for [f32; 3] {
    #[inline(always)]
    fn from(v: &Pos3) -> Self {
        [v.x, v.y, v.z]
    }
}

// ----------------------------------------------------------------------------
// Compatibility and convenience conversions to and from (f32, f32):

impl From<(f32, f32, f32)> for Pos3 {
    #[inline(always)]
    fn from(v: (f32, f32, f32)) -> Self {
        Self { x: v.0, y: v.1, z: v.2 }
    }
}

impl From<&(f32, f32, f32)> for Pos3 {
    #[inline(always)]
    fn from(v: &(f32, f32, f32)) -> Self {
        Self { x: v.0, y: v.1, z:v.2 }
    }
}

impl From<Pos3> for (f32, f32, f32) {
    #[inline(always)]
    fn from(v: Pos3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl From<&Pos3> for (f32, f32, f32) {
    #[inline(always)]
    fn from(v: &Pos3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl Pos3 {
    /// The zero position, the origin.
    /// The top left corner in a GUI.
    /// Same as `Pos3::default()`.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }


    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        self.x.hypot(self.y).hypot(self.z)
    }

    #[inline(always)]
    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }


    #[inline]
    pub fn distance_sq(self, other: Self) -> f32 {
        (self - other).length_sq()
    }

    #[inline(always)]
    pub fn floor(self) -> Self {
        pos3(self.x.floor(), self.y.floor(), self.z.floor())
    }

    #[inline(always)]
    pub fn round(self) -> Self {
        pos3(self.x.round(), self.y.round(), self.z.round())
    }

    #[inline(always)]
    pub fn ceil(self) -> Self {
        pos3(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    /// True if all members are also finite.
    #[inline(always)]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// True if any member is NaN.
    #[inline(always)]
    pub fn any_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    #[must_use]
    #[inline]
    pub fn min(self, other: Self) -> Self {
        pos3(self.x.min(other.x), self.y.min(other.y), self.z.min(other.z))
    }

    #[must_use]
    #[inline]
    pub fn max(self, other: Self) -> Self {
        pos3(self.x.max(other.x), self.y.max(other.y), self.z.max(other.z))
    }

    #[must_use]
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            z: self.z.clamp(min.z, max.z),
        }
    }
}

impl std::ops::Index<usize> for Pos3 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Pos3 index out of bounds: {}", index),
        }
    }
}

impl std::ops::IndexMut<usize> for Pos3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Pos3 index out of bounds: {}", index),
        }
    }
}

impl Eq for Pos3 {}

impl AddAssign<Pos3> for Pos3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Pos3) {
        *self = Pos3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl SubAssign<Pos3> for Pos3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Pos3) {
        *self = Pos3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Add<Pos3> for Pos3 {
    type Output = Pos3;

    #[inline(always)]
    fn add(self, rhs: Pos3) -> Pos3 {
        Pos3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Pos3 {
    type Output = Pos3;

    #[inline(always)]
    fn sub(self, rhs: Pos3) -> Pos3 {
        Pos3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}


impl std::fmt::Debug for Pos3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {:.1} y: {:.1} z: {:.1})", self.x, self.y, self.z)
    }
}

