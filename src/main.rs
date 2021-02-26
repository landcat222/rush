use std::io;
use std::io::Write;

fn main() {
    let mut cmd = String::new();

    print!("foo@bar ~ $ ");
    io::stdout().flush().ok();
    io::stdin().read_line(&mut cmd).ok();
    println!("{}", cmd);
}
