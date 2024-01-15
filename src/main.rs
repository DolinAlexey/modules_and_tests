use modules_and_tests::pair::Pair;
use modules_and_tests::signed::Signed;

fn main() {
    println!("{:?}", <(i32, i32) as Pair>::default_pair());
    println!("{:?}", Pair::pair_scalar_sum(&(872, 962), &(772, 72)));
    println!("{:?}", Pair::pair_vector_sum(&(345, 584), &(2, 3)));

    println!("{:?}", Signed::default_signed_counter());
    println!("{:?}", Signed::next_signed(&10, isize));
    println!("{:?}", Signed::prev_signed(&10));
}
