// day01 module

use crate::day01::command_line::parameters::Parameters;
use anyhow::{anyhow, Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EntriesError {
    #[error("Unable to find entries for target {target:?}")]
    NoEntriesFound { target: i64 },
}

pub mod command_line;

#[derive(Debug)]
pub struct Entries {
    entry1: i64,
    entry2: i64,
    target: i64,
}

impl Entries {
    fn new(entry1: i64, entry2: i64, target: i64) -> Entries {
        Entries {
            entry1,
            entry2,
            target,
        }
    }
}

fn two_entries_that_sum_to(input: impl Iterator<Item = i64>, target: i64) -> Option<Entries> {
    let mut remainders = std::collections::HashSet::<i64>::new();

    for item in input {
        let remainder = target - item;
        if !remainders.contains(&item) {
            remainders.insert(remainder);
        } else {
            return Some(Entries::new(remainder, item, target));
        }
    }

    None
}

fn process_reader<R: std::io::BufRead + std::io::Read>(
    reader: R,
    target: i64,
) -> anyhow::Result<Entries, anyhow::Error> {
    let empty: Vec<i64> = vec![];
    let mut result_reading_numbers: anyhow::Result<_, anyhow::Error> = Ok(empty.into_iter());
    let reader_numbers =
        reader
            .lines()
            .scan(
                &mut result_reading_numbers,
                |inner_error, inner_result| match inner_result {
                    Ok(o) => match o.parse::<i64>() {
                        Ok(i) => Some(i),
                        Err(e) => {
                            **inner_error = anyhow::Result::Err(anyhow::Error::new(e));
                            None
                        }
                    },
                    Err(e) => {
                        **inner_error = anyhow::Result::Err(anyhow::Error::new(e));
                        None
                    }
                },
            );
    let two_entries_result = two_entries_that_sum_to(reader_numbers, target);

    if result_reading_numbers.is_err() {
        return anyhow::Result::Err(result_reading_numbers.unwrap_err());
    }

    let result = match two_entries_result {
        Some(entries) => anyhow::Result::Ok(entries),
        None => anyhow::Result::Err(anyhow!(EntriesError::NoEntriesFound { target })),
    };

    result
}

pub fn run(parameters: &Parameters) -> Result<()> {
    let input_file = std::fs::File::open(&parameters.input_path).context(format!(
        "Unable to open file '{}'",
        &parameters.input_path.display()
    ))?;
    let reader = std::io::BufReader::new(input_file);
    let entries = process_reader(reader, parameters.target)?;

    println!("{}", entries.entry1 * entries.entry2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day01::{process_reader, two_entries_that_sum_to, Entries, EntriesError};
    use anyhow::Result;
    use std::io::Write;

    // Helper functions
    fn verify_entries_are_equal(left: &Entries, right: &Entries) {
        assert_eq!(left.entry1, right.entry1, "entry1 equal");
        assert_eq!(left.entry2, right.entry2, "entry2 equal");
        assert_eq!(left.target, right.target, "target equal");
    }

    fn ok_verify_entries_are_equal(
        left: &Entries,
        right: &Entries,
    ) -> anyhow::Result<(), anyhow::Error> {
        verify_entries_are_equal(&left, &right);

        Ok(())
    }

    fn verify_result_entries_are_equal(result: &Option<Entries>, expected_entry: &Entries) {
        match result {
            Some(entries) => verify_entries_are_equal(&entries, &expected_entry),
            None => assert!(false, format!("Expecting {:?}", expected_entry)),
        }
    }

    fn verify_result_no_entries_found(result: Option<Entries>) {
        match result {
            Some(entries) => assert!(false, format!("Unexpected entry {:?}", entries)),
            None => (),
        }
    }

    fn verify_process_reader_result_no_entries_found(result: Result<Entries>) {
        match result {
            Err(anyhow_error) => match anyhow_error.downcast::<EntriesError>() {
                Ok(EntriesError::NoEntriesFound { target }) => assert_eq!(target, 30),
                _ => panic!("Expected NoEntriesFound"),
            },
            Ok(entry) => panic!(format!("Expected Err. Got entry {:?}", entry)),
        };
    }

    fn setup_cursor(
        cursor: &mut std::io::Cursor<Vec<u8>>,
        entries: &[&str],
    ) -> std::io::Result<()> {
        let result = cursor.write_all(entries.join("\n").as_bytes());
        if result.is_ok() {
            cursor.set_position(0);
        }
        result
    }

    fn setup_new_cursor(entries: &[&str]) -> std::io::Result<std::io::Cursor<Vec<u8>>> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        if !entries.is_empty() {
            setup_cursor(&mut cursor, &entries)?;
        } else {
            cursor.set_position(0);
        }
        Ok(cursor)
    }

    // fn two_slice_entries_that_sum_to(input: impl Iterator<Item = i64>, target: i64) -> Option<Entries> {
    fn two_slice_entries_that_sum_to(input: &[i64], target: i64) -> Option<Entries> {
        two_entries_that_sum_to(input.iter().cloned(), target)
    }

    // Tests
    #[test]
    fn reading_empty_causes_error() {
        let cursor = setup_new_cursor(&[]).unwrap();

        let result = process_reader(cursor, 30);

        verify_process_reader_result_no_entries_found(result);
    }

    #[test]
    fn reading_10_20_target_30_works() -> anyhow::Result<(), anyhow::Error> {
        const TARGET: i64 = 30;
        let cursor = setup_new_cursor(&["10", "20"])?;

        let result = process_reader(cursor, TARGET)?;

        let expected_entry = Entries::new(10, 20, TARGET);
        ok_verify_entries_are_equal(&expected_entry, &result)
    }

    #[test]
    fn reading_non_numeric_after_valid_entries_10_20_target_30_works(
    ) -> anyhow::Result<(), anyhow::Error> {
        const TARGET: i64 = 30;
        let cursor = setup_new_cursor(&["10", "20", "fish"])?;

        let result = process_reader(cursor, TARGET)?;

        let expected_entry = Entries::new(10, 20, TARGET);
        ok_verify_entries_are_equal(&expected_entry, &result)
    }

    #[test]
    fn reading_non_numeric_causes_error() -> std::io::Result<()> {
        let cursor = setup_new_cursor(&["10", "fish", "20"])?;

        let result = process_reader(cursor, 30);

        match result {
            Result::Err(error) => {
                assert_eq!(error.to_string().as_str(), "invalid digit found in string")
            }
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn find_10_20_target_30_ok() {
        const TARGET: i64 = 30;
        let result = two_slice_entries_that_sum_to(&[10, 20], TARGET);

        let expected_entry = Entries::new(10, 20, TARGET);
        verify_result_entries_are_equal(&result, &expected_entry);
    }

    #[test]
    fn does_not_find_10_20_target_40() {
        let result = two_slice_entries_that_sum_to(&[10, 20], 40);

        verify_result_no_entries_found(result);
    }

    mod find_entry_tests {
        use super::{two_slice_entries_that_sum_to, Entries};
        use crate::day01::tests::{
            verify_result_entries_are_equal, verify_result_no_entries_found,
        };

        parameterized::ide!();

        struct TestParameters<'a> {
            entries: &'a [i64],
            expected: Entries,
        }

        impl<'a> TestParameters<'a> {
            fn new(entries: &'a [i64], expected: Entries) -> Self {
                Self { entries, expected }
            }
        }

        fn params(entries: &[i64], expected: Entries) -> TestParameters {
            TestParameters::new(entries, expected)
        }

        #[parameterized::parameterized(input = {
            params(&[10, 20], Entries::new(10, 20, 30)),
            params(&[0, 10, 0], Entries::new(0, 0, 0)),
            params(&[10, 1, 2, 3, 20], Entries::new(10, 20, 30)),
            params(&[1, 10, 2, 20, 3], Entries::new(10, 20, 30)),
            params(&[1, 20, 2, 10, 3], Entries::new(20, 10, 30)),
            params(&[1, 10, 40, 2, -10, 3, 10, 20], Entries::new(40, -10, 30)),
            params(&[1, 10, 40, 10, -10, 3, 10, 20], Entries::new(10, 10, 20)),
        })]
        fn entries_sum_to_target(input: TestParameters) {
            let result = two_slice_entries_that_sum_to(input.entries, input.expected.target);

            verify_result_entries_are_equal(&result, &input.expected);
        }

        #[parameterized::parameterized(input = {
            params(&[10, 20], Entries::new(10, 20, 40)),
            params(&[10], Entries::new(10, 10, 20)),
        })]
        fn entries_do_not_sum_to_target(input: TestParameters) {
            let result = two_slice_entries_that_sum_to(input.entries, input.expected.target);

            verify_result_no_entries_found(result);
        }
    }
}
