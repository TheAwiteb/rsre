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

#![feature(try_trait_v2, try_trait_v2_residual, const_trait_impl)]

mod actions;
mod errors;
mod parser;
mod utils;
use std::{env, ffi::OsStr};

use errors::{Error as RsreError, Statuses};

fn main() -> Statuses<(), RsreError> {
    let action = parser::parse(
        env::args_os()
            .skip(1)
            .into_iter()
            .collect::<Vec<_>>()
            .join(OsStr::new(" ")),
    )?;
    action.run().into()
}
