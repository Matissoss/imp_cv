mod logs;

mod length{
    pub mod normal{
        pub mod inches_to_meters;
        pub mod jards_to_meters;
        pub mod foot_to_meters;
        pub mod miles_to_kilometers;
    }
    pub mod imperial{
        pub mod meters_to_inches;
        pub mod meters_to_jards;
        pub mod meters_to_foot;
        pub mod meters_to_miles;
    }
}

mod temperature{
    pub mod farenheit;
    pub mod celcius;
}
mod weight{
    pub mod imperial{
        pub mod grams_to_ounces;
        pub mod kilograms_to_pounds;
        pub mod tons_to_uktons;
        pub mod tons_to_short_tons;
    }
    pub mod normal{
        pub mod ounces_to_grams;
        pub mod pounds_to_kilograms;
        pub mod uktons_to_tons;
        pub mod short_tons_to_tons;
    }
}

use length::{
    normal::{
        inches_to_meters,
        jards_to_meters,
        foot_to_meters,
        miles_to_kilometers
    },
    imperial::{
        meters_to_inches,
        meters_to_jards,
        meters_to_foot,
        meters_to_miles
    }
};

use temperature::{
    farenheit,
    celcius,
};

use weight::{
    imperial::{
        grams_to_ounces,
        kilograms_to_pounds,
        tons_to_uktons,
        tons_to_short_tons,
    },
    normal::{
        ounces_to_grams,
        pounds_to_kilograms,
        uktons_to_tons,
        short_tons_to_tons
    }
};

fn main() {
    let args : Vec<String> = std::env::args().collect();

    if &args.len() <= &2{
        if &args[1] == "-h" || &args[1] == "--help"{println!("{}", logs::help())}
    }
    else if &args.len() <= &4{
        if &args[1] == "-n"{
            if &args[2] == "-i" {
                inches_to_meters::convert(args[3].clone());
            }
            else if &args[2] == "-f"{
                foot_to_meters::convert(args[3].clone());
            }
            else if &args[2] == "-j"{
                jards_to_meters::convert(args[3].clone());
            }
            else if &args[2] == "-m"{
                miles_to_kilometers::convert(args[3].clone());
            }
            else if &args[2] == "-fh"{
                farenheit::convert(args[3].clone());
            }
            else if &args[2] == "-o"{
                ounces_to_grams::convert(args[3].clone());
            }
            else if &args[2] == "-p"{
                pounds_to_kilograms::convert(args[3].clone());
            }
            else if &args[2] == "-ut"{
                uktons_to_tons::convert(args[3].clone());
            }
            else if &args[2] == "-st"{
                short_tons_to_tons::convert(args[3].clone());
            }
        }
        else if &args[1] == "-i"{
            if &args[2] == "-mi"{
                meters_to_inches::convert(args[3].clone());
            }
            else if &args[2] == "-mf"{
                meters_to_foot::convert(args[3].clone());
            }
            else if &args[2] == "-mj"{
                meters_to_jards::convert(args[3].clone());
            }
            else if &args[2] == "-mm"{
                meters_to_miles::convert(args[3].clone());
            }
            else if &args[2] == "-c"{
                celcius::convert(args[3].clone());
            }
            else if &args[2] == "-go"{
                grams_to_ounces::convert(args[3].clone());
            }
            else if &args[2] == "-kp"{
                kilograms_to_pounds::convert(args[3].clone());
            }
            else if &args[2] == "-tut"{
                tons_to_uktons::convert(args[3].clone());
            }
            else if &args[2] == "-tst"{
                tons_to_short_tons::convert(args[3].clone());
            }
        }
    }
}
