pub fn convert(value: String){
    let start_value : f64 = match value.trim().parse(){
        Ok (v) =>{
            v
        }
        Err (_) =>{
            println!("Error parsing - terminating program");
            return;
        }
    };
    let to_return : f64 = (start_value * 1.8) + 32.0;

    println!("{} celcius = *{}* farenheit", start_value, to_return);
}
