pub fn run() {
    println!("\n====3.16 MATCH====");
    let country_code = 44;
    // TODO: Try to use another country codes
    // let country_code = 55;
    // let country_code = 2000;

    let country = match country_code {
        44 => "UK",
        46 => "Sweeden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    println!("The coutry with code {} is {}", country_code, country);
}