use seahorse_fry::{app::App, command::Command};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let app = App::new("simple")
        .description("a simple app to test `seahorse_fry`")
        .command(
            Command::new("foo")
                .description("bar")
                .subcommand(Command::new("baz").description("a simple desc"))
                .action(|ctx| {
                    dbg!(ctx.flags());
                    dbg!(ctx.values());
                    dbg!(ctx.value());
                }),
        );

    app.run(args);
}
