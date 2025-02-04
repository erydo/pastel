pub use std::io::Write;

pub use crate::config::Config;
pub use crate::error::{PastelError, Result};

pub use clap::ArgMatches;

pub use super::io::*;
pub use super::traits::*;

pub use pastel::ansi::{AnsiColor, Brush, ToAnsiStyle};
pub use pastel::Color;
