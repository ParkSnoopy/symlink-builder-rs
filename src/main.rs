use serde::Deserialize;
use std::{ env, fs,
    process::{ Command, Stdio },
};

#[derive(Deserialize, Debug)]
struct Instruction {
    source: String,
}

fn main() {
    let user_args: Vec<String> = env::args().skip(1).collect();

    let inst_dir = {
        let mut bin_d = env::current_exe()
            .expect("Couldn't find the running binary.");
        bin_d.pop();
        bin_d.push("inst.toml");
        bin_d
    };

    let inst: Instruction = toml::from_str(
        fs::read_to_string(inst_dir)
            .expect("failed to find `inst.toml`")
            .as_str()
    ).expect("failed to parse `inst.toml`");

    let mut source = inst.source.split_whitespace();
    let root = source.next()
        .expect("no file specified in `inst.toml`");

    let mut args: Vec<String> = source.map(|x| x.to_string()).collect();
    args.extend(user_args);

    Command::new(root)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect(format!("failed to execute command `{}`", &inst.source).as_str());
}
