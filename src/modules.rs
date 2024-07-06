#![allow(non_snake_case)]

use sysinfo::System;
use nix::{
            sys::uio::{RemoteIoVec, process_vm_readv},
            unistd::Pid,
        };
use colored::Colorize;

// Getting the process PID
pub fn getPID(pname: String) -> i32 {

    // Create sysinfo object and refresh to collect current os state
    let mut sys = System::new_all();
    sys.refresh_all();

    // Getting process
    let ppid = sys.processes_by_name(&pname).take(1).next().expect(format!("[{pname}] Process not found").as_str());

    // Process PID
    ppid.pid().as_u32() as i32
}


// Getting the heap address
pub fn heapADDR(pid: i32) -> (usize, usize) {
    
    // Read MAP file
    let data = std::fs::read_to_string(format!("/proc/{}/maps", pid)).unwrap();

    for line in data.split("\n")  {
       
        // Check line contains [heap]
        if line.contains("[heap]") {

            let value = line.split(" ").next().unwrap();
            let mut value = value.split("-");
            let (start, end) = (value.nth(0).unwrap(), value.nth(0).unwrap());

            let startADDR = u64::from_str_radix(start, 16).unwrap() as usize;
            let endADDR   = u64::from_str_radix(end, 16).unwrap() as usize;

            return (startADDR, endADDR);
        }
    }
        std::process::exit(1)
}


// Reading the process memory
pub fn readMEM(pid: i32, startADDR: usize, endADDR: usize) {
  
    let mut data = [0; 16];

    for addr in (startADDR..=endADDR).step_by(16) {
        // Local buffer 
        let local_iov = std::io::IoSliceMut::new(&mut data);

        // Remote buffer
        let remote_iov = RemoteIoVec {
                                        base: addr,
                                        len: 16,
                                    };
        
        // process_vm_readv() returns the number of bytes read 
        let nread = process_vm_readv(Pid::from_raw(pid), &mut [local_iov], &[remote_iov]).unwrap();

        if nread as i32 != 0 {

            for i in data {

                if i != 0 {
                    // Printing address form start-end 
                    print!("{}", format!("{:#x} | ", addr).white());
                        
                    // Printing the hex value 
                    for i in data {
                        
                        if (0x00..0x7E).contains(&i) {
                            print!("{}", format!(" {:02x} ", i).white().on_blue());
                        } else if i == 0x00 {
                            print!("{}", format!(" {:02x} ", i).red()); 
                        } else {
                             print!("{}", format!(" {:02x} ", i).yellow());
                        }

                    }

                    print!("{}", format!(" | ").white());

                    // Hex to ascii 
                    for i in data {
                        if i != 0x00 && (0x20..0x7E).contains(&i) {
                            print!("{}", format!("{}", String::from_utf8(vec![i]).unwrap()).cyan());
                        } else if i != 0x00 {
                            print!("{}", ".".to_string());
                        }
                    }

                    println!();
                    break;

                } else {
                         continue; 
                }
            }
        }
    }
}


