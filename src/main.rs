fn main() {
// This invalid indentation should cause the rustfmt check to fail:
println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn this_should_not_fail() {
        assert_eq!(1 + 1, 2);
    }
}
