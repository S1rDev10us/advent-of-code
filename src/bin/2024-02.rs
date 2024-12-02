#[derive(PartialEq, Debug, Copy, Clone)]
enum ReportStatus {
    Rising,
    Falling,
    Unsafe,
    Unset,
}

#[derive(Debug)]
struct StateTracker {
    state: ReportStatus,
    prev: i32,
}

fn main() {
    let actual_input = include_str!("../../puzzles/2024/02/input");
    let test_input = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9";
    let using_actual_input = true;

    let input = if using_actual_input {
        actual_input
    } else {
        test_input
    };

    let reports = input
        .split('\n')
        .map(|report| {
            report
                .split(' ')
                .filter(|num| num.trim() != "")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();
    // [["7", "6", ...], ...]
    let report_status = reports
        .iter()
        .map(|report| {
            for i_removed in 0..report.len() {
                let state = report
                    .iter()
                    .enumerate()
                    .fold(
                        StateTracker {
                            state: ReportStatus::Unset,
                            prev: -1,
                        },
                        |state_tracker, (i, current)| {
                            if i == i_removed {
                                return state_tracker;
                            }
                            // dbg!(&state_tracker);
                            if state_tracker.prev == -1 {
                                StateTracker {
                                    prev: *current,
                                    ..state_tracker
                                }
                            } else {
                                StateTracker {
                                    state: is_safe(
                                        state_tracker.state,
                                        state_tracker.prev,
                                        *current,
                                    ),
                                    prev: *current,
                                }
                            }
                        },
                    )
                    .state;
                if state != ReportStatus::Unsafe {
                    return state;
                }
            }
            // != CurrentState::Unsafe
            ReportStatus::Unsafe
        })
        .filter(|report| *report != ReportStatus::Unset);
    // println!();

    // dbg!(safe_reports.clone().collect::<Vec<&Vec<i32>>>());
    // dbg!(report_status.clone().collect::<Vec<ReportStatus>>());

    let output_1 = report_status
        .filter(|report| *report != ReportStatus::Unsafe)
        .count();

    dbg!(output_1);
    // assert_eq!(if using_actual_input { 230 } else { 2 }, output_1);
    assert_eq!(if using_actual_input { 301 } else { 4 }, output_1);
}

fn change_type(prev: i32, next: i32) -> ReportStatus {
    if prev == next {
        return ReportStatus::Unsafe;
    }

    if prev < next {
        ReportStatus::Rising
    } else {
        ReportStatus::Falling
    }
}

fn is_safe(state: ReportStatus, prev: i32, next: i32) -> ReportStatus {
    if let ReportStatus::Unsafe = state {
        return state;
    }
    // Change > 3
    if (prev - next).abs() > 3 {
        return ReportStatus::Unsafe;
    }
    // Change < 1
    if prev == next {
        return ReportStatus::Unsafe;
    }

    if let ReportStatus::Unset = state {
        return change_type(prev, next);
    }

    if change_type(prev, next) == state {
        state
    } else {
        ReportStatus::Unsafe
    }
}
