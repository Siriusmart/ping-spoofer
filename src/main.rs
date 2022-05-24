use std::{env, process::Command};

const VERSION: &str = "1.0.0";
const HELP: &str = "Ping Spoofer Help

Overview: Ping Spoofer is a simple CLI tool that artificially increases your ping systemwide.

Commands:
 - ping-spoofer start [ms] [device]
 - ping-spoofer stop [device]
 - ping-spoofer uninstall
 - ping-spoofer --help
 - ping-spoofer --version

Reference:
 - [ms] is the amount of milliseconds to increase your ping by.
 - [device] is the device to increase your ping on, can be found by running 'tc qdisc ls', it looks something like this: eth0/lan0/wlan0/...";

fn main() {
    let mut args = env::args().collect::<Vec<String>>().into_iter().skip(1);
    let cmd_type;

    match args.next() {
        None => {
            println!("No arguments provided");
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

                "uninstall" => CommandType::Uninstall,

                "--help" => CommandType::Help,

                "--version" => CommandType::Version,

                _ => {
                    println!("Invalid argument");
                    return;
                }
            }
        }
    }

    match &cmd_type {
        CommandType::On(_, device) | CommandType::Off(device) => {
            let mut command_off = Command::new("sudo");
            command_off.args(["tc", "qdisc", "del", "dev", &device, "root"]);
            command_off.output().unwrap();

            if cmd_type.is_on() {
                let mut command_on = Command::new("sudo");
                command_on.args([
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
                command_on.output().unwrap();
            }
        }

        CommandType::Uninstall => {
            let mut command_uninstall = Command::new("sudo");
            command_uninstall.args(["rm", "/bin/ping-spoofer"]);
            command_uninstall.output().unwrap();
            println!("Uninstalled successfully");
        }

        CommandType::Help => {
            println!("{}", HELP);
            return;
        }

        CommandType::Version => {
            println!("{}", VERSION);
            return;
        }
    };
}

#[derive(Debug)]
enum CommandType {
    On(usize, String),
    Off(String),
    Uninstall,
    Help,
    Version,
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
