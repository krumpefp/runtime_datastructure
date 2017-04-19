use regex::Regex;

use primitives::label::Label;

pub fn validate_label(s_input: &String) -> bool {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^\d{1,3}\.\d* \d{1,3}\.\d* \d+ \d+ \d+\.\d* \d+\.\d* \d+\.\d* '.*'").unwrap();
    }

    RE.is_match(s_input)
}

pub fn parse_label(s_input: &String) -> Label {
    lazy_static! {
        static ref RE2 : Regex = Regex::new(r"^(?P<y>\d{1,3}\.\d*) (?P<x>\d{1,3}\.\d*) (?P<osmId>\d+) (?P<prio>\d+) (?P<elimT>\d+\.\d*) (?P<rad>\d+\.\d*) (?P<lblFac>\d+\.\d*) '(?P<lbl>.*)'").unwrap();
    }
    //     println!("Trimmed string: {}", s_input);
//     let fields = RE2.captures(s_input).unwrap();
    let fields = match RE2.captures(s_input) {
        Some(capture) => capture,
        None => panic!("Could not evaulate poi: {}", s_input),
    };

    //     println!("Splitted fields {:?}", fields);

    let x: f64 = fields["x"].parse().expect("Could not parse float");
    let y: f64 = fields["y"].parse().expect("Could not parse float");
    let elim_t: f64 = fields["elimT"].parse().expect("Could not parse float");
    let osm_id: i64 = fields["osmId"].parse().expect("Could not parse i64");
    let prio: i32 = fields["prio"].parse().expect("Could not parse i32");
    let label: String = fields["lbl"].to_string();

    Label::new(x, y, elim_t, osm_id, prio, label)
}
