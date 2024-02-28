use crate::gui_mod::internal_prelude::*;
use std::{fmt::Formatter, num::{ParseFloatError, ParseIntError}, str::ParseBoolError, error::Error};



#[derive(Debug)]
pub enum GuiError {
	
	InvalidFieldName {field_name: String, line: usize, path: PathBuf},
	CannotCastToFloat {value: String, line: usize, path: PathBuf, cause: ParseFloatError},
	CannotCastToBool {value: String, line: usize, path: PathBuf, cause: ParseBoolError},
	CannotCastToXAlignment {value: String, line: usize, path: PathBuf},
	CannotCastToYAlignment {value: String, line: usize, path: PathBuf},
	CannotCastToColor {value: String, line: usize, path: PathBuf, cause: CannotCastToColorCause},
	NoStartQuote {line: usize, path: PathBuf},
	NoEndQuote {line: usize, path: PathBuf},
	MissingColon {line: usize, path: PathBuf},
	
	InvalidFileName {path: PathBuf},
	
	ApplyFieldError {cause: Box<dyn Error>},
	IoError {cause: IoError},
	
}

impl Display for GuiError {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		match self {
			
			Self::InvalidFieldName {field_name, line, path} => write!(fmt, "\"Invalid Field Name\": '{field_name}' in {path:?} line {line}"),
			Self::CannotCastToFloat {value, line, path, cause: error} => write!(fmt, "\"Cannot Cast To Float\": '{value}' in {path:?} line {line} (cause: {error})"),
			Self::CannotCastToBool {value, line, path, cause: error} => write!(fmt, "\"Cannot Cast To Bool\": '{value}' in {path:?} line {line} (cause: {error})"),
			Self::CannotCastToColor {value, line, path, cause} => write!(fmt, "\"Cannot Cast To Color\": '{value}' in {path:?} line {line} (cause: {cause})"),
			Self::CannotCastToXAlignment {value, line, path} => write!(fmt, "\"Cannot Cast To X Alignment\": '{value}' in {path:?} line {line}"),
			Self::CannotCastToYAlignment {value, line, path} => write!(fmt, "\"Cannot Cast To Y Alignment\": '{value}' in {path:?} line {line}"),
			Self::NoStartQuote {line, path} => write!(fmt, "\"No Start Quote\": in {path:?} line {line}"),
			Self::NoEndQuote {line, path} => write!(fmt, "\"No End Quote\": in {path:?} line {line}"),
			Self::MissingColon {line, path} => write!(fmt, "\"Missing Colon\": in {path:?} line {line}"),
			
			Self::InvalidFileName {path} => write!(fmt, "\"Invalid File Name\": in {path:?}"),
			
			Self::ApplyFieldError { cause } => write!(fmt, "\"Apply Field Error\": '{cause:?}'"),
			Self::IoError {cause} => write!(fmt, "\"IO Error\": '{cause:?}'"),
			
		}
	}
}

impl std::error::Error for GuiError {}

impl From<std::io::Error> for GuiError {
	fn from(err: std::io::Error) -> Self {
		Self::IoError {cause: err}
	}
}



#[derive(Debug)]
pub enum CannotCastToColorCause {
	ParseInt {cause: ParseIntError},
	InvalidLength {len: usize},
}

impl Display for CannotCastToColorCause {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		match self {
			Self::ParseInt {cause} => write!(fmt, "ParseIntError: {cause}"),
			Self::InvalidLength {len} => write!(fmt, "Invalid Length: {len}"),
		}
	}
}
