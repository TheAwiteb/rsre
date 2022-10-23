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

use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Try},
    process::{ExitCode, Termination},
};

pub enum Statuses<T, E: Termination> {
    Success(T),
    Failure(E),
}

impl<T, E: Termination> Termination for Statuses<T, E> {
    fn report(self) -> ExitCode {
        if let Self::Failure(err) = self {
            err.report()
        } else {
            ExitCode::SUCCESS
        }
    }
}

impl<T, E: Termination> From<Result<T, E>> for Statuses<T, E> {
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(v) => Statuses::Success(v),
            Err(e) => Statuses::Failure(e),
        }
    }
}

impl<T, E: Termination> FromResidual<Statuses<T, E>> for Statuses<T, E> {
    fn from_residual(residual: Statuses<T, E>) -> Self {
        match residual {
            Statuses::Success(v) => Self::Success(v),
            Statuses::Failure(err) => Self::Failure(err),
        }
    }
}

impl<T, E: Termination> FromResidual<Result<T, E>> for Statuses<T, E> {
    fn from_residual(residual: Result<T, E>) -> Self {
        match residual {
            Result::Ok(v) => Self::Success(v),
            Result::Err(err) => Self::Failure(err),
        }
    }
}

impl<E: Termination> FromResidual<Result<Infallible, E>> for Statuses<(), E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Result::Ok(_) => Self::Success(()),
            Result::Err(err) => Self::Failure(err),
        }
    }
}

impl<T> FromResidual<Option<T>> for Statuses<T, ExitCode> {
    fn from_residual(residual: Option<T>) -> Self {
        match residual {
            Some(v) => Self::Success(v),
            _ => Self::Failure(ExitCode::FAILURE),
        }
    }
}

impl<T, E: Termination> Try for Statuses<T, E> {
    type Output = T;
    type Residual = Statuses<T, E>;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Success(v) => ControlFlow::Continue(v),
            Self::Failure(err) => ControlFlow::Break(Statuses::Failure(err)),
        }
    }
}
