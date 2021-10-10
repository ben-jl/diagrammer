mod grammar;

pub use grammar::data::{Dataset, DatasetBase};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
