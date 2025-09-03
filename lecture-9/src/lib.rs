pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtract(left: u32, right: u32) -> u32 {
    left - right
}

#[cfg(test)]
mod tests {

    mod common;
    use super::subtract;

    #[test]
    fn test_subtract() {
        let answer = subtract(3, 1);
        assert_ne!(answer, 3);
        // panic!("Left hand side is not equal to right hand side");
    }
}
