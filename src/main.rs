use easy_ansi::*;
fn main() {
    colors();
}
fn colors() {
    println!(
        "{bg_orple} The {BOLD}{BLUE}quick {brown}brown {RED}fox{DEFAULT}{NORMAL} jumped over the {BOLD}{YELLOW}lazy {brown}dog!{RESET}",
        bg_orple = Bg(Color256(53)),
        brown = Fg(TrueColor(0x72, 0x55, 0x3d))
    );
    println!("{BG_RED}{UL}{BOLD}{ST}hihihihihihihihihihihihihihihihihihihiihi");
    println!("hihih{NOUL}ihihihihi{NOIN}hi{BG_BLUE}hihihihihih{NOST}ihihihihihi{RESET}hihihi\r");
    println!("hihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihihi");
}
