mod board;
mod movegen;
mod debug;
mod search;
mod uci;

fn main() {
    uci::UciParser::run_loop();
}
