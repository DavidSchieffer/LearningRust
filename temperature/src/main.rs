fn main() {
    let f_temp: f64 = 5.0;
    let c_temp: f64 = 15.0;

    println!("{f_temp} fahrenheit is {} celsius! Which is {} fahrenheit!"
            ,to_celsius(f_temp)
            ,to_fahrenheit(to_celsius(f_temp)));
    
    println!("{c_temp} celsius is {} fahrenheit! Which is {} celsius!"
            ,to_fahrenheit(c_temp)
            ,to_celsius(to_fahrenheit(c_temp)));
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}