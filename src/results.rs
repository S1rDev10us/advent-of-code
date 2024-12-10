use std::fmt;

pub fn check_result<T>(
    output: T,
    star_2: bool,
    actual_input: bool,
    test_1: Option<T>,
    actual_1: Option<T>,
    test_2: Option<T>,
    actual_2: Option<T>,
) where
    T: PartialEq<T> + fmt::Debug,
{
    let target = match (star_2, actual_input) {
        (false, false) => test_1,
        (false, true) => actual_1,

        (true, false) => test_2,
        (true, true) => actual_2,
    };

    dbg!(&output);

    if target.is_none() {
        println!("No target set");
        return;
    }

    assert_eq!(target.unwrap(), output);
}
