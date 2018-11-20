fn main() {
    let name_opt = std::env::args().skip(1).next();
    let name = name_opt.as_ref().map(String::as_ref).unwrap_or("world");

    println!("Hello, {}!", name);
}
