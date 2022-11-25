fn main() {
    let temp = celsius_fahrenheit_converter(200, TemperatureScales::FahrenheitToCelsius);
    println!("World Temperature: {}degC", temp);

    let recover_value = celsius_fahrenheit_converter(temp, TemperatureScales::CelsiusToFahrenheit);
    println!("World temperature: {}degF", recover_value);
}

#[derive(PartialEq)]
enum TemperatureScales {
    FahrenheitToCelsius,
    CelsiusToFahrenheit,
}

fn celsius_fahrenheit_converter(value: i64, scale: TemperatureScales) -> i64 {
    let resulting_temp;
    let result = if scale == TemperatureScales::CelsiusToFahrenheit {
        resulting_temp = 9 * value / 5 + 32;
        resulting_temp
    } else {
        resulting_temp = (value - 32) * 5 / 9;
        resulting_temp
    };

    result
}
