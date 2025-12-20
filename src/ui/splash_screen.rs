use crate::ui::terminal;
use crate::ui::terminal::{print_line_with_delay, wait_for_enter};

pub fn splash_screen() {
    let delay = 20;
    
    print_line_with_delay("████████╗██╗███╗   ███╗███████╗", delay);
    print_line_with_delay("╚══██╔══╝██║████╗ ████║██╔════╝", delay);
    print_line_with_delay("   ██║   ██║██╔████╔██║█████╗", delay);
    print_line_with_delay("   ██║   ██║██║╚██╔╝██║██╔══╝", delay);
    print_line_with_delay("   ██║   ██║██║ ╚═╝ ██║███████╗", delay);
    print_line_with_delay("   ╚═╝   ╚═╝╚═╝     ╚═╝╚══════╝", delay);
    print_line_with_delay("", delay);
    print_line_with_delay(
        "███╗   ███╗ █████╗  ██████╗██╗  ██╗██╗███╗   ██╗███████╗",
        20,
    );
    print_line_with_delay(
        "████╗ ████║██╔══██╗██╔════╝██║  ██║██║████╗  ██║██╔════╝",
        20,
    );
    print_line_with_delay("██╔████╔██║███████║██║     ███████║██║██╔██╗ ██║█████╗", delay);
    print_line_with_delay("██║╚██╔╝██║██╔══██║██║     ██╔══██║██║██║╚██╗██║██╔══╝", delay);
    print_line_with_delay(
        "██║ ╚═╝ ██║██║  ██║╚██████╗██║  ██║██║██║ ╚████║███████╗",
        20,
    );
    print_line_with_delay(
        "╚═╝     ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚══════╝",
        20,
    );
    print_line_with_delay("", delay);
    print_line_with_delay("Time Machine – Temporal Exploration Program", delay);
    print_line_with_delay("Property of the Time Machine Organization (TMO)", delay);
    print_line_with_delay("", delay);
    print_line_with_delay("Initializing temporal systems...", delay);
    terminal::pause_ms(600);
    print_line_with_delay("Calibrating spacetime coordinates...", delay);
    terminal::pause_ms(900);
    print_line_with_delay("Stabilizing quantum field...", delay);
    terminal::pause_ms(1500);
    print_line_with_delay("", delay);
    print_line_with_delay("Press ENTER to access TMO base.", delay);

    wait_for_enter();
}
