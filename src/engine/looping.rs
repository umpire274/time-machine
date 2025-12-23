use crate::engine::state::{GameState, Location};
use crate::mission::mission::MissionKind;
use crate::ui::{briefing, help, splash_screen, terminal};

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
    let trimmed = input.trim();

    match trimmed {
        "quit" | "exit" => {
            terminal::print_goodbye();
            state.running = false;
        }

        "story" => {
            help::print_story();
        }

        "start" => {
            if state.started {
                terminal::print_line("The system is already active.");
            } else {
                state.started = true;
                state.location = Location::BaseTmo;

                briefing::print_briefing();
                terminal::print_line(
                    "Type 'mission' when you are ready to select a temporal jump.",
                );
            }
        }

        "mission" => {
            if state.location != Location::BaseTmo {
                terminal::print_line("Mission selection is only available from the TMO base.");
            } else {
                terminal::print_mission_list();
            }
        }

        _ if trimmed.starts_with("mission ") => {
            if state.location != Location::BaseTmo {
                terminal::print_line("Mission selection is only available from the TMO base.");
                return;
            }

            let index = match trimmed
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse::<usize>().ok())
            {
                Some(i) if i > 0 => i - 1,
                _ => {
                    terminal::print_line("Invalid mission selection.");
                    return;
                }
            };

            let mission = match MissionKind::all().get(index) {
                Some(m) => *m,
                None => {
                    terminal::print_line("Invalid mission selection.");
                    return;
                }
            };

            if !mission.is_available() {
                terminal::print_line("This mission is not yet available.");
                return;
            }

            state.selected_mission = Some(mission);

            match mission {
                MissionKind::AncientEgypt => briefing::print_egypt_briefing(),
                _ => terminal::print_line("Mission briefing not implemented yet."),
            }
        }

        _ => {
            terminal::print_unknown();
        }
    }
}
