use regex::Regex;

use primitives::label::Label;

pub fn validate_label(s_input : &String) -> bool {
//     lazy_static! {
        let re = Regex::new(r"^\d{1,3}\.\d* \d{1,3}\.\d* \d+ \d+ \d+\.\d* \d+\.\d* \d+\.\d* '.*'").unwrap();
//     }
    
    re.is_match(s_input)
}

pub fn parse_label(s_input : &String) -> Label {
//     lazy_static! {
        let re = Regex::new(r"\s+").unwrap();
//     }
    println!("Trimmed string: {}", s_input);
    let fields: Vec<&str> = re.splitn(s_input, 8).collect();
    
    println!("Splitted fields {:?}", fields);

    Label::new(90., 90., 0.9, 1234567, 16, "Test".to_string())
}
