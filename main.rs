use std::str;
use colored::*;

fn main() {

  println!("{}", "System info:".bright_blue());

  use sysinfo::{System, SystemExt, DiskExt, ProcessorExt};

  let sys = System::new_all();

  println!("{}", "  disks:".bright_blue());
  println!("    {:32} {:32} {:8} {:8} {:8}", 
    "mount:".bright_blue(), 
    "name:".bright_blue(), 
    "fs:".bright_blue(),
    "size:".bright_blue(),
    "free:".bright_blue());
  for disk in sys.get_disks() {
      println!("    {:32} {:32} {:8} {:8} {:8}", 
      disk.get_mount_point().display(),
      format!("{}", disk.get_name().to_str().unwrap()), 
      format!("{}", str::from_utf8(disk.get_file_system()).unwrap()),
      format!("{}", disk.get_total_space() / 1000000),
      format!("{}", disk.get_available_space() / 1000000));
  }

  println!("{}", "  memory:".bright_blue());
  println!("    {:8} {:<8} MB",
    "total:".bright_blue(), 
    sys.get_total_memory() / 1000);
  println!("    {:8} {:<8} MB", 
    "used:".bright_blue(), 
    sys.get_used_memory() / 1000);

  println!("{}", "  processor:".bright_blue());
  println!("    {:8} {:64}", 
    "brand:".bright_blue(), 
    sys.get_processors()[0].get_brand());
  println!("    {:8} {:<4}", 
    "cores:".bright_blue(), 
    sys.get_processors().len());
  println!("    {:8} {:<8} mHz", 
    "freq:".bright_blue(), 
    sys.get_processors()[0].get_frequency());
}