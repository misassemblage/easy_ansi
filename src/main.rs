use easy_ansi::*;
fn main() {
    let bg_intorange = Bg::try_from(0xFF4F00).unwrap(); //(227, 127, 150);
    let ul_white = "\x1b[58;5;255m";
    println!("{bg_intorange}{UNDERLINE}{ul_white}{BOLD}Hello,{BLACK} world!{RESET}");
}
