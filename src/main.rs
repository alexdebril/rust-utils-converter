use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("please provide a number");
        return;
    }
    let number = &args[1];
    let result = number.parse();
    match result {
        Ok(i) => println!("{} bytes equals to {}", i, convert(i)),
        Err(_e)    => println!("{} is not a number", number),
    }
}

fn convert(number: u64) -> std::string::String {
    let units: [&str; 4] = ["Kb", "Mb", "Gb", "Tb"];
    let mut exp = 1;
    for unit in units.iter() {
        let div = 1024u64.pow(exp) as f64;
        let out = (number as f64 / div) as f64;
        if out < 1000.0 {
            return format!("{:.2} {}", out, unit);
        }
        exp = exp+1;
    }
    return format!("huge");
}
