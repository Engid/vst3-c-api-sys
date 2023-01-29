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

    fn to_rust_string(&self) -> String {
        format!("let {}: [u32; 4] = [{}, {}, {}, {}]\n", self.name, self.arg0, self.arg1, self.arg2, self.arg3)
    }
}

fn match_static_const_macros(text: &str) -> Vec<StaticConstParse>{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"static\s+const\s+(\w+)\s(\w+)\s=\s(\w+)\s*\((0x[a-fA-F0-9]+),\s*(0x[a-fA-F0-9]+),\s*(0x[a-fA-F0-9]+),\s*(0x[a-fA-F0-9]+)\);").unwrap(); 
    }

    let parse_vec = RE.captures_iter(text)
        .map(|cap| StaticConstParse::from_capture(cap))
        .collect();

    parse_vec
}

fn to_rust_string(parse_vec: Vec<StaticConstParse>) -> String {
    parse_vec.iter()
        .map(|x| x.to_rust_string())
        .collect()
        .join()
}

#[cfg(test)]
mod tests {
    use crate::{match_static_const_macros, StaticConstParse};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }


    #[test]
    fn it_parses_to_struct(){

        let input = "
typedef struct Steinberg_FUnknown
{
    struct Steinberg_FUnknownVtbl* lpVtbl;
} Steinberg_FUnknown;

static const Steinberg_TUID Steinberg_FUnknown_iid = SMTG_INLINE_UID (0x00000000, 0x00000000, 0xC0000000, 0x00000046);";

        let match_list = match_static_const_macros(input);
        let match_0 = &match_list[0];

        assert_eq!(match_0.data_type, "Steinberg_TUID".to_string());
        assert_eq!(match_0.name, "Steinberg_FUnknown_iid".to_string());
        assert_eq!(match_0.macro_fn_name, "SMTG_INLINE_UID".to_string());
        assert_eq!(match_0.arg0, "0x00000000".to_string());
        assert_eq!(match_0.arg1, "0x00000000".to_string());
        assert_eq!(match_0.arg2, "0xC0000000".to_string());
        assert_eq!(match_0.arg3, "0x00000046".to_string());
    }

    
    #[test]
    fn it_converts_to_string(){

        let iid = StaticConstParse {
            data_type: "Steinberg_TUID".to_string(),
            name: "Steinberg_FUnknown_iid".to_string(),
            macro_fn_name: "SMTG_INLINE_UID".to_string(),
            arg0: "0x00000000".to_string(),
            arg1: "0x00000000".to_string(),
            arg2: "0xC0000000".to_string(),
            arg3: "0x00000046".to_string()
        };

        let test_string = iid.to_rust_string();

        assert_eq!(test_string, "let Steinberg_FUnknown_iid: [u32; 4] = [0x00000000, 0x00000000, 0xC0000000, 0x00000046]\n")
    }


}