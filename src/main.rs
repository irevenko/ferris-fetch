use colored::*;
use sysinfo::{ProcessorExt, System, SystemExt};

const FERRIS_ART: &str = r###"
              ▄   ▓▄ ▄▓▓  ▓▓
            ▄  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ▄
           ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
        ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▌
      ▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▄▄▄
      ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▌
   ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▀▓▄▒▓▓▓▓▓▀▓▄▓▓▓▓▓▓▓▓▓▓▓▓▓
    ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▌ ▐██▒▓▓▒▌ ██▌▓▓▓▓▓▓▓▓▓▓▓
  ▄▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█████▒▓▓▒████▌▓▓▓▓▓▓▓▓▓▓▓▓▓▄
 ▓▓▓▌▀▓▓▓▓▓▓▓▓▓▓▒▄▄▌▒▓▓▓▓▓▒▒▄▒▒▓▓▓▓▒▒▓▓▀▀▓▀▓▓▓
  ▀▓▓▄ ▀▄ ▀▓▓▀▓▀▒▓▓▓▒▀▓▒▓▓▀▒▓▓▓▒▀▓▀▓▒▓▀  ▀ ▐▓▀
    ▓▄  ▄  ▀▓▓▓▓▓▀▀▀         ▓▓▓▓▓▀    ▀ ▄▓
      ▀       ▀▓▓▓▓▄▄     ▄▄▓▓▓▀         ▀
                 ▀▀▀▀     ▀▀▀             
"###;

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

    let mut cargo_vec: Vec<String> = Vec::new();

    for line in cargo_installs {
        if !line.starts_with("    ") { 
            cargo_vec.push(line.to_string());
        }
    }
    return cargo_vec.len();
}

fn get_kernel() -> Option<String> {
    System::new().get_kernel_version()
}

fn render(info: Vec<String>) {
    let lines = FERRIS_ART.lines();
    let mut i = 0;
    let empty = String::from("");
    for line in lines {
        println!(
            "{}{}",
            line.red(),
            match i < info.len() {
                true => {
                    i += 1;
                    &info[i - 1]
                }
                false => &empty,
            }
        );
    }
}

fn main() {
    let mut info: Vec<String> = vec![];

    let cpu_sys = System::new();
    let cpu = cpu_sys.get_processors();
    let cpu = cpu[0].get_brand();

    let mut ram_sys = sysinfo::System::new_all();
    let used_ram = ram_sys.get_used_memory() / 1024;
    let total_ram = ram_sys.get_total_memory() / 1024;
    ram_sys.refresh_all();

    let rustc_cmd = get_ver("rustc -V");
    let cargo_cmd = get_ver("cargo -V");
    let rustup_cmd = get_ver("rustup -V");
    let rust_ver: Vec<&str> = rustc_cmd.split_whitespace().collect();
    let cargo_ver: Vec<&str> = cargo_cmd.split_whitespace().collect();
    let rustup_ver: Vec<&str> = rustup_cmd.split_whitespace().collect();
    let cargo_packages = get_cargo_crates();

    // hell formatting (because of the crab shape)
    info.push("".to_string());
    info.push("".to_string());
    info.push(format!(
        "              {}{}{}",
        whoami::username().bright_red().bold(),
        "@".bold(),
        whoami::hostname().bright_red().bold()
    ));
    info.push(format!("              {}", "════════════════"));
    info.push(format!(
        "          {}{}",
        "rust ver: ".bright_red(),
        rust_ver[1]
    ));
    info.push(format!(
        "        {}{}",
        "rustup ver: ".bright_red(),
        rustup_ver[1]
    ));
    info.push(format!(
        "        {}{}",
        "cargo ver: ".bright_red(),
        cargo_ver[1]
    ));
    info.push(format!(
        "      {}{}",
        "cargo crates: ".bright_red(),
        cargo_packages
    ));
    info.push(format!("	  {}{}", "os: ".bright_red(), whoami::distro()));
    if let Some(kernel) = get_kernel() {
        info.push(format!("	  {}{}", "kernel: ".bright_red(), kernel));
    }
    info.push(format!("	  {}{}", "cpu: ".bright_red(), cpu));
    info.push(format!(
        "	  {}{} » {} MB",
        "ram: ".bright_red(),
        used_ram,
        total_ram
    ));
    info.push("".to_string());
    info.push(format!(
        "	  {}{}{}{}{}{}{}{}",
        "███".bright_red(),
        "███".bright_yellow(),
        "███".bright_green(),
        "███".bright_cyan(),
        "███".bright_blue(),
        "███".bright_magenta(),
        "███".bright_black(),
        "███".bright_white()
    ));
    info.push(format!(
        "	  {}{}{}{}{}{}{}{}",
        "███".red(),
        "███".yellow(),
        "███".green(),
        "███".cyan(),
        "███".blue(),
        "███".magenta(),
        "███".black(),
        "███".white()
    ));

    render(info);
}
