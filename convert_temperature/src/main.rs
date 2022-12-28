fn main() {
    loop {
        println!("\nWhich temperature conversion do you want?\n");
        println!("Fahrenheit to Celsius: 1");
        println!("Celsius to Fahrenheit: 2");
        println!("\nOr type any number to quit\n");

        let mut choose = String::new();

        std::io::stdin()
            .read_line(&mut choose)
            .expect("Failed to read line!");
        
        let choose: u32 = match choose.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choose == 1 {
            println!("Please input the temperature in Fahrenheit: ");
            let mut fahrenheit = String::new();

            std::io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read line!");
        
            let fahrenheit: f32 = fahrenheit
                .trim()
                .parse()
                .expect("Please type a number!");
        
            let final_celsius: f32 = ((fahrenheit - 32.).abs() * 5.0) / 9.0;
            println!("Temperature in Celsius: {final_celsius}");

        } else if choose == 2 {
            println!("Please input the temperature in Celsius: ");
            let mut celsius = String::new();

            std::io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read line!");
        
            let celsius: f32 = celsius
                .trim()
                .parse()
                .expect("Please type a number!");
        
            let final_fahrenheit: f32 = ((celsius * 9.0) / 5.0) + 32.0;
            println!("Temperature in Fahrenheit: {final_fahrenheit}");

        } else {
            break;
        }
    }
}
