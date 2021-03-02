// This version is based on the Go version on Rosettacode

#[derive(PartialEq, Debug, Copy, Clone)]
enum Month {
    May,
    June,
    July,
    August,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Birthday {
    month: Month,
    day: u8,
}

impl Birthday {
    fn month_unique_in(&self, birthdays: &[Birthday]) -> bool {
        birthdays
            .iter()
            .filter(|birthday| birthday.month == self.month)
            .count()
            == 1
    }

    fn day_unique_in(&self, birthdays: &[Birthday]) -> bool {
        birthdays
            .iter()
            .filter(|birthday| birthday.day == self.day)
            .count()
            == 1
    }

    fn month_with_unique_day_in(&self, birthdays: &[Birthday]) -> bool {
        birthdays
            .iter()
            .any(|birthday| self.month == birthday.month && birthday.day_unique_in(birthdays))
    }
}

fn solution() -> Option<Birthday> {
    let mut choices: Vec<Birthday> = vec![
        Birthday {
            month: Month::May,
            day: 15,
        },
        Birthday {
            month: Month::May,
            day: 16,
        },
        Birthday {
            month: Month::May,
            day: 19,
        },
        Birthday {
            month: Month::June,
            day: 17,
        },
        Birthday {
            month: Month::June,
            day: 18,
        },
        Birthday {
            month: Month::July,
            day: 14,
        },
        Birthday {
            month: Month::July,
            day: 16,
        },
        Birthday {
            month: Month::August,
            day: 14,
        },
        Birthday {
            month: Month::August,
            day: 15,
        },
        Birthday {
            month: Month::August,
            day: 17,
        },
    ];

    // Albert knows the month but doesn't know the day.
    // So the month can't be unique within the choices.
    let choices_copy = choices.clone();
    choices.retain(|birthday| !(&birthday.month_unique_in(&choices_copy)));

    // Albert also knows that Bernard doesn't know the answer.
    // So the month can't have a unique day.
    let choices_copy = choices.clone();
    choices.retain(|birthday| !(birthday.month_with_unique_day_in(&choices_copy)));

    // Bernard now knows the answer.
    // So the day must be unique within the remaining choices.
    let choices_copy = choices.clone();
    choices.retain(|birthday| birthday.day_unique_in(&choices_copy));

    // Albert now knows the answer too.
    // So the month must be unique within the remaining choices.
    let choices_copy = choices.clone();
    choices.retain(|birthday| birthday.month_unique_in(&choices_copy));

    if choices.len() == 1 {
        Some(choices[0])
    } else {
        None
    }
}

fn main() {
    match solution() {
        Some(solution) => println!("Cheryl's birthday is {:?}", solution),
        None => panic!("Didn't work!"),
    }
}

#[test]
fn test_solution_works() {
    let solution = solution().unwrap();

    let real_solution = Birthday {
        month: Month::July,
        day: 16,
    };

    assert_eq!(solution, real_solution);
}
