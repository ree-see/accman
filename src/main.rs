use std::io;
mod password;

use crate::password::password::Password;

fn main() -> io::Result<()> {
    print!("New Password >>> ");

    let mut input = String::new();

    let _  = io::stdin()
        .read_line(&mut input);
    let passwd = Password::new(input.trim().to_string());
    println!("New password  was:");
    println!("{:?}", passwd);
    
    Ok(())
}
