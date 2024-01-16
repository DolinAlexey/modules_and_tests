type UnsignedCounter = usize;
pub trait Unsigned {
    fn default_unsigned_counter() -> UnsignedCounter;
    fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter;
}

impl Unsigned for UnsignedCounter {
    fn default_unsigned_counter() -> UnsignedCounter {
        0
    }
    fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
        counter + 1
    }
}

#[cfg(test)]
mod tests_unsigned {
    use crate::unsigned::UnsignedCounter;
    use crate::unsigned::Unsigned;
    #[test]
    fn it_works() {
        assert_eq!(<UnsignedCounter as Unsigned>::default_unsigned_counter(), (0));
        assert_eq!(<UnsignedCounter as Unsigned>::next_unsigned(10), (11));
    }
}