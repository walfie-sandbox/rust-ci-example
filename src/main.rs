fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn this_should_fail() {
        assert_eq!(1 + 1, 3);
    }
}
