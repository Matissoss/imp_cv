use colored::*;
pub fn help() -> String{
    format!(
    "
    {} is a converter for imperial units

    {}
    -h | --help -> Shows all availiable Commands
    -n -arg1 arg2 -> Converts imperial to normal
    -i -arg1 arg2 -> Converts normal to imperial
    {}
    -n -i inches -> converts inch to meters
    -n -f feet   -> converts feet to meters
    -n -j jards  -> converts jards to meters
    -n -m miles  -> converts miles to kilometers
    {}
    -n -o ounces -> converts ounces to grams
    -n -p pounds -> converts pounds to kilograms
    -n -ut ton   -> converts UK tons to tons
    -n -st ton   -> converts US tons (short tons) to tons
    {}
    -i -mi meters -> converts meters to inches
    -i -mf meters -> converts meters to feet
    -i -mj meters -> converts meters to jards
    -i -mm meters -> converts meters to miles
    {}
    -i -go grams -> converts grams to ounces
    -i -kp kilos -> converts kilograms to pounds
    -i -tut tons -> converts tons to UK tons
    -i -tst tons -> converts tons to US tons (short tons)
    {}
    -n -fh farenheit -> converts farenheit to celcius
    -i -c celcius -> converts celcius to farenheit",

    "imp_cv".bold().yellow(), 
    "Commands".bold().cyan(), 
    "Normal Length Units".bold().blue(),
    "Normal Weight Units".bold().magenta(),
    "Imperial Length Units".bold().red(),
    "Imperial Weight Units".bold().yellow(),
    "Temperature Units".bold().green(),
    )
}
