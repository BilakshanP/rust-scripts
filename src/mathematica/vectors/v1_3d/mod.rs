use crate::mathematica::angles::Angle;
use super::gen_random;

use nalgebra::ComplexField;
use rand::rngs::ThreadRng;

mod internals;
mod trait_defs;


/// A 3D vector defined by origin and some other arbitary point
#[derive(Clone, Copy)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3D {
    /// Constructs a new, `Vector3D`.
    ///
    /// The functions takes 3 elements, `x`, `y` and `z` of type `f64`.
    /// 
    /// # Examples
    /// ```
    /// let v: Vector3D = Vector3D::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Constructs a new, `Vector3D`.
    ///
    /// The functions takes 1 element, `points` of type `(f64, f64, f64)`.
    /// 
    /// # Examples
    /// ```
    /// let points: (f64, f64, f64) = (1.0, 0.0, 0.5);
    /// let v: Vector3D = Vector3D::new_from_tuple(points);
    /// ```
    pub fn new_from_tuple(points: (f64, f64, f64)) -> Self {
        Self { x: points.0, y: points.1, z: points.2 }
    }

    /// Constructs a new, `Vector3D` from the provided (inclusive) range.
    ///
    /// The functions takes 2 elements, `start` of range and `end` of range of type `f64`.
    /// 
    /// # Examples
    /// ```
    /// // The range is [1.0, 10.0] including non-integers.
    /// let v1: Vector3D = Vector3D::new_random(1.0, 10.0);
    /// 
    /// // The range is [-9.0, 9.0] including non-integers.
    /// let v2: Vector3D = Vector3D::new_random(-9.0, 9.0);
    /// 
    /// // The range is [96.5, 102.98].
    /// let v3: Vector3D = Vector3D::new_random(96.5, 102.98);
    /// ```
    pub fn new_random(start: f64, end: f64) -> Self {
        let mut thread: ThreadRng = rand::thread_rng();
    
        let x: f64 = gen_random(start, end, &mut thread);
        let y: f64 = gen_random(start, end, &mut thread);
        let z: f64 = gen_random(start, end, &mut thread);

        Self { x, y, z }
    }

    /// Returns `x`, `y` and `z` components of the vector as a tuple of `f64`'s.
    /// 
    /// # Example
    /// 
    /// ```
    /// let v: Vector3D = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(v, (1.0 as f64, 2.0 as f64, 3.0 as f64))
    /// ```
    pub fn get(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    // properties

    /// Returns the magnitude of the given vector.
    /// 
    /// # Exanple
    /// 
    /// ```
    /// let v: Vector3D = Vector::new(4.0, 3.0, 0.0);
    /// assert_eq!(v.magnitude(), 25.0)
    /// ```
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Returns the square of the magnitude of the given vector.
    /// 
    /// # Exanple
    /// 
    /// ```
    /// let v: Vector3D = Vector::new(4.0, 3.0, 0.0);
    /// assert_eq!(v.magnitude_sq(), 625.0)
    /// ```
    pub fn magnitude_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Returns the dot product of vector to itself by calling `self.magnitude_sq()`.
    /// 
    /// # Exanple
    /// 
    /// ```
    /// let v: Vector3D = Vector::new(4.0, 3.0, 0.0);
    /// assert_eq!(v.pow2(), 625.0)
    /// ```
    /// 
    /// #### _Note: `v.v = ||v||^2`_
    pub fn pow2(&self) -> f64 {
        self.magnitude_sq()
    }

    /// Returns the dot product of vector to itself by calculating `magnitude_sq` itself.
    /// 
    /// # Exanple
    /// 
    /// ```
    /// let v: Vector3D = Vector::new(4.0, 3.0, 0.0);
    /// assert_eq!(v.pow2_inline(), 625.0)
    /// ```
    /// 
    /// #### _Note: `v.v = ||v||^2`_
    pub fn pow2_inline(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Returns the `unit vector` of the given vector.
    pub fn unit(&self) -> Self {
        *self / self.magnitude()
    }

    /// Returns the `semi-unit vector` of the given vector.
    pub fn unit_denominator_sq(&self) -> Self {
        *self / self.magnitude_sq()
    }

    /// Returns direction cosines of the given vector.
    pub fn direction_cos(&self) -> (f64, f64, f64) {
        self.unit().get()
    }

    /// Returns the angles made by each `component` of the given vector with `x`, `y` and `z` axia respectively.
    pub fn axis_angles(&self) -> (Angle, Angle, Angle) {
        let (l, m, n) = self.direction_cos();

        (
            Angle::new_from_radians(l.acos()),
            Angle::new_from_radians(m.acos()),
            Angle::new_from_radians(n.acos())
        )
    }

    // modifiers

    /// Returns a vector but by taking `absolute` value of each `component` of the given vector.
    pub fn abs(&self) -> Self {
        Self { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    /// Returns a vector but by taking `decimal` value of each `component` of the given vector.
    pub fn fract(&self) -> Self {
        Self { x: self.x.fract(), y: self.y.fract(), z: self.z.fract() }
    }

    /// Returns a vector but by taking `integer` value of each `component` of the given vector.
    pub fn trunc(&self) -> Self {
        Self { x: self.x.trunc(), y: self.y.trunc(), z: self.z.trunc() }
    }

    // operations

    /// Returns `a` vector on `b` vector.
    pub fn on(&self, other: Self) -> f64 {
        self.dot(other.unit())
    }

    /// Returns angle between two vectors.
    pub fn angle(&self, other: Self) -> Angle {
        Angle::new_from_radians((self.dot(other) / (self.magnitude_sq() * other.magnitude_sq()).sqrt()).acos())
    }

    /// Returns `dot` product of two vectors.
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns `cross` product of  two vectors.
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    // compound operation properties

    /// Return the `scalar factor` by which two vectors are collinear.
    pub fn scalar_factor(&self, other: Self) -> Option<f64> {
        let (x1, y1, z1) = self.get();
        let (x2, y2, z2) = other.get();

        let x_by: f64 = x1/x2;

        if x_by == y1/y2 && x_by == z1/z2 {
            return Some(x_by);
        }

        None
    }

    /// Returns `mid-point` between two vectors.
    pub fn mid_point(&self, other: Self) -> Self {
        (*self + other) / 2.0
    }

    // checks

    /// Checks whether the given vector is `null` or not.
    pub fn is_null(&self) -> bool {
        self.get() == (0.0, 0.0, 0.0)
    }

    /// Checks whether the given vector is `unit` or not. (error margin) 
    pub fn is_unit(&self) -> bool {
        (1.0 - self.magnitude()).abs() < f64::EPSILON
    }

    ///  Checks whether the given vector is`unit` or not.
    pub fn is_unit_raw(&self) -> bool {
        self.magnitude() == 1.0
    }

    ///  Checks whether the given vector is `unit` of not, by comparing the `magnitude_sq`. (error margin)
    pub fn is_unit_sq(&self) -> bool {
        (1.0 - self.magnitude_sq()).abs() < f64::EPSILON
    }

    ///  Checks whether the given vector is `unit` of not, by comparing the `magnitude_sq`.
    pub fn is_unit_sq_raw(&self) -> bool {
        (1.0 - self.magnitude_sq()).abs() < f64::EPSILON
    }

    // compound checks

    /// Checkks whether the magnitude of two vectors is equal or not.
    pub fn are_equal(&self, other: Self) -> bool {
        self.magnitude_sq() == other.magnitude_sq()
    }

    /// Checks whether two vectors are collinear or not.
    pub fn are_collinear(&self, other: Self) -> bool {
        self.unit() == other.unit()
    }

    /// Checks whether two vectors are collinear or not by checking the `unit_sq`.
    pub fn are_collinear_sq(&self, other: Self) -> bool {
        self.unit_denominator_sq() == other.unit_denominator_sq()
    }

    /// Checks whether two vectors are collinear or not by checking the scalar factor.
    pub fn are_collinear_scalar_factor(&self, other: Self) -> bool {
        if self.scalar_factor(other).is_none() {
            return false;
        }

        true
    }

    // misc associative

    /// Find the angle between two vectors using the provided `dot` product and `vector magnitude`'s.
    /// 
    /// #### _Note: `v1.v2 = ||v1||*||v2||*cos(θ)`_
    pub fn dot_angle(dot_product: f64, vec_1_mag: f64, vec_2_mag: f64) -> Angle {
        Angle::new_from_radians((dot_product / (vec_1_mag * vec_2_mag)).acos())
    }

    /// Find the angle between two vectors using the provided `cross` product magnitude and `vector magnitude`'s.
    /// 
    /// #### _Note: `v1xv2 = ||v1||*||v2||*sin(θ)`_
    pub fn cross_angle(cross_magnitude: f64, vec_1_mag: f64, vec_2_mag: f64) -> Angle {
        Angle::new_from_radians((cross_magnitude / (vec_1_mag * vec_2_mag)).asin())
    }
}