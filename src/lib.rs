#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use regex::{Regex, Captures };

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct StaticConstParse {
    data_type: String,
    name: String,
    macro_fn_name: String,
    arg0: String,
    arg1: String,
    arg2: String,
    arg3: String,
}

impl StaticConstParse {
    fn from_capture(cap: Captures) -> Self {
        StaticConstParse {
            data_type: cap[1].to_string(),
            name: cap[2].to_string(),
            macro_fn_name: cap[3].to_string(),
            arg0: cap[4].to_string(),
            arg1: cap[5].to_string(),
            arg2: cap[6].to_string(),
            arg3: cap[7].to_string(),
        }
    }
}

fn match_static_const_macros(text: &str) -> Vec<StaticConstParse>{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"static\s+const\s+(\w+)\s(\w+)\s=\s(\w+)\s*\((0x[a-fA-F0-9]+),\s*(0x[a-fA-F0-9]+),\s*(0x[a-fA-F0-9]+),\s*(0x[a-fA-F0-9]+)\);").unwrap(); 
    }

    let parseVec = RE.captures_iter(text)
        .map(|cap| StaticConstParse::from_capture(cap))
        .collect();

    parseVec
}

#[cfg(test)]
mod tests {
    use crate::{match_static_const_macros};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }


    #[test]
    fn it_captures_pattern(){

        let input = "
typedef struct Steinberg_FUnknown
{
    struct Steinberg_FUnknownVtbl* lpVtbl;
} Steinberg_FUnknown;

static const Steinberg_TUID Steinberg_FUnknown_iid = SMTG_INLINE_UID (0x00000000, 0x00000000, 0xC0000000, 0x00000046);";

        assert_eq!(match_static_const_macros(input)[0].data_type, "Steinberg_TUID".to_string());
        assert_eq!(match_static_const_macros(input)[0].name, "Steinberg_FUnknown_iid".to_string());
        assert_eq!(match_static_const_macros(input)[0].macro_fn_name, "SMTG_INLINE_UID".to_string());
        assert_eq!(match_static_const_macros(input)[0].arg0, "0x00000000".to_string());
        assert_eq!(match_static_const_macros(input)[0].arg1, "0x00000000".to_string());
        assert_eq!(match_static_const_macros(input)[0].arg2, "0xC0000000".to_string());
        assert_eq!(match_static_const_macros(input)[0].arg3, "0x00000046".to_string());
    }

}