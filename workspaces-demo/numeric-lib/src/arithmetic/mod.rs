/// This is a function level documentation
/// Below is an add function
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        //write test here
        assert_eq!(add(2, 2), 4);
    }
}
