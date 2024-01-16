use modules_and_tests::{pair::Pair, signed::*, unsigned::Unsigned, vec3::*};
fn main() {
    <(i32, i32) as Pair>::default_pair();
    Pair::pair_scalar_sum(&(872, 962), &(772, 72));
    Pair::pair_vector_sum(&(345, 584), &(2, 3));

    <SignedCounter as Signed>::default_signed_counter();
    <isize as Signed>::next_signed(10);
    <isize as Signed>::prev_signed(10);

    <usize as Unsigned>::default_unsigned_counter();
    <usize as Unsigned>::next_unsigned(10);

    <Vec3 as Vector>::default_vec3();
    <Vec3 as Vector>::vec3_scalar_sum([345, 584, 33], [394, 98, 372]);
    <Vec3 as Vector>::vec3_vector_sum([1, 2, 3], [4, 5, 6]);
}
