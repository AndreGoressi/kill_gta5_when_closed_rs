extern crate winapi;
mod processes;
mod raw;
use processes::get_processes_by_name;
use std::{thread, time};

fn main() {
    let target_name = "GTA5_Enhanced.exe";
    println!("=== GTA V ENHANCED KILLER START ===");
    println!("Sirching...: {}", target_name);

    loop {
        let processes = get_processes_by_name(target_name, None);

        if processes.is_empty() {
            thread::sleep(time::Duration::from_secs(2));
            continue;
        }

        for process in processes {
            let window = process.get_main_window();
            
            if window.is_none() {
                println!("Zombie gefunden (PID: {})! Fenster ist weg. Kille jetzt...", process.pid);
                match process.kill(Some(0)) {
                    Ok(_) => println!("Erfolgreich terminiert."),
                    Err(e) => println!("Fehler beim Killen: {}", e),
                }
            } else {
                println!("Spiel läuft (PID: {}) - Fenster aktiv.", process.pid);
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
