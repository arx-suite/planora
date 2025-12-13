#![allow(dead_code, unused_imports)]

mod error;
mod service;

pub use error::Error as BucketError;
pub use service::S3Service;
