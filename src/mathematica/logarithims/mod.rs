use super::errors::LogarithimError;

pub struct Log {
    arg: f64,
    base: f64
}

impl Log {
    pub fn new(arg: f64, base: f64) -> Self {
        Self { arg, base }
    }
}

impl Log {
    pub fn evaluate(&self) -> Result<f64, LogarithimError> {
        if self.arg < 0.0 {
            return Err(LogarithimError::InvalidArgument(self.arg));
        }

        if self.base <= 1.0 {
            return Err(LogarithimError::InvalidBase(self.base.signum()));
        }

        Ok(self.arg.log(self.base))
    }
}

impl std::ops::Add for Log {
    type Output = Result<Self, LogarithimError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.base != rhs.base {
            return Err(LogarithimError::DifferentBase(self.base, rhs.base));
        }

        Ok(Self { arg: self.arg * rhs.arg, base: self.base })
    }
}

impl std::ops::Sub for Log {
    type Output = Result<Self, LogarithimError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.base != rhs.base {
            return Err(LogarithimError::DifferentBase(self.base, rhs.base));
        }

        Ok(Self { arg: self.arg / rhs.arg, base: self.base })
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "log{}({})", self.arg, self.base)
    }
}