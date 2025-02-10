use std::io;

fn main() {
    println!("Select a unit to convert...");

    println!("Type into the console one of the select unit conversions from below:");
    println!("Temperature Conversion: temperature/t");
    println!("Weight Conversion: weight/w");
    println!("Length Conversion: length/l");

    let mut unit: String = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read user input.");

    //println!("{}", unit.to_ascii_lowercase().trim());

    let unit = unit.trim().to_ascii_lowercase();

    //println!("{}", unit);

    let mut specific_unit: String = String::new();

    if unit == "temperature" || unit == "t" {
        //println!("Temperature recieved");

        println!("Type in fc to convert from fahrenheit to celsius. Type in cf to convert from celsius to fahrenheit.");

        io::stdin()
            .read_line(&mut specific_unit)
            .expect("Failed to read user input.");

        let mut conversion: String = String::new();

        if specific_unit == "fc" {
            println!("Input a number to convert from fahrenheit to celsius: ");

            io::stdin()
                .read_line(&mut conversion)
                .expect("Failed to read user input");

            let conversion: f64 = match conversion.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };

        } else {

        }
    } else if unit == "weight" || unit == "w" {
        //println!("Weight received");
    } else if unit == "length" || unit == "l" {
        //println!("Length received");
    } else {
        println!("Unknown Unit!");
    }
}
