use std::ffi::{CStr, CString};

use parser::{CpcParser, Rule};
use pest::Parser;

use crate::parser::get_action;

pub mod parser;
pub mod action;

pub fn parse_line(line: &str) -> Result<pest::iterators::Pairs<'_, Rule>, pest::error::Error<Rule>>{
	CpcParser::parse(Rule::command, line)
}

pub fn execute_line(line: &str) -> Result<(), pest::error::Error<Rule>>{
	let mut rules = parse_line(line)?;
	let action = get_action(rules.next().unwrap());
	action.execute();
	Ok(())
}



#[no_mangle]
/// Return NULL if success and a pointer to a string in case of error. You have to dealloc the string yourself.
/// Line must contain only utf8 valid chars
pub extern fn cpcemu_execute_line(line: *const i8) -> *const i8 {
	let line = unsafe { CStr::from_ptr(line) };
	let line = line.to_string_lossy().into_owned();
	dbg!(&line);
	match execute_line(&line) {
    Ok(_) => {
		return  std::ptr::null();
	},
    Err(e) => {
		let msg = e.to_string();
		let msg = CString::new(msg).unwrap();
		return msg.into_raw() as _;
	},
}
}

// no idea why, but rsutlike tests do not work (failure at link)
#[no_mangle]
pub extern fn launch_tests() {
	assert!(dbg!(CpcParser::parse(Rule::dec_number, "123")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::hex_number, "0x100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::hex_number, "&100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::hex_number, "#100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::hex_number, "$100")).is_ok());


	assert!(dbg!(CpcParser::parse(Rule::address, "123")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::address, "0x100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::address, "&100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::address, "#100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::address, "$100")).is_ok());

	assert!(dbg!(CpcParser::parse(Rule::label, "ici")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label, "ici_et_la")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label, "ici_et_la.ou_ici")).is_ok());


	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "123")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "0x100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "&100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "#100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "$100")).is_ok());


	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "123")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "0x100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "&100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "#100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "$100")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "ici")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "ici_et_la")).is_ok());
	assert!(dbg!(CpcParser::parse(Rule::label_or_address, "ici_et_la.ou_ici")).is_ok());
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
