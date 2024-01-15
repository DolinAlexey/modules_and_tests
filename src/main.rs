use modules_and_tests::pair::Pair;
use modules_and_tests::signed::Signed;
use modules_and_tests::unsigned::Unsigned;
use modules_and_tests::vec3::Vector;

fn main() {

    println!("{:?}", <(i32, i32) as Pair>::default_pair());
    println!("{:?}", Pair::pair_scalar_sum(&(872, 962), &(772, 72)));
    println!("{:?}", Pair::pair_vector_sum(&(345, 584), &(2, 3)));

    println!("{:?}", <isize as Signed>::default_signed_counter());
    println!("{:?}", <isize as Signed>::next_signed(10));
    println!("{:?}", <isize as Signed>::prev_signed(10));

    println!("{:?}", <usize as Unsigned>::default_unsigned_counter());
    println!("{:?}", <usize as Unsigned>::next_unsigned(10));

    println!("{:?}", Vec3 as Vector>::default_vec3());
    println!("{:?}", Vector::vec3_scalar_sum([345, 584, 33], [394, 98, 372]));
    println!("{:?}", Vector::vec3_vector_sum([1, 2, 3], [4, 5, 6]));
}
