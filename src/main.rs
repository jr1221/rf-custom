use clap::{App, AppSettings, Arg};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let matches = App::new("rf-custom")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1")
        .author("Jack R")
        .about("Customize rEFInd default boot and other settings.  Specify one argument (excluding efi-path)")
        .after_help("This program can change the boot selection when the timeout reaches zero, and other settings for the rEFInd bootloader, which must be installed to your EFI partition.")
        .arg(Arg::new("efi-path")
            .short('p')
            .long("efi-path")
            .value_name("PATH")
            .about("Sets the path to the EFI folder.  DEFAULT: /boot/efi/"))
        .arg(Arg::new("number")
            .conflicts_with_all(&["substring","get", "clear"])
            .short('n')
            .long("number")
            .value_name("N")
            .about("Default to the nth option in the menu (from left to right)")
            .possible_values(&["1","2","3","4","5","6","7","8","9",])
        )
        .arg(Arg::new("substring")
            .conflicts_with_all(&["number","get", "clear"])
            .short('s')
            .long("substring")
            .about("Default to the first matching string of any boot description, separate multiple strings by commas")
            .value_name("STRING"))
        .arg(Arg::new("get")
            .conflicts_with_all(&["number","substring", "clear"])
            .short('g')
            .long("get")
            .about("Get the current selection configuration, and any defaults set by this program"))
            .arg(Arg::new("clear")
                .conflicts_with_all(&["number","get", "substring"])
                .short('c')
                    .long("clear")
                    .about("Clear all rules, previously booted OS will be default")
            )
        .get_matches();

    let mut efipath = matches
        .value_of("efi-path")
        .unwrap_or("/boot/efi/")
        .to_string();
    if !efipath.ends_with("/") {
        &efipath.push('/');
    }

    let mut refind_conf = OpenOptions::new()
        .append(true)
        .read(true)
        .open(format!("{}{}", efipath, "EFI/refind/refind.conf"))
        .unwrap();

    if BufReader::new(&refind_conf)
        .lines()
        .into_iter()
        .find(|x| {
            x.as_ref()
                .unwrap()
                .eq("include refind-default-selections.conf")
        })
        .is_none()
    {
        writeln!(refind_conf, "include refind-default-selections.conf")
            .expect("Error writing to refind.conf");
    }

    let mut refind_defaults = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(format!(
            "{}{}",
            efipath, "EFI/refind/refind-default-selections.conf"
        ))
        .unwrap();

    if matches.is_present("number") {
        refind_defaults.set_len(0).expect("Error");
        refind_defaults
            .write(
                format!(
                    "{}{}",
                    "default_selection ",
                    matches.value_of("number").unwrap()
                )
                .as_bytes(),
            )
            .expect("Error adding default selection (num)");
    }

    if matches.is_present("substring") {
        refind_defaults.set_len(0).expect("Error");
        refind_defaults
            .write(
                format!(
                    "{}{}",
                    "default_selection ",
                    matches.value_of("substring").unwrap()
                )
                .as_bytes(),
            )
            .expect("Error adding default selection (str)");
        return;
    }

    if matches.is_present("get") {
        for l in BufReader::new(refind_defaults).lines() {
            println!("{}", l.unwrap());
        }
        return;
    }
    if matches.is_present("clear") {
        refind_defaults.set_len(0).expect("Error clearing file");
        return;
    }
}
