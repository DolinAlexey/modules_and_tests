use modules_and_tests::pair::Pair;

fn main() {
    println!("{:?}", <(i32, i32) as Pair>::default_pair());
    println!("{:?}", Pair::pair_scalar_sum(&(872, 962), &(772, 72)));
    println!("{:?}", Pair::pair_vector_sum(&(345, 584), &(2, 3)));

    println!("{:?}", Signed::default_signed_counter());
}
