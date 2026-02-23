use serde::Deserialize;
use std::{
    env,
    fs,
    process::{Command, Stdio},
};

#[derive(Deserialize, Debug)]
struct Instruction {
    source: String,
}

fn main() {
    let user_args: Vec<String> = env::args().skip(1).collect();

    let inst_file = env::current_exe()
        .expect("Couldn't find the running binary.")
        .as_path()
        .with_extension("toml");

    let inst: Instruction = toml::from_str(
        fs::read_to_string(inst_file)
            .expect("Failed to find instruction file")
            .as_str(),
    )
    .expect("Failed to parse instruction file");

    let mut source = inst.source.split_whitespace();
    let root = source.next().expect("Bad instruction file format");

    let mut args: Vec<String> = source.map(|x| x.to_string()).collect();
    args.extend(user_args);

    Command::new(root)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect(format!("failed to execute command `{}`", &inst.source).as_str());
}
