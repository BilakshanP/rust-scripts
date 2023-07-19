use super::Vector3D;
// use crate::mathematica::angles::Angle;

mod internals;
mod trait_defs;


/// A 3D vector defined by two arbitary points, i.e vector joining two points
/// or a vector defined by two other vectors, from vec_1 to vec_2
#[derive(Clone, Copy)]
pub struct Vector2P {
    vec_1: Vector3D,
    vec_2: Vector3D
}

impl Vector2P {
    pub fn new(vec_1: Vector3D, vec_2: Vector3D) -> Self {
        Self { vec_1, vec_2 }
    }

    pub fn new_raw(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> Self {
        Self {
            vec_1: Vector3D::new_from_tuple(v1),
            vec_2: Vector3D::new_from_tuple(v2)
        }
    }

    //

    pub fn get(&self) -> (Vector3D, Vector3D) {
        (self.vec_1, self.vec_2)
    }

    pub fn get_raw(&self) -> ((f64, f64, f64), (f64, f64, f64)) {
        (self.vec_1.get(), self.vec_2.get())
    }

    pub fn to_vec3d(&self) -> Vector3D {
        self.vec_2 - self.vec_1
    }

    // properties

    pub fn magnitude(&self) -> f64 {
        self.to_vec3d().magnitude()
    }

    pub fn magnitude_sq(&self) -> f64 {
        self.to_vec3d().magnitude_sq()
    }

    pub fn mid_point(&self) -> Vector3D {
        (self.vec_1 + self.vec_2) / 2.0
    }

    // operations

    pub fn section_internal(&self, m: f64, n: f64) -> Vector3D {
        ((m * self.vec_2) + (n * self.vec_1)) / (m + n)
    }

    pub fn section_external(&self, m: f64, n: f64) -> Vector3D {
        ((m * self.vec_2) - (n * self.vec_1)) / (m - n)
    }

    // formation copound checks

    pub fn are_collinear(vec_1: Vector3D, vec_2: Vector3D, vec_3: Vector3D) -> bool {
        let mut mags: [f64; 3] = [(vec_2 - vec_1).magnitude(),(vec_3 - vec_2).magnitude(),(vec_3 - vec_1).magnitude()];

        for i in 0..2 {
            for j in 0..(2 - i) {
                if mags[j] > mags[j + 1] {
                    mags.swap(j, j + 1);
                }
            }
        }

        mags[0] + mags[1] == mags[2]
    }

    pub fn are_right_triangle(vec_1: Vector3D, vec_2: Vector3D, vec_3: Vector3D) -> bool {
        let mut mag_sqs: [f64; 3] = [(vec_2 - vec_1).magnitude_sq(), (vec_3 - vec_2).magnitude_sq(), (vec_1 - vec_3).magnitude_sq()];

        for i in 0..2 {
            for j in 0..(2 - i) {
                if mag_sqs[j] > mag_sqs[j + 1] {
                    mag_sqs.swap(j, j + 1);
                }
            }
        }

        mag_sqs[0] + mag_sqs[1] == mag_sqs[2]
    }
}