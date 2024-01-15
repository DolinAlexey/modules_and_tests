type SignedCounter = isize;
pub trait Signed {
    fn default_signed_counter() -> SignedCounter;
    fn next_signed(counter: SignedCounter) -> SignedCounter;
    fn prev_signed(counter: SignedCounter) -> SignedCounter;
}

impl Signed for SignedCounter {
    fn default_signed_counter() -> SignedCounter {
        0
    }
    fn next_signed(counter: SignedCounter) -> SignedCounter {
        counter + 1
    }
    fn prev_signed(counter: SignedCounter) -> SignedCounter {
        counter - 1
    }
}
