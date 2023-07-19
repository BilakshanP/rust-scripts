mod v1_3d;
mod v2_3d;

pub use v1_3d::Vector3D;
pub use v2_3d::Vector2P;

use rand::{Rng, rngs::ThreadRng};

pub fn gen_random<T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform>(start: T, end: T, thread: &mut ThreadRng) -> T {
    thread.gen_range(start..=end)
}