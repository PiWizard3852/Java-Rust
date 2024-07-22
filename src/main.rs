mod parser;

use parser::lexer;

fn main() {
    println!("{:#?}", lexer::lex("
    public class Main {
        public static void main(String[] args) {
            if (20 > 18) {
                System.out.println(\"20 is greater than 18\");
            }
        }
    }
    "));
}
