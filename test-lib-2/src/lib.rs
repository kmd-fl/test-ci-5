use test_lib1::add1;

pub fn add_suprize(left: usize, right: usize) -> usize {
    left + right + add1(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_suprize(2, 2);
        assert_eq!(result, 8);
    }
}
