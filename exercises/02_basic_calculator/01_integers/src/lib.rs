fn compute(a: u32, b: u32) -> u32 {
    a + b * 4u8 as u32
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
