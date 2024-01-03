use std::{io, fmt::Display, process, panic::PanicInfo, fs, backtrace::Backtrace};
use chrono::Local;
use colored::Colorize;

pub trait ResultExt<T> {
	fn unwrap_or_exit(self) -> T;

	fn to_io_result(self) -> io::Result<T>;
}

impl<T, E: Display> ResultExt<T> for Result<T, E> {
	fn unwrap_or_exit(self) -> T {
		match self {
			Ok(t) => t,
			Err(e) => handler(e),
		}
	}

	fn to_io_result(self) -> io::Result<T> {
		match self {
			Ok(t) => Ok(t),
			Err(e) => Err(io::Error::other(e.to_string())),
		}
	}
}

pub fn handler(e: impl Display) -> ! {
	eprintln!("{} {e}", "error:".red().bold());
	process::exit(-1)
}

pub fn panic_hook(info: &PanicInfo) {
	let file_name = Local::now().format("panic-log(%F %T).txt").to_string();
	let contents = format!("{}\nstack backtrace:\n{}", info, Backtrace::force_capture());
	handler(format!(
		"something really bad happened (check {})",
		if fs::write(&file_name, &contents).is_err() {
			eprintln!("{contents}");
			"above"
		} else {
			file_name.as_str()
		}
	));
}
