fn persistence(mut num: u64) -> u64 {
    let mut count = 0u64;
    while num > 9 {
        num = num.to_string().chars()
            .map(|x| x.to_digit(10).unwrap_or(1) as u64)
            .fold(1, |acc, next| acc * next);
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(persistence(39), 3);
        assert_eq!(persistence(4), 0);
        assert_eq!(persistence(999), 4);
        assert_eq!(persistence(25), 2);
    }
}