use std::fmt::{Display, Formatter, Error};
use std::error;

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
pub struct PermutationError {
    pub message: String,
}

impl Display for PermutationError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for PermutationError {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct CombinationError {
    pub message: String,
}

impl Display for CombinationError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for CombinationError {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct VariationError {
    pub message: String,
}

impl Display for VariationError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for VariationError {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DistributionError {
    pub message: String,
}

impl Display for DistributionError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for DistributionError {
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
pub struct RowError {
    pub message: String,
    pub columns: usize,
}

impl Display for RowError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}: {}", self.message, self.columns)
    }
}

impl error::Error for RowError {
    fn description(&self) -> &str {
        &self.message
    }
}