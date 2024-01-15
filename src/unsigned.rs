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
