use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    actual_input: bool,
    #[arg(long)]
    star_2: bool,
}

pub fn initialize<'a>(actual_input: &'static str, test_input: &'a str) -> (&'a str, bool) {
    let args = Args::parse();

    let input = if args.actual_input {
        actual_input
    } else {
        test_input
    };

    (input, args.star_2)
}

#[macro_export]
macro_rules! initialize_macro {
    ($test_input:expr) => {{
        use advent_of_code::initialize;
        initialize(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                // "/puzzles/2024/",
                "/puzzles/",
                env!("CARGO_BIN_NAME"),
                // stringify!($day),
                "/input"
            )),
            stringify!($test_input),
        )
    }};
}
