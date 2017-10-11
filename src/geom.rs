// geom.rs    Simple geometry stuff.
//
// Copyright (c) 2017  Douglas P Lau
//
use std::f32;
use std::ops;

/// 2-dimensional vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// 3-dimensional vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 3x3 Matrix
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3x3 {
    pub e: [f32; 9],
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self{
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl ops::Mul for Vec2 {
    type Output = f32;

    /// Calculate the cross product of two Vec2
    fn mul(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, s: f32) -> Self {
        Vec2::new(self.x / s, self.y / s)
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec2::new(-self.x, -self.y)
    }
}

impl Vec2 {
    /// Create a new Vec2
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x: x, y: y }
    }
    /// Create a zero Vec2
    pub fn zero() -> Self {
        Vec2::new(0f32, 0f32)
    }
    /// Get the magnitude of a Vec2
    pub fn mag(self) -> f32 {
        self.x.hypot(self.y)
    }
    /// Create a copy normalized to unit length
    pub fn normalize(self) -> Self {
        let m = self.mag();
        if m > 0f32 {
            self / m
        } else {
            Vec2::zero()
        }
    }
    /// Calculate the distance squared between two Vec2
    pub fn dist_sq(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
    /// Calculate the distance between two Vec2
    #[allow(dead_code)]
    pub fn dist(self, other: Self) -> f32 {
        self.dist_sq(other).sqrt()
    }
    /// Get the midpoint of two Vec2
    pub fn midpoint(self, other: Self) -> Self {
        let x = (self.x + other.x) / 2f32;
        let y = (self.y + other.y) / 2f32;
        Vec2::new(x, y)
    }
    /// Create a left-hand perpendicular Vec2
    pub fn left(self) -> Self {
        Vec2::new(-self.y, self.x)
    }
    /// Create a right-hand perpendicular Vec2
    #[allow(dead_code)]
    pub fn right(self) -> Self {
        Vec2::new(self.y, -self.x)
    }
    /// Calculate winding order for two Vec2.
    ///
    /// The Vec2 should be initialized as edges pointing toward the same vertex.
    /// Returns true if the winding order is widdershins (counter-clockwise).
    #[allow(dead_code)]
    pub fn widdershins(self, other: Self) -> bool {
        // Cross product (with Z zero) is used to determine the winding order.
        (self.x * other.y) > (other.x * self.y)
    }
    /// Calculate linear interpolation of two Vec2
    ///
    /// * `t` Interpolation amount, from 0 to 1
    #[allow(dead_code)]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let x = float_lerp(self.x, other.x, t);
        let y = float_lerp(self.y, other.y, t);
        Vec2::new(x, y)
    }
    /// Calculate the relative angle to another Vec2.
    ///
    /// The result will be between `-PI` and `+PI`.
    pub fn angle_rel(self, other: Self) -> f32 {
        let pi = f32::consts::PI;
        let th = self.y.atan2(self.x) - other.y.atan2(other.x);
        if th < -pi {
            th + 2f32 * pi
        } else if th > pi {
            th - 2f32 * pi
        } else {
            th
        }
    }
}

/// Calculate linear interpolation of two values
///
/// The t value should be between 0 and 1.
pub fn float_lerp(a: f32, b: f32, t: f32) -> f32 {
    b + (a - b) * t
}

/// Calculate intersection point of two lines.
///
/// * `a0` First point on line a.
/// * `a1` Second point on line a.
/// * `b0` First point on line b.
/// * `b1` Second point on line b.
/// Returns None if the lines are colinear.
pub fn intersection(a0: Vec2,
                    a1: Vec2,
                    b0: Vec2,
                    b1: Vec2) -> Option<Vec2>
{
    let av = a0 - a1;
    let bv = b0 - b1;
    let den = av * bv;
    if den != 0f32 {
        let ca = a0 * a1;
        let cb = b0 * b1;
        let xn = bv.x * ca - av.x * cb;
        let yn = bv.y * ca - av.y * cb;
        Some(Vec2::new(xn / den, yn / den))
    } else {
        None
    }
}

impl Vec3 {
    /// Create a new Vec3
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
    /// Find the midpoint between two Vec3
    pub fn midpoint(self, other: Self) -> Self {
        let x = (self.x + other.x) / 2f32;
        let y = (self.y + other.y) / 2f32;
        let z = (self.z + other.z) / 2f32;
        Vec3::new(x, y, z)
    }
}

impl ops::MulAssign for Mat3x3 {
    fn mul_assign(&mut self, other: Self) {
        for c in 0..3 {
            let mut m = [0f32; 3];
            for r in 0..3 {
                m[r] = self.get(0, c) * other.get(r, 0) +
                       self.get(1, c) * other.get(r, 1) +
                       self.get(2, c) * other.get(r, 2);
            }
            for r in 0..3 {
                self.set(r, c, m[r]);
            }
        }
    }
}

