// calc factorial using recursion without memoization.
pub fn fact_recur(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => n * fact_recur(n - 1),
    }
}

// factorial using iteration
pub fn fact_iter(n: usize) -> usize {
    let mut res: usize = 1;
    match n {
        0 | 1 => res,
        _ => {
            for i in 2..=n {
                res = res * i;
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fact_iter_test() {
        let expected_results: Vec<usize> = vec![1, 1, 2, 6, 24, 120, 720];
        for (n, exp) in expected_results.iter().enumerate() {
            assert_eq!(*exp, fact_iter(n));
        }
    }

    #[test]
    fn fact_recur_test() {
        let expected_results: Vec<usize> = vec![1, 1, 2, 6, 24, 120, 720];
        for (n, exp) in expected_results.iter().enumerate() {
            assert_eq!(*exp, fact_recur(n));
        }
    }
}
