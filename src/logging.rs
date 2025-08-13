use colored::Colorize;

pub fn print_banner(title: &str, color: fn(&str) -> colored::ColoredString) {
    let border = "╔═══════════════════════════════════════════════════════════════╗";
    let bottom = "╚═══════════════════════════════════════════════════════════════╝";
    
    println!("{}", color(border));
    println!("{}", color(&format!("║{:^63}║", title)));
    println!("{}", color(bottom));
}