impl ops::Mul for Mat3x3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut m = Mat3x3::new();
        for c in 0..3 {
            for r in 0..3 {
                let e = self.get(0, c) * other.get(r, 0) +
                        self.get(1, c) * other.get(r, 1) +
                        self.get(2, c) * other.get(r, 2);
                m.set(r, c, e);
            }
        }
        m
    }
}

impl ops::Mul<Vec2> for Mat3x3 {
    type Output = Vec2;

    fn mul(self, s: Vec2) -> Vec2 {
        let x = self.get(0, 0) * s.x + self.get(0, 1) * s.y + self.get(0, 2);
        let y = self.get(1, 0) * s.x + self.get(1, 1) * s.y + self.get(1, 2);
        Vec2::new(x, y)
    }
}

impl Mat3x3 {
    /// Create a new identity matrix.
    pub fn new() -> Self {
        Mat3x3 {
            e: [1f32, 0f32, 0f32,
                0f32, 1f32, 0f32,
                0f32, 0f32, 1f32]
        }
    }
    /// Create a new translation matrix.
    pub fn new_translate(tx: f32, ty: f32) -> Self {
        Mat3x3 {
            e: [1f32, 0f32,   tx,
                0f32, 1f32,   ty,
                0f32, 0f32, 1f32]
        }
    }
    /// Create a new scale matrix.
    pub fn new_scale(sx: f32, sy: f32) -> Self {
        Mat3x3 {
            e: [  sx, 0f32, 0f32,
                0f32,   sy, 0f32,
                0f32, 0f32, 1f32]
        }
    }
    /// Create a new rotation matrix.
    pub fn new_rotate(th: f32) -> Self {
        // Counter-clockwise
        let sn = th.sin();
        let cs = th.cos();
        Mat3x3 {
            e: [  cs,  -sn, 0f32,
                  sn,   cs, 0f32,
                0f32, 0f32, 1f32]
        }
    }
    /// Create a new X-axis skew matrix.
    pub fn new_skew_x(a: f32) -> Self {
        let t = a.tan();
        Mat3x3 {
            e: [1f32,    t, 0f32,
                0f32, 1f32, 0f32,
                0f32, 0f32, 1f32]
        }
    }
    /// Create a new Y-axis skew matrix.
    pub fn new_skew_y(a: f32) -> Self {
        let t = a.tan();
        Mat3x3 {
            e: [1f32, 0f32, 0f32,
                   t, 1f32, 0f32,
                0f32, 0f32, 1f32]
        }
    }
    /// Get a value.
    fn get(&self, row: usize, col: usize) -> f32 {
        self.e[row * 3 + col]
    }
    /// Set a value.
    fn set(&mut self, row: usize, col: usize, e: f32) {
        self.e[row * 3 + col] = e;
    }
    /// Translate by tx, ty
    pub fn translate(&mut self, tx: f32, ty: f32) {
        *self *= Mat3x3::new_translate(tx, ty);
    }
    /// Scale by sx, sy
    pub fn scale(&mut self, sx: f32, sy: f32) {
        *self *= Mat3x3::new_scale(sx, sy);
    }
    /// Rotate by th
    pub fn rotate(&mut self, th: f32) {
        *self *= Mat3x3::new_rotate(th);
    }
    /// Skew X-axis by a
    pub fn skew_x(&mut self, a: f32) {
        *self *= Mat3x3::new_skew_x(a);
    }
    /// Skew Y-axis by a
    pub fn skew_y(&mut self, a: f32) {
        *self *= Mat3x3::new_skew_y(a);
    }
}

#[cfg(test)]
mod test {
    use super::Vec2;

    #[test]
    fn test_vec2() {
        let a = Vec2::new(2f32, 1f32);
        let b = Vec2::new(3f32, 4f32);
        let c = Vec2::new(-1f32, 1f32);
        assert!(a + b == Vec2::new(5f32, 5f32));
        assert!(b - a == Vec2::new(1f32, 3f32));
        assert!(a * 2f32 == Vec2::new(4f32, 2f32));
        assert!(a / 2f32 == Vec2::new(1f32, 0.5f32));
        assert!(-a == Vec2::new(-2f32, -1f32));
        assert!(b.mag() == 5f32);
        assert!(a.normalize() == Vec2::new(0.8944272f32, 0.4472136f32));
        assert!(a.dist_sq(b) == 10f32);
        assert!(b.dist(Vec2::new(0f32, 0f32)) == 5f32);
        assert!(a.midpoint(b) == Vec2::new(2.5f32, 2.5f32));
        assert!(a.left() == Vec2::new(-1f32, 2f32));
        assert!(a.right() == Vec2::new(1f32, -2f32));
        assert!(a.angle_rel(b) == -0.4636476f32);
        assert!(c.angle_rel(Vec2::new(1f32, 1f32)) == 1.5707963f32);
        assert!(Vec2::new(-1f32, -1f32).angle_rel(c) == 1.5707965f32);
    }
}
