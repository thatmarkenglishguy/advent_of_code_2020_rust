use argh::FromArgs;
use std::ops::Deref;

#[derive(FromArgs, PartialEq, Debug)]
/// Day 1 of Advent of Code. <link>https://adventofcode.com/2020/day/1</link>
pub struct Parameters {
    #[argh(
        option,
        default = "std::path::PathBuf::from(\"./day1.txt\")",
        long = "input"
    )]
    /// path to read day 1 input from.
    pub input_path: std::path::PathBuf,

    #[argh(option, default = "2020")]
    /// target to reach from inputs
    pub target: i64,
}

impl Parameters {
    pub fn from_command_line() -> Parameters {
        argh::from_env()
    }

    pub fn input_path(self: &Self) -> &std::path::Path {
        self.input_path.deref()
    }
}

#[cfg(test)]
mod tests {
    use crate::day01::command_line::parameters::Parameters;
    use argh::FromArgs;
    use std::path::PathBuf;

    const EXPECTED_DEFAULT_INPUT_PATH_STRING: &'static str = "./day1.txt";
    const EXPECTED_DEFAULT_TARGET: i64 = 2020i64;

    fn expected_default_input_path() -> PathBuf {
        std::path::PathBuf::from(EXPECTED_DEFAULT_INPUT_PATH_STRING)
    }

    #[test]
    fn omitting_parameters_gets_default() -> Result<(), std::io::Error> {
        let parameters = Parameters::from_args(&["day01"], &[]).expect("failed parameters default");

        assert_eq!(
            parameters,
            Parameters {
                input_path: expected_default_input_path(),
                target: EXPECTED_DEFAULT_TARGET
            }
        );

        Ok(())
    }

    #[test]
    fn specifying_input_path_works() -> Result<(), std::io::Error> {
        let expected_input_path = "test_input";
        let parameters = Parameters::from_args(&["day01"], &["--input", expected_input_path])
            .expect("failed parameters with input path");

        assert_eq!(
            parameters,
            Parameters {
                input_path: std::path::PathBuf::from(expected_input_path),
                target: EXPECTED_DEFAULT_TARGET
            }
        );

        Ok(())
    }

    #[test]
    fn specifying_target_works() -> Result<(), std::io::Error> {
        let expected_target = 1920i64;
        let parameters =
            Parameters::from_args(&["day01"], &["--target", &format!("{}", expected_target)])
                .expect("failed parameters with target");

        assert_eq!(
            parameters,
            Parameters {
                input_path: expected_default_input_path(),
                target: expected_target
            }
        );

        Ok(())
    }
}
