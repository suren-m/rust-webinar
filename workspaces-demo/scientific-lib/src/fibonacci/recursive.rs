use std::collections::HashMap;

// similar to https://docs.scala-lang.org/tour/pattern-matching.html
pub fn fib_recur(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recur(n - 2) + fib_recur(n - 1),
    }
}

pub fn fib_recur_memo(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    match n {
        0 => 0,
        1 => 1,
        num => {
            if let Some(cached_res) = cache.get(&num) {
                *cached_res
            } else {
                let res = fib_recur_memo(n - 2, cache) + fib_recur_memo(n - 1, cache);
                cache.insert(num, res);
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_recur_test() {
        let exp_results = [0, 1, 1, 2, 3, 5];
        for (n, exp) in exp_results.iter().enumerate() {
            println!("expected {} for {}", exp, n);
            assert_eq!(*exp, fib_recur(n));
        }

        let num = 25; // seq until or closest to
        let expected = 75025;
        assert_eq!(expected, fib_recur(num));

        // let num = 83;
        // println!("calculating for {}", num);
        // let expected = 99194853094755497;
        // assert_eq!(expected, fib_recur(num));
    }

    #[test]
    fn fib_recur_memo_test() {
        let exp_results = [0, 1, 1, 2, 3, 5];
        let mut cache: HashMap<usize, usize> = HashMap::new();
        for (n, exp) in exp_results.iter().enumerate() {
            assert_eq!(*exp, fib_recur_memo(n, &mut cache));
        }

        let num = 25; // seq until or closest to
        let expected = 75025;
        let mut cache: HashMap<usize, usize> = HashMap::new();
        assert_eq!(expected, fib_recur_memo(num, &mut cache));

        let num = 83;
        println!("calculating for {}", num);
        let expected = 99194853094755497;
        let mut cache: HashMap<usize, usize> = HashMap::new();
        assert_eq!(expected, fib_recur_memo(num, &mut cache));
    }
}
