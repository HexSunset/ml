use math_lang_lib::parse::lexer::Lexer;

fn main() {
    let prog = "(add_numbers 3 4)";

    for t in Lexer::from_str(prog) {
        println!("{:?}", t);
    }
}
