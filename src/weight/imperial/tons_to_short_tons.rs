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
    let to_return : f64 = start_value / 0.907;

    println!("{} tons = *{}* short tons (US Tons)", start_value, to_return);
}

