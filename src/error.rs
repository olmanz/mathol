use std::fmt::{Display, Formatter, Error};
use std::error;

#[derive(Debug, Clone, PartialEq)]
pub enum MatholError {
    NegativeValueCause(NegativeValueError),
    EmptyVecCause(EmptyVectorError),
    SummationCause(SummationError),
    OutgrowCause(OutgrowError),
    RangeCause(RangeError),
    ComparisonCause(ComparisonError),
    ContainsZeroCause(ContainsZeroError),
    VectorCause(VectorError),
    MatriceCause(MatriceError),
    OutOfBoundsCause(OutOfBoundsError),
    LengthCause(LengthError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NegativeValueError {
    pub message: String,
}

impl Display for NegativeValueError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for NegativeValueError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyVectorError {
    pub message: String,
}

impl Display for EmptyVectorError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for EmptyVectorError {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct SummationError {
    pub message: String
}

impl Display for SummationError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for SummationError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutgrowError {
    pub message: String
}

impl Display for OutgrowError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for OutgrowError {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct RangeError {
    pub message: String
}

impl Display for RangeError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for RangeError {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonError {
    pub message: String
}

impl Display for ComparisonError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ComparisonError {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContainsZeroError {
    pub message: String
}

impl Display for ContainsZeroError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ContainsZeroError {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct VectorError {
    pub message: String,
}

impl Display for VectorError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for VectorError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatriceError {
    pub message: String,
}

impl Display for MatriceError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for MatriceError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutOfBoundsError {
    pub message: String,
}

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for OutOfBoundsError {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct LengthError {
    pub message: String,
}

impl Display for LengthError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for LengthError {
    fn description(&self) -> &str {
        &self.message
    }
}

