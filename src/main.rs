#![allow(non_snake_case)]

const MAX_CHARS: usize = 50; // Max number of characters for the disk's available space bar.

use colored::*;
use sysinfo::{DiskExt, SystemExt};
use std::ffi::{OsString};
use std::env;

fn get_frac(avail: &u64, total: &u64) -> f64 {
    if *total == 0 { // Prevent divide by 0.
        return 0 as f64;
    }

    return 1 as f64 - (*avail as f64/ *total as f64);
}

struct NDFDisk {
    name: String,
    space_asfrac: f64,
    mnt: String
}

impl NDFDisk {
    fn create_NDFDisk(disk: &sysinfo::Disk) -> NDFDisk {
        let frac = get_frac(&disk.get_available_space(), &disk.get_total_space());

        match OsString::from(disk.get_name()).into_string() {
            Ok(s) => {
                NDFDisk {
                    name: s,
                    space_asfrac: frac,
                    mnt: disk.get_mount_point().display().to_string()
                }
            },
            Err(_) => panic!("No name for disk.")
        }
    }
    fn create_bar(&self) -> colored::ColoredString {
        let chars_num = ((MAX_CHARS as f64*self.space_asfrac).ceil()) as usize;
        let chars = "▓".repeat(chars_num);
        let rem_num = (MAX_CHARS - chars_num) as usize;
        let rem = "░".repeat(rem_num);

        if rem_num < (MAX_CHARS as f64 * 0.2) as usize {
            format!("{}{}", chars, rem).red()
        } else {
            format!("{}{}", chars, rem).green()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut compactmode = false;

    match args.get(1) {
        Some(val) => {
            if val == "compact" {
                compactmode = true;
            }
        },
        None => {
            ();
        }
    }

    let sys = sysinfo::System::new();
    let mut disks: Vec<NDFDisk> = Vec::new();
    for disk in sys.get_disks() {
        disks.push(NDFDisk::create_NDFDisk(disk));
    };

    println!("{}", "\nndf - nice disk free".bold());

    if compactmode {
        for disk in disks.into_iter() {
            println!("{}: {} {:.0}%", disk.name, disk.create_bar(), disk.space_asfrac*100 as f64);
        }
    } else {
        for disk in disks.into_iter() {
            println!("{} @ {}\n{} {:.0}%\n", disk.name, disk.mnt, disk.create_bar(), disk.space_asfrac*100 as f64);
        }
    }
}