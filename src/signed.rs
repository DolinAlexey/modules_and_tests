pub type SignedCounter = isize;
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

#[cfg(test)]
mod tests_signed {
    use crate::signed::{Signed, SignedCounter};

    #[test]
    fn it_works() {
        assert_eq!(<SignedCounter as Signed>::default_signed_counter(), (0));
        assert_eq!(<SignedCounter as Signed>::next_signed (10), 11);
        assert_eq!(<SignedCounter as Signed>::prev_signed(10), 9);
    }
}