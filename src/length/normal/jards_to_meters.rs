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
    let to_return : f64 = start_value * 0.9144;

    println!("{} jards = *{}* meters", start_value, to_return);
}
