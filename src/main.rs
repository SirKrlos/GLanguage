use rustyline::error::ReadlineError;
use rustyline::Editor;
mod gl_env;
mod gl_exception;
mod gl_lexer;
mod gl_runtime;
mod gl_token;
mod gl_token_position;
mod gl_tokens;

const GL_VERSION: &str = "0.1.0-alpha";

fn main() {}

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
