#![cfg_attr(not(feature = "std"), no_std)]

mod commitment;
mod fft;
mod keypair;
mod poly;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
