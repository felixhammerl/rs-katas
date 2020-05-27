fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut v0 = 0;
    let mut v1 = 1;
    
    while v0 * v1 < prod {
        let temp = v0 + v1;
        v0 = v1;
        v1 = temp
    }

    return (v0, v1, v0*v1==prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
        assert_eq!(product_fib(prod), exp)
    }

    #[test]
    fn basics_product_fib() {
        dotest(4895, (55, 89, true));
        dotest(5895, (89, 144, false));
    }
}
