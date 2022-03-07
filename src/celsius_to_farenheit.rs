use std::io;

pub fn run() {
    loop {
        let mut celsius = String::new();

        println!("Enter celsius");

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line or is not a number");

        let celsius: f64 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        println!("Farenheit: {}", celsius_to_farenheit(celsius))
    }
}

fn celsius_to_farenheit(celsius: f64) -> f64 {
    const ONE_CELSIUS_FARENHEIT: f64 = 33.8;

    ((celsius * ONE_CELSIUS_FARENHEIT) * 100.0).round() / 100.0
}
