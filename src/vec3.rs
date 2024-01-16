pub type Vec3 = [i32; VEC3_LEN];
pub const VEC3_LEN: usize = 3;

pub trait Vector {
    fn default_vec3() -> Vec3;
    fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32;
    fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3;
}

impl Vector for Vec3 {
    fn default_vec3() -> Vec3 {
        [0; 3]
    }
    fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
        let mut c = 0;
        for i in 0..VEC3_LEN {
            c += a[i] + b[i];
        }
        c
    }
    fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
        let mut c = Self::default_vec3();
        for i in 0..3 {
            c[i] = a[i] + b[i];
        }
        c
    }
}

#[cfg(test)]
mod tests_vec3 {
    use crate::vec3::Vec3;
    use crate::vec3::Vector;

    #[test]
    fn it_works() {
        assert_eq!(<Vec3 as Vector>::default_vec3(), [0, 0, 0]);
        assert_eq!(
            <Vec3 as Vector>::vec3_scalar_sum([345, 584, 33], [394, 98, 372]),
            1826
        );
        assert_eq!(
            <Vec3 as Vector>::vec3_vector_sum([345, 584, 285], [284, 293, 194]),
            [629, 877, 479]
        );
    }
}
