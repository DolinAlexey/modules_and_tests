pub trait Pair {
    fn default_pair() -> Self;
    fn pair_scalar_sum(a: &Self, b: &Self) -> i32;
    fn pair_vector_sum(a: &Self, b: &Self) -> Self;
}

impl Pair for (i32, i32) {
    fn default_pair() -> Self {
        (0, 0)
    }
    fn pair_scalar_sum(a: &Self, b: &Self) -> i32 {
        a.0 + a.1 + b.0 + b.1
    }
    fn pair_vector_sum(a: &Self, b: &Self) -> Self {
        (a.0 + b.0, a.1 + b.1)
    }
}

#[cfg(test)]
mod tests_pair {
    use crate::pair::Pair;
    #[test]
    fn it_works() {
        assert_eq!(<(i32, i32) as Pair>::default_pair(), (0, 0));
        assert_eq!(Pair::pair_scalar_sum(&(872, 962), &(772, 72)), 2678);
        assert_eq!(Pair::pair_vector_sum(&(345, 584), &(2, 3)), (347, 587));
    }
}