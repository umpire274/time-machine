use crate::ui::terminal::print_line;

pub fn print_briefing() {
    print_line("");
    print_line("TMO BASE – BRIEFING");
    print_line("");

    print_line("You have been selected as a Temporal Operative of the Time Machine Organization.");
    print_line(
        "Your mission is to recover the fragments of the Noks scattered across human history.",
    );
    print_line(
        "Due to current limitations, each temporal jump will last no longer than three days.",
    );
    print_line("Failure or death will result in an automatic recall to this base.");
    print_line("");
    print_line("When you are ready, the Time Machine will be calibrated for your first jump.");
    print_line("");
}

pub fn print_egypt_briefing() {
    print_line("");
    print_line("TEMPORAL MISSION BRIEFING – ANCIENT EGYPT");
    print_line("");

    print_line("The temporal coordinates point to Ancient Egypt, during a period of political and");
    print_line("religious instability. The fragment of the Noks is believed to be hidden within a");
    print_line("restricted area connected to a temple complex.");
    print_line("");
    print_line(
        "Your objective is to locate and recover the fragment without altering the timeline.",
    );
    print_line("Direct interaction with key historical figures must be avoided.");
    print_line("");
    print_line("When ready, type 'jump' to initiate the temporal transfer.");
    print_line("");
}
