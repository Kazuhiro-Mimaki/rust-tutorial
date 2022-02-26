use structOpt::StructOpt;
use std::fs::read_to_string;

#[derive(StructOpt)]
#[structOpt(name = "rsgrep")]
struct GrepArgs {
    #[structOpt(name = "PATTERN")]
    pattern: String,
    #[structOpt(name = "FILE")]
    path: String,
}

fn grep(content: String, state: &GrepArgs) {
    for line in content.lines() {
        if line.contains(state.pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(state: GrepArgs) {
    match read_to_string(&state.path) {
        Ok(content) => grep(content, &state.pattern),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    run(GrepArgs::from_args());
}
