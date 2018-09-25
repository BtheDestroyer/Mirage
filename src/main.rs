extern crate termion;

use termion::color;

fn prep_terminal() {
    
}

fn show_title() {
    let color_main = color::LightBlue;
    let color_leaves = color::LightGreen;
    let color_trunk = color::LightYellow;
    println!("{}{}{}{}{}{}", color::Fg(color_main), "                           ", color::Fg(color_leaves), "               ", color::Fg(color_main), "                                                              _____   ");
    println!("{}{}{}{}{}{}", color::Fg(color_main), "        ___________        ", color::Fg(color_leaves), "  ____________ ", color::Fg(color_main), "___________          _____                _____          _____\\    \\  ");
    println!("{}{}{}{}{}{}", color::Fg(color_main), "       /           \\       ", color::Fg(color_leaves), " /            \\", color::Fg(color_main), "\\          \\       /      |_         _____\\    \\_       /    / |    | ");
    println!("{}{}{}{}{}{}{}{}{}{}", color::Fg(color_main), "      /    _   _    \\      ", color::Fg(color_leaves), "|\\___/", color::Fg(color_trunk), "\\  \\", color::Fg(color_leaves), "\\___/|", color::Fg(color_main), "\\    /\\    \\     /         \\       /     /|     |     /    /  /___/| ");
    println!("{}{}{}{}{}{}{}{}{}{}", color::Fg(color_main), "     /    //   \\\\    \\     ", color::Fg(color_leaves), " \\|____", color::Fg(color_trunk), "\\  \\", color::Fg(color_leaves), "___|/", color::Fg(color_main), " |   \\_\\    |   |     /\\    \\     /     / /____/|    |    |__ |___|/ ");
    println!("{}{}{}{}{}{}", color::Fg(color_main), "    /    //     \\\\    \\    ", color::Fg(color_trunk), "       |  |    ", color::Fg(color_main), "  |      ___/    |    |  |    \\   |     | |_____|/    |       \\       ");
    println!("{}{}{}{}{}{}{}{}{}{}", color::Fg(color_main), "   /     \\\\_____//     \\   ", color::Fg(color_leaves), "  __", color::Fg(color_trunk), "  /   /", color::Fg(color_leaves), " __ ", color::Fg(color_main), "  |      \\  ____ |     \\/      \\  |     | |_________  |     __/ __    ");
    println!("{}{}{}{}{}{}{}{}{}{}", color::Fg(color_main), "  /       \\ ___ /       \\  ", color::Fg(color_leaves), " /  \\", color::Fg(color_trunk), "/   /", color::Fg(color_leaves), "_/  |", color::Fg(color_main), " /     /\\ \\/    \\|\\      /\\     \\ |\\     \\|\\        \\ |\\    \\  /  \\   ");
    println!("{}{}{}{}{}{}", color::Fg(color_main), " /________/|   |\\________\\ ", color::Fg(color_leaves), "|____________/|", color::Fg(color_main), "/_____/ |\\______|| \\_____\\ \\_____\\| \\_____\\|    |\\__/|| \\____\\/    |  ");
    println!("{}{}{}{}{}{}", color::Fg(color_main), "|        | |   | |        |", color::Fg(color_leaves), "|           | /", color::Fg(color_main), "|     | | |     || |     | |     || |     /____/| | ||| |    |____/|  ");
    println!("{}{}{}{}{}{}", color::Fg(color_main), "|________|/     \\|________|", color::Fg(color_leaves), "|___________|/ ", color::Fg(color_main), "|_____|/ \\|_____| \\|_____|\\|_____| \\|_____|     |\\|_|/ \\|____|   | |  ");
    println!("{}{}{}{}{}{}", color::Fg(color_main), "                           ", color::Fg(color_leaves), "               ", color::Fg(color_main), "                                          |____/             |___|/   ");
    println!("{}                                                                                                     v{}.{}.{}", color::Fg(color::Reset), env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));
}

fn main() {
    let _guard = termion::init();
    prep_terminal();
    show_title();
}
