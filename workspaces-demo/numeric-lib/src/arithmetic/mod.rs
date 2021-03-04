/// This is a function level documentation
/// function syntax similar to https://docs.swift.org/swift-book/GuidedTour/GuidedTour.html#ID463
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
