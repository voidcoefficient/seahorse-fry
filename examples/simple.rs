use seahorse_fry::{app::App, command::Command};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let app = App::new("simple")
        .description("a simple app to test `seahorse_fry`")
        .command(
            Command::new("foo")
                .description("bar")
                .action(|_| println!("from command foo!")),
        );

    app.run(args);
}
