extern crate colored;

use colored::*;

fn prep_terminal() {
    
}

fn show_title() {
    let color_main = "bright blue";
    let color_leaves = "bright green";
    let color_trunk = "bright yellow";
    println!("{}{}{}", "                           ".color(color_main), "               ".color(color_leaves), "                                                              _____   ".color(color_main));
    println!("{}{}{}", "        ___________        ".color(color_main), "  ____________ ".color(color_leaves), "___________          _____                _____          _____\\    \\  ".color(color_main));
    println!("{}{}{}", "       /           \\       ".color(color_main), " /            \\".color(color_leaves), "\\          \\       /      |_         _____\\    \\_       /    / |    | ".color(color_main));
    println!("{}{}{}{}{}", "      /    _   _    \\      ".color(color_main), "|\\___/".color(color_leaves), "\\  \\".color(color_trunk), "\\___/|".color(color_leaves), "\\    /\\    \\     /         \\       /     /|     |     /    /  /___/| ".color(color_main));
    println!("{}{}{}{}{}", "     /    //   \\\\    \\     ".color(color_main), " \\|____".color(color_leaves), "\\  \\".color(color_trunk), "___|/".color(color_leaves), " |   \\_\\    |   |     /\\    \\     /     / /____/|    |    |__ |___|/ ".color(color_main));
    println!("{}{}{}", "    /    //     \\\\    \\    ".color(color_main), "       |  |    ".color(color_trunk), "  |      ___/    |    |  |    \\   |     | |_____|/    |       \\       ".color(color_main));
    println!("{}{}{}{}{}", "   /     \\\\_____//     \\   ".color(color_main), "  __".color(color_leaves), "  /   /".color(color_trunk), " __ ".color(color_leaves), "  |      \\  ____ |     \\/      \\  |     | |_________  |     __/ __    ".color(color_main));
    println!("{}{}{}{}{}", "  /       \\ ___ /       \\  ".color(color_main), " /  \\".color(color_leaves), "/   /".color(color_trunk), "_/  |".color(color_leaves), " /     /\\ \\/    \\|\\      /\\     \\ |\\     \\|\\        \\ |\\    \\  /  \\   ".color(color_main));
    println!("{}{}{}", " /________/|   |\\________\\ ".color(color_main), "|____________/|".color(color_leaves), "/_____/ |\\______|| \\_____\\ \\_____\\| \\_____\\|    |\\__/|| \\____\\/    |  ".color(color_main));
    println!("{}{}{}", "|        | |   | |        |".color(color_main), "|           | /".color(color_leaves), "|     | | |     || |     | |     || |     /____/| | ||| |    |____/|  ".color(color_main));
    println!("{}{}{}", "|________|/     \\|________|".color(color_main), "|___________|/ ".color(color_leaves), "|_____|/ \\|_____| \\|_____|\\|_____| \\|_____|     |\\|_|/ \\|____|   | |  ".color(color_main));
    println!("{}{}{}", "                           ".color(color_main), "               ".color(color_leaves), "                                          |____/             |___|/   ".color(color_main));
    println!("                                                                                                     v{}.{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));
}

fn main() {
    prep_terminal();
    show_title();
}
