#![allow(dead_code)]
#![allow(unused_variables)]
/* use std::mem; */

const COUNTRY_CODES_RANGE: [i64; 4] = [1,999,50,66];
const START:i64 = COUNTRY_CODES_RANGE[0];
const END:i64 = COUNTRY_CODES_RANGE[1];

pub fn match_statement() {
    println!("Match statement----------------------------------");
    let spain_country_code = 34;
    let sweden_country_code = 46;
    let invalid_country = 1234;
    let unknown_country = 52;
    //for spain
    country_code_evaluation(spain_country_code);
    //for sweden
    country_code_evaluation(sweden_country_code);
    //for invalid
    country_code_evaluation(invalid_country);
    //for unknown
    country_code_evaluation(unknown_country);
}
fn country_code_evaluation(country_code:i64){
  let country = match country_code {
        34 => "Spain",
        46 => "Sweden",
        49 => "Germany",
        7 => "Russia",
        START..=END => "unknown",
        /* this cant work because rust needs to know the values of each match arm at compile time
        country_codes_range[0]..=country_codes_range[1] => "unknown", */
        _ => "invalid",
    };
    if country == "invalid" {
        println!(
            "An {} country code is {} because is out of the range [{},{}]",
            country, country_code,COUNTRY_CODES_RANGE[0],COUNTRY_CODES_RANGE[1]
        );
    } else {
        println!("The country with code {} is {}", country_code, country);
    }
}