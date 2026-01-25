
/* 
Hello! Welcome to Afterfetch. 
a: awesome
f: fetch
t: tool
e: eroded in
r: rust 

This is currently Linux-only, but feel free to edit the source code
and put in Windows compatibility.

Be-ware! this project was made for fun.
not written by a pro
its pretty cool tho

sorry for not commenting anything i just kind of got distracted and now i just looked 
back lol uhh yeah

Good Luck!

-- n8t-lx
*/

use std::fs;
use std::process::Command;

fn main() {
    println!("\x1b[34m   __ _  / _| |_ ___ _ __ \x1b[0m");
    println!("\x1b[35m  / _` || |_| __/ _ \\ '__|\x1b[0m");
    println!("\x1b[34m | (_| ||  _| ||  __/ |   \x1b[0m   \x1b[1mAfterfetch\x1b[0m");
    println!("\x1b[35m  \\__,_||_|  \\__\\___|_|   \x1b[36m   Eroded in Rust\n");
        
        // hostnameee
     let hostname = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "host"
            .to_string());
    println!("{}", hostname.trim());
    println!("-------------------------->");
    // OPERATING SYSTEM CHECKS -- 
    
    let os = std::fs::read_to_string("/etc/os-release").expect("test"); // checks your fs, reads it.
    let osname = os.lines() // the variable is os.lines()
        .find(|line| line.starts_with("PRETTY_NAME")) // looks for line that begins w 'PRETTY_NAME'
        .and_then(|line| line.split('"').nth(1)) // after that, it splits it.
        .unwrap_or("Unknown"); // This unwraps the text, and it is sure the file is there. or, panic.
    println!("\x1b[34mos: {} \x1b[0m", osname); // This prints out the OS name
    
    // :) 
    
    match osname {
        "Arch Linux" => {
            print!(" -- nice"); // :)
        }
        _ => {
            () // Do nothing.
        }
    }
    // Memory Usage
    let memusg = std::fs::read_to_string("/proc/meminfo").expect("test");
    let memusgd = memusg.lines()
        .find(|line| line.starts_with("MemTotal:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("0");
        
    let memusg2d = memusg.lines()
        .find(|line| line.starts_with("MemAvailable:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("0");
    
    
    let memusgd2p: u32 = memusg2d.trim()
    .parse()
    .expect("test");
    
    let memusgdp: u32 = memusgd.trim()
    .parse()
    .expect("test");
    
    let formulamem = (memusgdp - memusgd2p) / 1024;
    
    let memusgdpdiv  = memusgdp / 1024;
    
    println!("\x1b[35mram: \x1b[0m {formulamem} mbytes / {memusgdpdiv} mbytes");

    let mut dothemathforme: f32 = 1.024;
    dothemathforme *= 100.0;
    if (formulamem as f32) < dothemathforme {
        print!(" -- thats pretty low \n");
    } 
    
    let uptime = std::fs::read_to_string("/proc/uptime").expect("test");
    let uptimed: f32 = uptime.split_whitespace()
    .next()
    .unwrap_or("0")
    .parse()
    .expect("test");
    
    let uptimedmin: u32 = (uptimed / 60.0) as u32;
    let remsec = uptimed % 60.0;
    println!("\x1b[34muptime: \x1b[0m {uptimedmin}min, {:.0}sec.", remsec);
    
    // CPU
    let cpu = std::fs::read_to_string("/proc/cpuinfo").expect("test");
    let cpuname = cpu.lines()
    .find(|line| line.starts_with("model name"))
    .and_then(|line| line.split(':').nth(1))
    .unwrap_or("unregistered")
    .trim();
    
    println!("\x1b[35mcpu: \x1b[0m {cpuname}");
    
    // GPU SEARCH AGUGUGHGNGHGHGHG
    let output = Command::new("sh")
    .arg("-c")
    .arg("lspci | grep -E 'VGA|3D' | cut -d ':' -f3")
    .output()
    .expect("failed to execute");

let gpu_name = String::from_utf8_lossy(&output.stdout);

       if !gpu_name.trim().is_empty() {
    println!("\x1b[33mgpu model: \x1b[0m  {} ", gpu_name.trim());
       }
    
 let drm_path = "/sys/class/drm/";
    let mut found = false;
    if let Ok(entries) = fs::read_dir(drm_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy();

            if name.starts_with("card") && !name.contains('-') {
                let vendor_file = path.join("device/vendor");
                let device_file = path.join("device/device");

                if let (Ok(v_id), Ok(d_id)) = (fs::read_to_string(vendor_file), fs::read_to_string(device_file)) {
                    let vendor_hex = v_id.trim();
                    let device_hex = d_id.trim();

                    let vendor_name = match vendor_hex {
                        "0x8086" => "intel",
                        "0x10de" => "nvidia",
                        "0x1002" | "0x1022" => "amd", 
                        _ => "unregistered",
                    };

                    if vendor_name != "unregistered" {
                        println!("\x1b[32mgpu specifics: \x1b[0m {vendor_name} [{device_hex}] (found at {name})");
                        found = true;
                    }
                }
            }
        }
    }

    if !found {
        println!("gpu: unregistered [none]");
    }
    // kernel version!
    let kernelv = std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_else(|_| "kernel not found".to_string());
    println!("\x1b[31mkernel: \x1b[0m {kernelv}");
    
    // user
    let user = std::env::var("USER")
        .unwrap_or_else(|_| "user"
            .to_string());
    println!("\x1b[35muser: \x1b[0m {user} ");

    // reso
let reso = Command::new("sh")
    .arg("-c")
    .arg("xrandr | grep '*' | awk '{print $1}'")
    .output()
    .ok()
    .and_then(|out| {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if s.is_empty() { None } else { Some(s) }
    })
    .unwrap_or_else(|| {
        std::fs::read_to_string("/sys/class/graphics/fb0/virtual_size")
            .unwrap_or_else(|_| "Unknown".to_string())
            .trim()
            .replace(',', "x")
    });

println!("\x1b[34mresolution: \x1b[0m {}", reso);

    
    // ughhhh
    let disk = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .and_then(|s| {
            s.lines()
                .nth(1)? 
                .split_whitespace()
                .nth(2) 
                .map(|val| val.to_string())
        })
        .unwrap_or_else(|| "0".to_string());


    let totalgb = fahhhh();

    println!("\x1b[37mdisk used: {} / {} \x1b[0m", disk, totalgb);
}
    
    
    
    fn fahhhh() -> String {
    let bp = "/sys/class/block";

    if let Ok(entries) = fs::read_dir(bp) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.starts_with("sd") || name_str.starts_with("nvme") || name_str.starts_with("vd") {
                let size_path = format!("{}/{}/size", bp, name_str);
                
                if let Ok(size_str) = fs::read_to_string(size_path) {
                    if let Ok(sectors) = size_str.trim().parse::<u64>() {
                        let gb = (sectors as f64 * 512.0) / 1_000_000_000.0;
                        return format!("{:.0}gbytes", gb);
                    }
                }
            }
        }
    }
    "Unknown".to_string()
}
