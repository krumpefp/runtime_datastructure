///
/// A module to parse lines strings and create a corresponding label object.
///
/// The strings must be of the form defined in the [Module description](index.html)
///

use regex::Regex;

use primitives::label::Label;

///
/// Validate if a string matches the required format
///
/// # Examples
/// ```
/// use rt_datastructure::input::parse;
/// use rt_datastructure::primitives::label;
///
/// let s = "53.143155300000004 8.9351249 3627273522 1 1.4922737369836614 3300.0 11.0 \
///          'Timmersloh'".to_string();
/// let v = parse::validate_label(&s);
/// assert!(v);
/// ```
///
/// ```
/// use rt_datastructure::input::parse;
/// use rt_datastructure::primitives::label;
///
/// let s = "8.9351249 3627273522 1 1.4922737369836614 3300.0 11.0 'Timmersloh'".to_string();
/// let v = parse::validate_label(&s);
/// assert!(!v);
/// ```
///
pub fn validate_label(s_input: &String) -> bool {
    lazy_static! {
        static ref RE : Regex = Regex::new("\
        ^-?\\d{1,3}\\.\\d*(e-?\\d+)? \
        -?\\d{1,3}\\.\\d*(e-?\\d+)? \
        \\d+ \\d+ \
        \\d+\\.\\d*(e-?\\d+)? \
        \\d+\\.\\d*(e-?\\d+)? \
        \\d+\\.\\d*(e-?\\d+)? \
        '.*'\
        ").unwrap();
    }

    RE.is_match(s_input)
}

///
/// Parse a string reference and create a corresponding label
///
/// # Panics
/// * Panics if the string does not match the required format.
///
/// # Examples
/// ```
/// use rt_datastructure::input::parse;
/// use rt_datastructure::primitives::label;
///
/// let s = "53.143155300000004 8.9351249 3627273522 1 1.4922737369836614 3300.0 11.0 \
///          'Timmersloh'".to_string();
/// let l = parse::parse_label(&s);
/// ```
///
/// ```should_panic
/// use rt_datastructure::input::parse;
/// use rt_datastructure::primitives::label;
///
/// let s = "8.9351249 3627273522 1 1.4922737369836614 3300.0 11.0 'Timmersloh'".to_string();
/// let l = parse::parse_label(&s);
/// ```
///
pub fn parse_label(s_input: &String) -> Label {
    lazy_static! {
        static ref RE2 : Regex = Regex::new("\
        ^(?P<y>-?\\d{1,3}\\.\\d*(e-?\\d+)?) \
        (?P<x>-?\\d{1,3}\\.\\d*(e-?\\d+)?) \
        (?P<osmId>\\d+) \
        (?P<prio>\\d+) \
        (?P<elimT>\\d+\\.\\d*(e-?\\d+)?) \
        (?P<rad>\\d+\\.\\d*(e-?\\d+)?) \
        (?P<lblFac>\\d+\\.\\d*(e-?\\d+)?) \
        '(?P<lbl>.*)'\
        ").unwrap();
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
