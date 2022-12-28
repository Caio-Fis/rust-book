fn main() {
    println!(r#"Type "quit" to leave!"#);
    loop {
        println!("Input a number: ");

        let mut num = String::new();
        std::io::stdin().read_line(&mut num).expect("Failed to read line");

        if num.trim() == "quit" {
            break;
        }

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{num} -> {}", fibonacci(num));
    }
}    

fn fibonacci(num: u32) -> u32 {
    match num {
        0 | 1 => 1,
        _ => fibonacci(num-1)+fibonacci(num-2),
    }
}