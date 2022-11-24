#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::integer_division)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::string_to_string)]
#![deny(clippy::str_to_string)]
#![deny(clippy::try_err)]
#![deny(clippy::panic)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
// TODO: I don't understand these two linters. They require #[must_use] added to functions.
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]

mod constants;
mod iterators;
pub mod polygon_matcher;
pub mod shapes;
mod traits;
mod util;