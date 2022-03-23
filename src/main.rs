extern crate timer;
extern crate chrono;

use sysinfo::{System, SystemExt};
use std::sync::mpsc::channel;

fn main() {
    let sys = System::new_all();
    let system_info = SystemInfo::new(sys);
    loop_timer(system_info);
}

fn loop_timer(mut system_info: SystemInfo) {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    timer.schedule_with_delay(chrono::Duration::seconds(1), move || {
        tx.send(()).unwrap();
    });
      
    let tmp = rx.recv();
    match tmp {
        Ok(_) => {}
        Err(_) => {}
    }
    print!("\x1B[2J\x1B[1;1H");

    system_info.refresh_info();
    system_info.print_system_name();
    system_info.print_total_memory();
    system_info.print_percentage_used_memory();

    loop_timer(system_info);
}

struct SystemInfo {
    sys: sysinfo::System
}

impl SystemInfo {
    fn new(sys: sysinfo::System) -> SystemInfo {
        SystemInfo { sys: sys }
    }

    fn refresh_info(&mut self) {
        self.sys.refresh_all();
    }

    fn print_system_name(&self) {
        let system_name = self.sys.name();
        match system_name {
            Some(n) => {
                println!("OS: {}", n);
            }
            None => {
                println!("Não foi possível obter o nome do sistema operacional");
            }
        }
    }

    fn print_total_memory(&self) {
        println!("Total memory: {}gb", self.sys.total_memory() / 1000000);
    }

    fn print_percentage_used_memory(&self) {
        let total_memory = self.sys.total_memory() as f32;
        let used_memory = self.sys.used_memory() as f32;

        let percentage = (used_memory / total_memory) * 100.0;
        self.print_bar_used_memory(percentage, used_memory);
    }

    fn print_bar_used_memory(&self, percentage: f32, used_memory: f32) {
        let mut bar = "[==========]".to_string();
        let percentage_to_string = percentage.to_string();
        let first_number = percentage_to_string.chars().next().unwrap();

        let number_to_remove = 10 - first_number.to_digit(10).unwrap();

        let mut counter = 0;

        loop {
            bar.pop();

            if counter == number_to_remove {
                break;
            }

            counter += 1;
        }

        counter = 0;

        loop {
            bar.push(' ');

            if counter == number_to_remove - 1 {
                bar.push(']');
                break;
            }

            counter += 1;
        }

        println!("Used memory: {} {:.1}% ➡️ {:.1}gb", bar, percentage, used_memory / 1000000.0);
    } 
}