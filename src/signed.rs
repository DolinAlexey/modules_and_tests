type SignedCounter = isize;
pub trait Signed {
    fn default_signed_counter() -> SignedCounter;
    fn next_signed(&self) -> SignedCounter;
    fn prev_signed(&self, counter: SignedCounter) -> SignedCounter;
}

impl Signed for i32 {
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
