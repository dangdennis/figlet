extern crate clap;
// mod render;

use clap::{App, Arg};
// use render::render;

fn main() {
    let matches = App::new("figlet")
        .version("0.1")
        .author("Dennis D. <dang.dennis9@gmail.com>")
        .about("Generates ASCII art from Figlet fonts")
        .arg(
            Arg::with_name("font")
                .short("f")
                .long("font")
                .value_name("font name")
                .help("Font to render with (default: <big.flf>")
                .default_value("standard.flf")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("direction")
                .short("d")
                .long("direction")
                .value_name("choice")
                .help("Select text direction")
                .default_value("auto")
                .takes_value(true),
            // Choices: auto, ltr, rtl
        )
        .arg(
            Arg::with_name("justify")
                .short("j")
                .long("justify")
                .value_name("choice")
                .help("Set justification of text")
                .default_value("auto")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Set terminal width for wrapping/justification")
                .default_value("80")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("reverse")
                .short("r")
                .long("reverse")
                .help("Shows mirror image of output text"),
        )
        .arg(
            Arg::with_name("flip")
                .short("F")
                .long("flip")
                .help("Flips rendered output text over"),
        )
        .arg(
            Arg::with_name("list fonts")
                .short("l")
                .long("list")
                .help("Show all fonts available for use"),
        )
        // .arg(Arg::with_name("font info"))
        // .arg(Arg::with_name("load font"))
        // .arg(Arg::with_name("color"))
        .arg(Arg::with_name("input").required(true))
        .get_matches();

    if let Some(i) = matches.value_of("input") {
        println!("Value for output: {:?}", i);
        // render(i);
    }
}

struct FigletFont {
    comments: str,
    //     self.comment = ''
    // self.chars = {}
    // self.width = {}
    // self.data = self.preloadFont(font)
    // self.loadFont()
}
