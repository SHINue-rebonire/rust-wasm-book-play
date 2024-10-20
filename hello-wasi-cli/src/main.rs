use ferris_says::say;

fn main() {
    say("Hello, world!", 80, &mut std::io::stdout()).unwrap();
}
