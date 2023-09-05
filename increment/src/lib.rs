pub fn increment(n: usize) -> usize {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = increment(2);
        assert_eq!(result, 3);
    }
}
