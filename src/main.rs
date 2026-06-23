use std::env;
use std::process::{Command, exit};

fn main() {
    // SSH calls us as the askpass helper — just print the password
    if env::var("_SSHPASS_ASKPASS").is_ok() {
        print!("{}", env::var("SSHPASS").unwrap_or_default());
        return;
    }

    let args: Vec<String> = env::args().collect();
    let mut password = String::new();
    let mut ssh_args: &[String] = &[];
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-p" => {
                i += 1;
                password = args.get(i).cloned().unwrap_or_default();
            }
            "-e" => {
                password = env::var("SSHPASS").expect("SSHPASS env var not set");
            }
            _ => {
                ssh_args = &args[i..];
                break;
            }
        }
        i += 1;
    }

    if ssh_args.is_empty() {
        eprintln!("Usage: sshpass -p <password> ssh [args...]");
        eprintln!("       sshpass -e ssh [args...]  (reads SSHPASS env var)");
        exit(1);
    }

    let self_exe = env::current_exe().expect("can't resolve own path");

    let status = Command::new(&ssh_args[0])
        .args(&ssh_args[1..])
        .env("SSHPASS", &password)
        .env("SSH_ASKPASS", self_exe)
        .env("SSH_ASKPASS_REQUIRE", "force")
        .env("_SSHPASS_ASKPASS", "1")
        .status()
        .expect("failed to spawn SSH");

    exit(status.code().unwrap_or(1));
}
