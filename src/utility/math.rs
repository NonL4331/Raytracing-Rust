use std::thread::current;

use crate::utility::vec::Vec3;
use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};

#[cfg(all(feature = "f64"))]
pub type Float = f64;

#[cfg(not(feature = "f64"))]
pub type Float = f32;

pub fn random_unit_vector() -> Vec3 {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let (mut x, mut y, mut z) = (1.0, 1.0, 1.0);
    while x * x + y * y + z * z > 1.0 {
        x = rng.gen_range(-1.0..1.0);
        y = rng.gen_range(-1.0..1.0);
        z = rng.gen_range(-1.0..1.0);
    }

    Vec3::new(x, y, z).normalised()
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let mut point = Vec3::new(1.0, 1.0, 0.0);

    while point.mag_sq() >= 1.0 {
        point.x = rng.gen_range(-1.0..1.0);
        point.y = rng.gen_range(-1.0..1.0);
    }
    point
}

pub fn random_float() -> Float {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    rng.gen()
}

pub fn near_zero(vec: Vec3) -> bool {
    let s = 0.001;
    vec.x.abs() < s && vec.y.abs() < s && vec.z.abs() < s
}

pub fn next_float(mut float: Float) -> Float {
    if float.is_infinite() && float > 0.0 {
        return float;
    }

    if float == -0.0 {
        float = 0.0
    }

    Float::from_bits(if float >= 0.0 {
        Float::to_bits(float) + 1
    } else {
        Float::to_bits(float) - 1
    })
}

pub fn previous_float(mut float: Float) -> Float {
    if float.is_infinite() && float < 0.0 {
        return float;
    }

    if float == 0.0 {
        float = -0.0
    }

    Float::from_bits(if float <= 0.0 {
        Float::to_bits(float) + 1
    } else {
        Float::to_bits(float) - 1
    })
}

pub fn gamma(n: u32) -> Float {
    let nm = n as Float * 0.5 * Float::EPSILON;
    return (nm) / (1.0 - nm);
}

pub fn sort_by_indices<T>(vec: &mut [T], mut indices: Vec<usize>) {
    for index in 0..vec.len() {
        if indices[index] != index {
            let mut current_index = index;
            loop {
                let target_index = indices[current_index];
                indices[current_index] = current_index;
                if indices[target_index] == target_index {
                    break;
                }
                vec.swap(current_index, target_index);
                current_index = target_index;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::math::sort_by_indices;

    #[test]
    fn sort_vec_by_indices() {
        let indices = vec![0, 4, 2, 1, 3];
        let mut values = ["a", "b", "c", "d", "e"];

        sort_by_indices(&mut values, indices);

        assert_eq!(values, ["a", "e", "c", "b", "d"]);
    }
}
