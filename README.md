# rf-custom
Customize rEFInd default boot and more right from the command line.

Ensure rEFInd is installed to your EFI partition.  If your efi partition directory (labeled EFI) is not in /boot/efi/, include the path to the directory using `-p`
```
rf-custom 0.1
Jack R
Customize rEFInd default boot and other settings.  Specify one argument (excluding efi-path)

USAGE:
    rf-custom [FLAGS] [OPTIONS]

FLAGS:
    -c, --clear      Clear all rules, previously booted OS will be default
    -g, --get        Get the current selection configuration, and any defaults set by this program
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --efi-path <PATH>       Sets the path to the EFI folder.  DEFAULT: /boot/efi/
    -n, --number <N>            Default to the nth option in the menu (from left to right) [possible
                                values: 1, 2, 3, 4, 5, 6, 7, 8, 9]
    -s, --substring <STRING>    Default to the first matching string of any boot description,
                                separate multiple strings by commas

This program can change the boot selection when the timeout reaches zero, and other settings for the
rEFInd bootloader, which must be installed to your EFI partition.
```

## Building
Just use `cargo build` after installing rust.

### License
This project is licensed under the EUPL 1.2.

### Contributing
Contrbutions welcome.
