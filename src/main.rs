use rustyline::error::ReadlineError;
use rustyline::Editor;
mod gl_ast;
mod gl_env;
mod gl_eval;
mod gl_exception;
mod gl_lexer;
mod gl_parser;
mod gl_runtime;
mod gl_statement;
mod gl_std;
mod gl_token;
mod gl_token_position;
mod gl_tokens;

const GL_VERSION: &str = "0.1.0-alpha";

fn main() {
	let gl_args: Vec<String> = std::env::args().map(|arg| String::from(arg)).collect::<Vec<String>>();
	let mut runtime = gl_runtime::RunTime::new(0);

	if gl_args.len() == 1 {
		show_version();
		println!("exit using Ctrl+D");
		let mut sheel = Editor::<()>::new();
		if let Err(_) = sheel.load_history(".glanguage.history") {}

		loop {
			let codetext = sheel.readline(">>> ");
			match codetext {
				Ok(mut codetext) => {
					sheel.add_history_entry(&codetext);
					codetext.push('\n');
					runtime.run_codetext("<stdin>".to_string(), codetext);
				}
				Err(ReadlineError::Interrupted) => {
					println!("KeyboardInterrupt");
				}
				Err(ReadlineError::Eof) => break,
				Err(_err) => break,
			}
		}
		sheel.save_history(".glanguage.history").unwrap();
	} else {
		let gl_flags = if gl_args.len() == 2 { &gl_args[2..gl_args.len()] } else { &gl_args[3..gl_args.len()] };

		if gl_args[1] == "--version" {
			show_version();
		} else if gl_args[1] == "--help" {
			show_help();
		} else if gl_args[1] == "run" || gl_args[1] == "cmd" {
			if gl_flags.len() > 0 {
				println!("Unknown flag '{}'", gl_flags[0]);
				return;
			}

			if gl_args[1] == "run" {
				if gl_args.len() == 2 {
					println!("GL: No input file");
					println!("Usage: gl run main.gl [arguments]");
				} else {
					if gl_args[2] == "--help" {
						println!("Usage: gl run main.gl [arguments]");
						return;
					}

					let path_file_dot_gl = std::path::Path::new(&gl_args[2]);
					if path_file_dot_gl.exists() && path_file_dot_gl.is_file() {
						if gl_args[2].ends_with(".gl") {
							let codetext: String = std::fs::read_to_string(&gl_args[2]).expect("");
							runtime.run_codetext(String::from(&gl_args[2]), codetext);
						} else {
							println!("GL: Invalid file extension, expected file with extension '.gl'");
						}
					} else {
						println!("GL: Can't open file '{}': No such file", gl_args[2]);
					}
				}
			} else if gl_args[1] == "cmd" {
				if gl_args.len() == 2 {
					println!("GL: Argument expected for the 'cmd' option");
				} else {
					let codetext: String = String::from(&gl_args[2]);
					runtime.run_codetext("<cmd>".to_string(), codetext);
				}
			}
		} else {
			println!("Unknown option '{}'", gl_args[1]);
			println!("Usage: gl run main.gl [arguments]");
			println!("Try `gl --help` for more information");
		}
	}
}

fn show_version() {
	println!("GLanguage v{}", GL_VERSION);
}

fn show_help() {
	show_version();
	println!("");
	println!("Usage: gl [options] [arguments]");
	println!("       gl run main.gl [arguments]");
	println!("");
	println!("Options:");
	println!("  --help        print this help message and exit");
	println!("  --version     print the glanguage version number and exit");
	println!("  cmd           run program passed as string");
	println!("  run           run program from script file");
	println!("");
}
