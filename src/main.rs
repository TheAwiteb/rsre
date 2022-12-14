// Copyright 2022 Awiteb <https://github.com/TheAwiteb>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(try_trait_v2, try_trait_v2_residual)]

mod actions;
mod errors;
mod macros;
mod parser;
mod utils;

use debug as de;
use errors::{Error as RsreError, Statuses};
use std::{env, ffi::OsString};

fn main() -> Statuses<(), RsreError> {
    let args = env::args_os()
        .map(OsString::into_string)
        .skip(1)
        .collect::<Result<Vec<String>, OsString>>()
        .map_err(|err| {
            RsreError::FileSystem(err.into_string().expect("Cannot convert error to String"))
        })?;
    de! {println!("Args: {:?}", args)};
    let action = parser::parse(args)?;
    de! {println!("Action: {:?}", action)};
    action.run().into()
}
