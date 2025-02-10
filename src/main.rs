use std::io;

const EXPECT_MESSAGE:&str = "Failed to read user input.";

fn main() {

    loop {
        println!("Select a unit to convert...");

        println!("Type into the console one of the select unit conversions from below:\n");
        println!("Temperature Conversion: temperature/t");
        println!("Weight Conversion: weight/w");
        println!("Length Conversion: length/l");

        println!("\nOtherwise, enter q to quit this program.");

        let mut unit: String = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect(EXPECT_MESSAGE);

        //println!("{}", unit.to_ascii_lowercase().trim());

        let unit = unit.trim().to_ascii_lowercase();

        //println!("{}", unit);

        if unit == "q" {
            println!("\nYou have quit this program. Thank you and have a great day!\n");
            break;
        }

        let mut specific_unit: String = String::new();

        if unit == "temperature" || unit == "t" {

            println!("Type in fc to convert from fahrenheit to celsius. Type in cf to convert from celsius to fahrenheit.");

            io::stdin()
                .read_line(&mut specific_unit)
                .expect(EXPECT_MESSAGE);

            let specific_unit = specific_unit.trim().to_ascii_lowercase();

            let mut conversion: String = String::new();

            if specific_unit == "fc" {
                println!("Input a number to convert from fahrenheit to celsius: ");

                io::stdin()
                .read_line(&mut conversion)
                .expect(EXPECT_MESSAGE);

                let conversion: f64 = match conversion.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };

                let mut celsius_number:f64 = conversion - 32.0;

                celsius_number = celsius_number * 5.0/9.0;

                println!("Converted to {celsius_number}°C\n");

            } else if specific_unit == "cf" {
                println!("Input a number to convert from celsius to fahrenheit: ");

                io::stdin()
                .read_line(&mut conversion)
                .expect(EXPECT_MESSAGE);

                let conversion: f64 = match conversion.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };

                let mut fahrenheit_number:f64 = conversion * 1.8;

                fahrenheit_number = fahrenheit_number + 32.0;

                println!("Converted to {fahrenheit_number}°F\n");

            } else {
                println!("Incorrect input! Please try again and enter in fc or cf when you select temperature conversion.\n");
                continue;
            }
        } else if unit == "weight" || unit =="w" {
            println!("Type in pk to convert from pounds to kilograms. Type in kp to convert from kilograms to pounds.");

            io::stdin()
                .read_line(&mut specific_unit)
                .expect(EXPECT_MESSAGE);

            let specific_unit = specific_unit.trim().to_ascii_lowercase();

            let mut conversion: String = String::new();

            if specific_unit == "pk" {
                println!("Input a number to convert from pounds to kilograms: ");

                io::stdin()
                .read_line(&mut conversion)
                .expect(EXPECT_MESSAGE);

                let conversion: f64 = match conversion.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };

                let kilograms_number:f64 = conversion / 2.2046;

                println!("Converted to {kilograms_number} kg\n");

            } else if specific_unit == "kp" {
                println!("Input a number to convert from kilograms to pounds: ");

                io::stdin()
                .read_line(&mut conversion)
                .expect(EXPECT_MESSAGE);

                let conversion: f64 = match conversion.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };

                let pounds_number:f64 = conversion * 2.2046;

                println!("Converted to {pounds_number} lbs\n");

            } else {
                println!("Incorrect input! Please try again and enter in pk or kp when you select weight conversion.\n");
                continue;
            }

        } else if unit == "length" || unit == "l" {
            println!("Type in incm to convert from inches to centimeters. Type in cmin to convert from centimeters to inches.");

            io::stdin()
                .read_line(&mut specific_unit)
                .expect(EXPECT_MESSAGE);

            let specific_unit = specific_unit.trim().to_ascii_lowercase();

            let mut conversion: String = String::new();

            if specific_unit == "incm" {
                println!("Input a number to convert from inches to centimeters: ");

                io::stdin()
                .read_line(&mut conversion)
                .expect(EXPECT_MESSAGE);

                let conversion: f64 = match conversion.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };

                let centimeters_number:f64 = conversion * 2.54;

                println!("Converted to {centimeters_number} cm.\n");

            } else if specific_unit == "cmin" {
                println!("Input a number to convert from centimeters to inches: ");

                io::stdin()
                .read_line(&mut conversion)
                .expect(EXPECT_MESSAGE);

                let conversion: f64 = match conversion.trim().parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };

                let inches_number:f64 = conversion / 2.54;

                println!("Converted to {inches_number} in.\n");

            } else {
                println!("Incorrect input! Please try again and enter in incm or cmin when you select length conversion.\n");
                continue;
            }
        } else {
            println!("Unit was not recognized. Please try again.");
            continue;
        }
    }

}
