use super::*;

impl<'i, T> Try for ParseResult<'i, T> {
    type Output = Parsed<'i, T>;
    type Residual = ParseResult<'i, Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Self::Pending(output.0, output.1)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Pending(state, value) => ControlFlow::Continue((state, value)),
            Self::Stop(e) => ControlFlow::Break(ParseResult::Stop(e)),
        }
    }
}

impl<'i, T> FromResidual for ParseResult<'i, T> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        match residual {
            ParseResult::Pending(_, _) => unreachable!(),
            ParseResult::Stop(e) => Self::Stop(e),
        }
    }
}

impl<'i, T, E> FromResidual<Result<Infallible, E>> for ParseResult<'i, T>
where
    E: Into<StopBecause>,
{
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => Self::Stop(e.into()),
        }
    }
}

impl<'i, T, E> FromResidual<ParseResult<'i, Infallible>> for Result<T, E>
where
    E: From<StopBecause>,
{
    fn from_residual(residual: ParseResult<'i, Infallible>) -> Self {
        match residual {
            ParseResult::Pending(_, _) => unreachable!(),
            ParseResult::Stop(e) => Err(E::from(e)),
        }
    }
}

impl<'i, T> FromResidual<Option<Infallible>> for ParseResult<'i, T> {
    fn from_residual(residual: Option<Infallible>) -> Self {
        match residual {
            Some(_) => unreachable!(),
            None => Self::Stop(StopBecause::Uninitialized),
        }
    }
}
