pub fn plus(a: u32, b: u32) -> u32 {
    a * b
}

pub mod a;

#[cfg(test)]
mod tests {
    use super::plus;
    #[test]
    fn it_works() {
        assert_eq!(plus(1, 3), 3);
    }
}
