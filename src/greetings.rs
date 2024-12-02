use colored::*;
use std::{thread, time::Duration};

pub fn print_hello_message() {
    let binding = "
                                                                                        
      /$$$$$$$  /$$$$$$  /$$   /$$ /$$   /$$          /$$$$$$$  /$$$$$$   /$$$$$$$      
     /$$_____/ /$$__  $$|  $$ /$$/| $$  | $$ /$$$$$$ /$$_____/ /$$__  $$ /$$_____/      
    |  $$$$$$ | $$$$$$$$ \\  $$$$/ | $$  | $$|______/|  $$$$$$ | $$  \\ $$| $$            
     \\____  $$| $$_____/  >$$  $$ | $$  | $$         \\____  $$| $$  | $$| $$             
     /$$$$$$$/|  $$$$$$$ /$$/\\  $$|  $$$$$$$         /$$$$$$$/|  $$$$$$/|  $$$$$$$     
    |_______/  \\_______/|__/  \\__/ \\____  $$        |_______/  \\______/  \\_______/      
                                   /$$  | $$                                            
                                  |  $$$$$$/                                            
                                  \\______/                                             
                                                                                        \n"
    .truecolor(193, 251, 222)
    .italic();
    let hello_world_string = binding.split("\n");

    for part in hello_world_string {
        println!("{}", part.truecolor(193, 251, 222).italic());
        thread::sleep(Duration::from_millis(300));
    }
}
