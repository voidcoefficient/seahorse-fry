use seahorse_fry::{
    app::App,
    command::Command,
    flag::{Flag, FlagType, FlagValue},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let app = App::new("simple")
        .description("a simple app to test `seahorse_fry`")
        .command(
            Command::new("foo").description("bar").subcommand(
                Command::new("baz")
                    .description("a simple desc")
                    .usage("simple foo baz <STRING>")
                    .flag(Flag::new("d", FlagType::String).description("does something"))
                    .flag(Flag::new("p", FlagType::Bool).description("toggles"))
                    .action(|ctx| {
                        let p = ctx
                            .get_flag_value("p", FlagType::Bool)
                            .unwrap_or(FlagValue::Bool(false));
                        dbg!(&p);
                    }),
            ),
        );

    app.run(args);
}
