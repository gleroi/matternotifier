extern crate mm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = mm::Client::new();
    c.login_with_gitlab("test", "testtest")
}
