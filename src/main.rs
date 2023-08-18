use colored::*;
use std::string::ToString;
use sysinfo::{System, SystemExt, RefreshKind, ProcessorExt};

const FERRIS_ART: &[&str] = &[
    "                                              ",
    "              ▄   ▓▄ ▄▓▓  ▓▓                  ",
    "            ▄  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ▄          ",
    "           ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓          ",
    "        ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▌      ",
    "      ▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▄▄▄    ",
    "      ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▌    ",
    "   ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▀▓▄▒▓▓▓▓▓▀▓▄▓▓▓▓▓▓▓▓▓▓▓▓▓  ",
    "    ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▌ ▐██▒▓▓▒▌ ██▌▓▓▓▓▓▓▓▓▓▓▓   ",
    "  ▄▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█████▒▓▓▒████▌▓▓▓▓▓▓▓▓▓▓▓▓▓▄",
    " ▓▓▓▌▀▓▓▓▓▓▓▓▓▓▓▒▄▄▌▒▓▓▓▓▓▒▒▄▒▒▓▓▓▓▒▒▓▓▀▀▓▀▓▓▓",
    "  ▀▓▓▄ ▀▄ ▀▓▓▀▓▀▒▓▓▓▒▀▓▒▓▓▀▒▓▓▓▒▀▓▀▓▒▓▀  ▀ ▐▓▀",
    "    ▓▄  ▄  ▀▓▓▓▓▓▀▀▀         ▓▓▓▓▓▀    ▀ ▄▓   ",
    "      ▀       ▀▓▓▓▓▄▄     ▄▄▓▓▓▀         ▀    ",
    "                 ▀▀▀▀     ▀▀▀                 ",
    "                                              ",
];

fn exc(exc: &str) -> Result<std::process::Output, std::io::Error> {
    let mut exc = exc.split_whitespace();
    let mut cmd = std::process::Command::new(exc.next().unwrap());
    cmd.args(exc).output()
}

fn get_ver(cmd: &str) -> String {
    exc(cmd)
        .ok()
        .and_then(|ver| String::from_utf8(ver.stdout).ok())
        .and_then(|line| line.split_whitespace().nth(1).map(ToString::to_string))
        .unwrap_or_else(|| "not present".to_string())
}

fn get_cargo_crates() -> usize {
    exc("cargo install --list")
        .ok()
        .and_then(|installs| String::from_utf8(installs.stdout).ok())
        .map_or(0, |ilist| {
            ilist
                .lines()
                .filter(|line| !line.starts_with("    "))
                .count()
        })
}

fn render(art: bool, info: &[String]) {
    if art {
        for (ferris_line, info_line) in FERRIS_ART.iter().zip(info) {
            println!("{}   {}", ferris_line.red(), info_line);
        }
    } else {
        for line in info {
            println!("{}", line);
        }
    }
}

fn main() {
    let mut art = true;
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "-s" {
        art = false;
    }

    let mut sys = System::new_with_specifics(RefreshKind::new().with_cpu());
    sys.refresh_all();
    let kernel = sys.kernel_version().unwrap_or_else(|| "Unknown".into());
    let total_ram = sys.total_memory() / 1000;
    let used_ram = sys.used_memory() / 1000;
    let cpu = sys.processors()[0].brand();

    let rustc_ver = get_ver("rustc  -V");
    let cargo_ver = get_ver("cargo  -V");
    let rustup_ver = get_ver("rustup -V");
    let cargo_crates = get_cargo_crates();

    let userinfo = format!(
        "{}{}{}",
        whoami::username().bright_red().bold(),
        "@".bold(),
        whoami::hostname().bright_red().bold()
    );
    let splitline = "═".repeat(whoami::username().len() + whoami::hostname().len() + 1);
    let rustc_ver = format!("{}{}", "rustc  ver: ".bright_red(), rustc_ver);
    let rustup_ver = format!("{}{}", "rustup ver: ".bright_red(), rustup_ver);
    let cargo_ver = format!("{}{}", "cargo  ver: ".bright_red(), cargo_ver);
    let cargo_crates = format!("{}{}", "cargo crates: ".bright_red(), cargo_crates);
    let os = format!("{}{}", "os: ".bright_red(), whoami::distro());
    let kernel = format!("{}{}", "kernel: ".bright_red(), kernel);
    let cpu = format!("{}{}", "cpu: ".bright_red(), cpu);
    let ram = format!(
        "{}{} » {}{}",
        "ram: ".bright_red(),
        used_ram,
        total_ram,
        " MB"
    );

    let bright_colors = format!(
        "{}{}{}{}{}{}{}{}",
        "███".bright_red(),
        "███".bright_yellow(),
        "███".bright_green(),
        "███".bright_cyan(),
        "███".bright_blue(),
        "███".bright_magenta(),
        "███".bright_black(),
        "███".bright_white()
    );
    let dark_colors = format!(
        "{}{}{}{}{}{}{}{}",
        "███".red(),
        "███".yellow(),
        "███".green(),
        "███".cyan(),
        "███".blue(),
        "███".magenta(),
        "███".black(),
        "███".white()
    );

    render(
        art,
        &[
            "".to_string(),
            "".to_string(),
            userinfo,
            splitline,
            rustc_ver,
            rustup_ver,
            cargo_ver,
            cargo_crates,
            os,
            kernel,
            cpu,
            ram,
            "".to_string(),
            bright_colors,
            dark_colors,
            "".to_string(),
        ],
    );
}
