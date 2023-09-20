
use std::io::{Read, Write, Error};

use super::vars;
use super::shell;
use super::args;

fn fn_set(shell: &mut shell::Shell, args: &args::Args) -> Result<String, Error> {
	Ok("".to_string())
}

fn fn_prompt(shell: &mut shell::Shell, args: &args::Args) -> Result<String, Error> {
	// read byte from shell.termin
	let mut inp: String = "".to_string();
	let mut cur: usize = 0; 
	let mut buf: [u8; 1] = [0; 1];
	loop {
		shell.termin.read(&mut buf)?;
		println!("{:?}", buf);
		match buf[0] {
			b'\r' | b'\n' => break,
			b'\x03' | b'\x04' => return Err(Error::new(std::io::ErrorKind::Other, "Interrupted")),
			b'\x7f' => { // backspace
				if cur == 0 { continue }
				cur -= 1;
				inp.remove(cur);
			},
			_ => { // default
				inp.insert(cur, buf[0] as char);
				cur += 1;
			}
		}
		static PROMPT: &str = "PROMPT";
		let prompt: String = shell.eval(&PROMPT.to_string())?;
		shell.termout.write("\x1b[2K\x1b[1G".as_bytes())?;
		shell.termout.write(prompt.as_bytes())?;
		shell.termout.write(inp.as_bytes())?;
		shell.termout.write("\x1b[".as_bytes())?;
		shell.termout.write((cur + prompt.len() + 1).to_string().as_bytes())?;
		shell.termout.write("G".as_bytes())?;
		shell.termout.flush()?;
	}
	Ok(inp)
}

pub fn add(shell: &mut shell::Shell) {
	let mut builtins = vars::Vars::new();
	
	builtins.set(&"home".to_string(), vars::Var::Value(
		std::env::var("HOME").unwrap()
	));
	builtins.set(&"prompt".to_string(), vars::Var::Value(
		"$ ".to_string()
	));
	builtins.set(&"readline".to_string(), vars::Var::Func(
		fn_prompt
	));
	// builtins.set(&"echo".to_string(), vars::Var::Func(
	// 	fn_set
	// ));
	shell.vars.add(&builtins);
}