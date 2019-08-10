pub type Map<K, V> = schemars::Map<K, V>;

pub mod openapi3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
