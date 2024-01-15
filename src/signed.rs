type SignedCounter = isize;
pub trait Signed {
    fn default_signed_counter() -> SignedCounter;
    fn next_signed(&self, counter: SignedCounter) -> SignedCounter;
    fn prev_signed(&self, counter: SignedCounter) -> SignedCounter;
}

impl<T: Signed> Signed for T {
    fn default_signed_counter() -> SignedCounter {
        0
    }
    fn next_signed(&self, counter: SignedCounter) -> SignedCounter {
        counter + 1
    }
    fn prev_signed(&self, counter: SignedCounter) -> SignedCounter {
        counter - 1
    }
}
