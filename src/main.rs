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

use length::normal::{
    inches_to_meters,
    jards_to_meters,
    foot_to_meters,
    miles_to_kilometers
};
use length::imperial::{
    meters_to_inches,
    meters_to_jards,
    meters_to_foot,
    meters_to_miles
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
        }
    }
}
