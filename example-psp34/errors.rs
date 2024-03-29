
use ink::prelude::string::String;
use core::fmt;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP34Error {
    Custom(String),
    SelfApprove,
    NotApproved,
    TokenExists,
    TokenNotExists,
    SafeTransferCheckFailed(String),
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OwnableError {
    NotAnOwner
}

impl core::fmt::Display for OwnableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display_ownable_error() {
        let error = crate::errors::OwnableError::NotAnOwner;
        assert_eq!(format!("{}", error), "NotAnOwner");
    }
}