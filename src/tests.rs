use std::process::{Command, Stdio};

macro_rules! test_bin {
    ($bin:ident) => {
        #[test]
        fn $bin() {
            let file_name = &stringify!($bin)[1..].replace("_", "-");
            for star_2 in [false, true] {
                for is_actual in [false, true] {
                    let is_debug = cfg!(debug_assertions);
                    let mut build_command = Command::new("cargo");
                    build_command.args(["build", "--bin", file_name]);
                    if !is_debug {
                        build_command.arg("--release");
                    }
                    build_command
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null());
                    build_command.status().unwrap();

                    let mut command = Command::new(
                        "target/".to_string()
                            + if is_debug { "debug" } else { "release" }
                            + "/"
                            + file_name,
                    );

                    let mut args = vec![];
                    if star_2 {
                        args.push("--star-2");
                    }
                    if is_actual {
                        args.push("-a");
                    }
                    command.args(&args);
                    let output = command.output();
                    if output.is_err() {
                        panic!("Binary {} failed with args '{}'", file_name, args.join(" "))
                    }
                    let output = output.unwrap();
                    if !output.status.success() {
                        panic!(
                            "Binary {} failed with args '{}'.\nOuput:\n\t{}",
                            file_name,
                            args.join(" "),
                            String::from_utf8(output.stderr)
                                .unwrap()
                                .replace("\n", "\n\t")
                        );
                    }

                    println!("Binary {} passed with args '{}'", file_name, args.join(" "));
                }
            }
        }
    };
}
test_bin!(b2024_01);
test_bin!(b2024_02);
test_bin!(b2024_03);
test_bin!(b2024_04);
test_bin!(b2024_05);
test_bin!(b2024_06);
test_bin!(b2024_07);
test_bin!(b2024_08);
test_bin!(b2024_09);
test_bin!(b2024_10);
test_bin!(b2024_11);
test_bin!(b2024_12);
test_bin!(b2024_13);
