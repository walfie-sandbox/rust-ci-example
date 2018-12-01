fn main() {
    let name_opt = std::env::args().skip(1).next();
    let name = name_opt.as_ref().map(String::as_ref).unwrap_or("world");

// This invalid indentation should cause the rustfmt check to fail:
println!("Hello, {}!", name);
}
