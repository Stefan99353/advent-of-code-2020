use std::io;
use std::str::FromStr;

use anyhow::{Error, Result};

/// Reads input as lines, where every line is parsed to given type.
pub fn input_iter<T, Input>(input: Input) -> impl Iterator<Item=Result<T>>
    where
        T: FromStr,
        T::Err: Into<Error>,
        Input: io::BufRead,
{
    input
        .lines()
        .map(|item| -> Result<T> { item?.parse().map_err(Into::into) })
}

/// Reads input as lines, where every line is parsed to given type.
pub fn input_vec<T, Input>(input: Input) -> Result<Vec<T>>
    where
        T: FromStr,
        T::Err: Into<Error>,
        Input: io::BufRead,
{
    input_iter(input).collect()
}

