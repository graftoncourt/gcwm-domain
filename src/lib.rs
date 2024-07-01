/*!
The core domain model library for Grafton Court Wealth Management

# Useage
This crate is only available via the organisations private repository.
In order to gain access you require a App Key and Private Key available
from mark.ashworth@gcwm.co.uk

```toml
[dependencies]
gcwm-domain = {git="git@github.com:graftoncourt/gcwm-domain.git"}
```
*/

mod simple_types;
mod tests;

pub mod contexts {
    //! Bounded contexts within the Grafton Court domain
    //! 
    //! This module contains all the bounded contexts available:
    //! - Annual Review
    
    /// Module for the Annual Review bounded context
    pub mod annual_review;
}
