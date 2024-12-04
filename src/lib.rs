use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    actual_input: bool,
    #[arg(long)]
    star_2: bool,
}

pub fn initialize<'a>(actual_input: &'static str, test_input: &'a str) -> (&'a str, bool, bool) {
    let args = Args::parse();

    let input = if args.actual_input {
        actual_input
    } else {
        test_input
    };

    (input, args.star_2, args.actual_input)
}

#[macro_export]
macro_rules! initialize_macro {
    ($test_input:tt) => {{
        use advent_of_code::initialize_macro;
        initialize_macro!(env!("CARGO_BIN_NAME"), $test_input)
    }};
    ($path:expr, $test_input:tt) => {{
        use advent_of_code::initialize;
        initialize(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/puzzles/",
                $path,
                "/input"
            )),
            $test_input,
        )
    }};
}
