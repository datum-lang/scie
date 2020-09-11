extern {
    #[link(name="foo++", kind="static")]
    fn testcall_cpp(v: f32);
}

fn main() {
    println!("Hello, world from Rust!");
    unsafe {
        testcall_cpp(3.14159);
    };
}
