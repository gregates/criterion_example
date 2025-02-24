pub fn add(values: &[usize]) -> usize {
    return values[0] + values[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(&[2, 2]);
        assert_eq!(result, 4);
    }
}
