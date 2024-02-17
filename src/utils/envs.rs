use std::env;
//use std::str::FromStr;

pub fn get_env(name: &'static str) -> Result<String> {
   env::var(name).map_err(|_| Error::MissingEnv(name))
}



// region:    ---  Error
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {	

	MissingEnv(&'static str),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// endregion:  ---  Error
