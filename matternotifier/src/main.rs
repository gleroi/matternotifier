extern crate mm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = mm::Client::new("https://192.168.122.76:8065");
    let token = c.login_with_gitlab("test", "testtest")?;
    println!("token is {}", token);
    Ok(())
}
