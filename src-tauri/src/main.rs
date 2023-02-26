// use std::net::{IpAddr, Ipv4Addr};
// use itertools::Itertools;

use std::process::Command;
use regex::Regex;

// we aren't compiling for windows :frcoal:

#[tauri::command]
fn ping(ip: &str) -> f32 {
    // let (split_ip1, split_ip2, split_ip3, split_ip4) = ip.split(".").collect_tuple().unwrap();
    // let _a = ping::ping(IpAddr::V4(Ipv4Addr::new(split_ip1.parse::<u8>().unwrap(), split_ip2.parse::<u8>().unwrap(), split_ip3.parse::<u8>().unwrap(), split_ip4.parse::<u8>().unwrap())), None, None, None, Some(3), None).unwrap();
    // return _a;
    let output = 
    Command::new("sh")
            .arg("-c")
            .arg(format!("ping -c 3 {}", ip))
            .output();
    
    let dowereturn = match &output {
        Ok(_text) => false,
        Err(_err) => true
    };

    if dowereturn {
        return 9999999.0;
    }

    let outputfin = output.unwrap();

    let out = outputfin.stdout;
    
    let s = String::from_utf8_lossy(&out);
    
    println!("{}", s);

    let re = Regex::new(r"/(1|2|3|4|5|6|7|8|9|0)*\.(1|2|3|4|5|6|7|8|9|0)*/");

    let a = match re.expect("a regex").find(&s) {
        Some(time) => time.as_str(),
        _ => "/9999999.0/"
    };
    return a[1..a.len()-1].to_string().parse::<f32>().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
