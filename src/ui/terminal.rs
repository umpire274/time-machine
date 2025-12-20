use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

const DEFAULT_DELAY_MS: u64 = 40;

/* -------- core internals -------- */

fn typewriter_print_internal(text: &str, delay_ms: u64) {
    for ch in text.chars() {
        print!("{ch}");
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
}

/* -------- public API -------- */

pub fn print(text: &str) {
    typewriter_print_internal(text, DEFAULT_DELAY_MS);
}

pub fn print_with_delay(text: &str, delay_ms: u64) {
    typewriter_print_internal(text, delay_ms);
}

pub fn print_line(text: &str) {
    print(text);
    println!();
}

pub fn print_line_with_delay(text: &str, delay_ms: u64) {
    print_with_delay(text, delay_ms);
    println!();
}

pub fn print_intro() {
    print_line("Time Machine");
    print_line("Temporal Exploration Program");
    print_line("Type 'quit' to exit.");
    println!();
}

pub fn print_unknown() {
    print_line("Command not recognized.");
}

pub fn print_goodbye() {
    print_line("Session terminated. Returning to TMO base.");
}

pub fn read_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn wait_for_enter() {
    use std::io;

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
}

pub fn pause_ms(ms: u64) {
    sleep(Duration::from_millis(ms));
}
