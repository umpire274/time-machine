use crate::engine::state::GameState;
use crate::ui::{help, splash_screen, terminal};

pub fn game_loop(state: &mut GameState) {
    println!();
    splash_screen::splash_screen();
    terminal::print_intro();

    while state.running {
        let input = terminal::read_input();
        handle_input(input, state);
    }
    println!();
}

fn handle_input(input: String, state: &mut GameState) {
    match input.trim() {
        "quit" | "exit" => {
            terminal::print_goodbye();
            state.running = false;
        }
        "help" => {
            help::print_help();
        }
        _ => {
            terminal::print_unknown();
        }
    }
}
