use colored::*;
use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

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
    let exc: Vec<&str> = exc.split_whitespace().collect();
    let mut cmd = std::process::Command::new(exc[0]);
    cmd.args(&exc[1..exc.len()]).output()
}

fn get_ver(cmd: &str) -> String {
    let get_ver = match exc(cmd) {
        Ok(ver) => ver.stdout,
        Err(_) => "not present".as_bytes().to_vec(),
    };
    let mut get_ver = std::str::from_utf8(&get_ver).unwrap().lines();
    match get_ver.next() {
        Some(v) => v.to_string(),
        None => "not present.".to_string(),
    }
}

fn get_cargo_crates() -> usize {
    let cargo_installs = match exc("cargo install --list") {
        Ok(installs) => installs.stdout,
        Err(_) => "not present".as_bytes().to_vec(),
    };

    let cargo_installs = std::str::from_utf8(&cargo_installs).unwrap().lines();
    cargo_installs.filter(|line| !line.starts_with("    ")).count()
}

fn render(info: &[String]) {
    let mut info = info.iter();
    for line in FERRIS_ART {
        println!(
            "{}   {}",
            line.red(),
            match info.next() {
                Some(info) => info.as_str(),
                None => "",
            }
        );
    }
}

fn main() {
    let sys = System::new_with_specifics(RefreshKind::new().with_cpu().with_memory());
    let cpu = sys.get_processors();
    let cpu = cpu[0].get_brand();

    let used_ram = sys.get_used_memory() / 1024;
    let total_ram = sys.get_total_memory() / 1024;

    let rustc_cmd = get_ver("rustc -V");
    let cargo_cmd = get_ver("cargo -V");
    let rustup_cmd = get_ver("rustup -V");
    let rust_ver: Vec<&str> = rustc_cmd.split_whitespace().collect();
    let cargo_ver: Vec<&str> = cargo_cmd.split_whitespace().collect();
    let rustup_ver: Vec<&str> = rustup_cmd.split_whitespace().collect();
    let cargo_packages = get_cargo_crates();

    let info = &[
        "".into(),
        "".into(),
        format!(
            "{}{}{}",
            whoami::username().bright_red().bold(),
            "@".bold(),
            whoami::hostname().bright_red().bold()
        ),
        "════════════════".into(),
        format!("{}{}", "rust ver: ".bright_red(), rust_ver[1]),
        format!("{}{}", "rustup ver: ".bright_red(), rustup_ver[1]),
        format!("{}{}", "cargo ver: ".bright_red(), cargo_ver[1]),
        format!("{}{}", "cargo crates: ".bright_red(), cargo_packages),
        format!("{}{}", "os: ".bright_red(), whoami::distro()),
        format!(
            "{}{}",
            "kernel: ".bright_red(),
            sys.get_kernel_version().unwrap_or_else(|| "Unknown".into())
        ),
        format!("{}{}", "cpu: ".bright_red(), cpu),
        format!("{}{} » {} MB", "ram: ".bright_red(), used_ram, total_ram),
        "".into(),
        format!(
            "{}{}{}{}{}{}{}{}",
            "███".bright_red(),
            "███".bright_yellow(),
            "███".bright_green(),
            "███".bright_cyan(),
            "███".bright_blue(),
            "███".bright_magenta(),
            "███".bright_black(),
            "███".bright_white()
        ),
        format!(
            "{}{}{}{}{}{}{}{}",
            "███".red(),
            "███".yellow(),
            "███".green(),
            "███".cyan(),
            "███".blue(),
            "███".magenta(),
            "███".black(),
            "███".white()
        ),
    ];

    render(info);
}
