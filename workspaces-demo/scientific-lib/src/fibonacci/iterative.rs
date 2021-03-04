
/// calc fibonnaci using only iteration
pub fn fib_iter(n: usize) -> usize {
    let mut a = 0;
    let mut b = 1;
    let mut res = 0;

    match n {
        0 => a,
        1 => b,
        n => {
            for _ in 2..=n {
                res = a + b;
                a = b;
                b = res;
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fib_iter_test() {
        let exp_results = [0, 1, 1, 2, 3, 5];
        for (n, exp) in exp_results.iter().enumerate() {
            println!("expected {} for {}", exp, n);
            assert_eq!(*exp, fib_iter(n));
        }

        let num = 25; // seq until or closest to
        let expected = 75025;
        assert_eq!(expected, fib_iter(num));

        let num = 83;
        println!("calculating for {}", num);
        let expected = 99194853094755497;
        assert_eq!(expected, fib_iter(num));
    }
}
