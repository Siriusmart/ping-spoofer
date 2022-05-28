use nix::unistd;
use std::{
    env,
    process::{self, Command},
};

const VERSION: &str = "Version 1.1.0";
const UID_TO_ROOT: &str = "Failed to set uid to root\nFollow the instructions in the url below to setup the program:\nhttps://github.com/Siriusmart/ping-spoofer";
const HELP: &str = "Ping Spoofer Help

Overview: Ping Spoofer is a simple CLI tool that artificially increases your ping systemwide.

Commands:
 - ping-spoofer on [ms] [device]
 - ping-spoofer off [device]
 - ping-spoofer increase [ms] [device]
 - ping-spoofer decrease [ms] [device]
 - ping-spoofer status [device]
 - ping-spoofer uninstall

Flags (note that the flags can be placed anywhere in the command):
 - help: Prints this help message
 - version: Prints the version of the tool
 - bypass-root: Bypasses the root check

Reference:
 - [ms] is the amount of milliseconds to increase your ping by.
 - [device] is the device to increase your ping on, can be found by running 'tc qdisc ls', it looks something like this: eth0/lan0/wlan0/...
 
 Examples:
 - ping-spoofer on 100 wlan0
 - ping-spoofer off wlan0
 - ping-spoofer on 100 wlan0 --bypass-root
 - ping-spoofer off wlan0 --bypass-root
";

fn main() {
    let mut args = env::args()
        .collect::<Vec<String>>()
        .into_iter()
        .skip(1)
        .filter(|arg| {
            if arg.starts_with("--") {
                match arg.as_str() {
                    "--help" => {
                        println!("{}", HELP);
                        process::exit(0);
                    }

                    "--version" => {
                        println!("{}", VERSION);
                        process::exit(0);
                    }

                    "--bypass-root" => {
                        match unistd::setuid(unistd::Uid::from_raw(0)) {
                            Ok(_) => {}
                            Err(_) => {
                                println!("{}", UID_TO_ROOT);
                                process::exit(1);
                            }
                        }
                        return false;
                    }

                    _ => {
                        println!("Unknown flag: {}", arg);
                        process::exit(1);
                    }
                }
            }

            true
        });

    let cmd_type;

    match args.next() {
        None => {
            println!("No arguments provided. Run 'ping-spoofer --help' for more information.");
            return;
        }
        Some(s) => {
            cmd_type = match s.as_str() {
                "on" => CommandType::On(
                    match args.next() {
                        None => {
                            println!("No arguments provided");
                            return;
                        }
                        Some(s) => match s.parse::<usize>() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("Invalid argument");
                                return;
                            }
                        },
                    },
                    match args.next() {
                        None => {
                            println!("No device provided");
                            return;
                        }
                        Some(s) => s,
                    },
                ),

                "off" => CommandType::Off(match args.next() {
                    None => {
                        println!("No device provided");
                        return;
                    }
                    Some(s) => s,
                }),

                "increase" | "decrease" => {
                    let change = match args.next() {
                        None => {
                            println!("No arguments provided");
                            return;
                        }
                        Some(s) => match s.parse::<i32>() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("Invalid argument");
                                return;
                            }
                        },
                    };

                    let device = match args.next() {
                        None => {
                            println!("No device provided");
                            return;
                        }
                        Some(s) => s,
                    };

                    CommandType::Change(if s == "increase" { change } else { -change }, device)
                }

                "status" => CommandType::Status(match args.next() {
                    None => {
                        println!("No device provided");
                        return;
                    }
                    Some(s) => s,
                }),

                "uninstall" => CommandType::Uninstall,

                _ => {
                    println!("Invalid argument");
                    return;
                }
            }
        }
    }

    match &cmd_type {
        CommandType::On(_, device) | CommandType::Off(device) => {
            let mut command = Command::new("sudo");
            command.args(["tc", "qdisc", "del", "dev", &device, "root"]);
            command.output().unwrap();

            if cmd_type.is_on() {
                let mut command = Command::new("sudo");
                command.args([
                    "tc",
                    "qdisc",
                    "add",
                    "dev",
                    &device,
                    "root",
                    "netem",
                    "delay",
                    &format!("{}ms", cmd_type.get_ms()),
                ]);
                command.output().unwrap();
            }
        }

        CommandType::Change(ms, device) => {
            let mut command = Command::new("sudo");
            command.args(["tc", "qdisc", "show", "dev", &device]);

            let output = command.output().unwrap();
            let mut output_string = String::from_utf8(output.stdout).unwrap();
            output_string = output_string.lines().next().unwrap().to_string();

            let output_args = output_string.split(" ");

            let mut command = Command::new("sudo");
            command.args(["tc", "qdisc", "del", "dev", &device, "root"]);
            command.output().unwrap();

            for arg in output_args {
                if arg.ends_with("ms") {
                    let original_ms = arg[..arg.len() - 2].parse::<i32>().unwrap();
                    let mut command = Command::new("sudo");

                    command.args([
                        "tc",
                        "qdisc",
                        "add",
                        "dev",
                        &device,
                        "root",
                        "netem",
                        "delay",
                        &format!("{}ms", original_ms + ms),
                    ]);
                    command.output().unwrap();
                }
            }

            let mut command = Command::new("sudo");

            command.args([
                "tc",
                "qdisc",
                "add",
                "dev",
                &device,
                "root",
                "netem",
                "delay",
                &format!("{}ms", ms),
            ]);
            command.output().unwrap();
        }

        CommandType::Uninstall => {
            let mut command_uninstall = Command::new("rm");
            command_uninstall.args(["-f", "/bin/ping-spoofer"]);
            command_uninstall.output().unwrap();

            match home::home_dir() {
                Some(path) => {
                    let mut command_uninstall = Command::new("rm");
                    command_uninstall
                        .args(["-f", path.join(".cargo/bin/ping-spoofer").to_str().unwrap()]);
                    command_uninstall.output().unwrap();
                }
                None => println!("Unable to get your home dir. Skipping ~/.cargo/bin/ping-spoofer"),
            }
            println!("Uninstalled successfully");
        }

        CommandType::Status(device) => {
            let mut command = Command::new("sudo");
            command.args(["tc", "qdisc", "show", "dev", &device]);

            let output = command.output().unwrap();
            let mut output_string = String::from_utf8(output.stdout).unwrap();
            output_string = output_string.lines().next().unwrap().to_string();

            let output_args = output_string.split(" ");

            for arg in output_args {
                if arg.ends_with("ms") {
                    println!("{}", arg);
                    return;
                }
            }

            println!("Ping spoofer is off");
        }
    };
}

enum CommandType {
    On(usize, String),
    Off(String),
    Change(i32, String),
    Status(String),
    Uninstall,
}

impl CommandType {
    fn is_on(&self) -> bool {
        match self {
            CommandType::On(_, _) => true,
            _ => false,
        }
    }

    fn get_ms(&self) -> usize {
        match self {
            CommandType::On(ms, _) => *ms,
            _ => 0,
        }
    }
}
