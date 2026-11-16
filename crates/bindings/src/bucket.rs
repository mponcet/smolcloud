use std::collections::HashMap;
use std::future::Future;
use std::str::FromStr;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("bucket error")]
pub struct BucketError {
    pub message: String,
    #[source]
    pub source: anyhow::Error,
}

pub enum BucketRange {
    Start {
        start: u64,
    },
    /// Range 0-499 will request 500 bytes
    RangeInclusive {
        start: u64,
        end: u64,
    },
    Suffix {
        suffix: u64,
    },
}

impl FromStr for BucketRange {
    type Err = String;

    /// Convert an HTTP range header to a [`BucketRange`]. Multiple ranges are not supported.
    /// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Range#syntax>
    ///
    /// # Examples
    ///
    /// ```
    /// use bindings::BucketRange;
    /// use std::str::FromStr;
    ///
    /// let range = "bytes=0-499".parse::<BucketRange>().unwrap();
    /// assert!(matches!(range, BucketRange::RangeInclusive { start: 0, end: 499 }));
    ///
    /// let range = "bytes=500-".parse::<BucketRange>().unwrap();
    /// assert!(matches!(range, BucketRange::Start { start: 500 }));
    ///
    /// let range = "bytes=-500".parse::<BucketRange>().unwrap();
    /// assert!(matches!(range, BucketRange::Suffix { suffix: 500 }));
    /// ```
    fn from_str(range: &str) -> Result<Self, Self::Err> {
        fn parse_part(s: &str) -> Option<u64> {
            if s.is_empty() { None } else { s.parse().ok() }
        }

        let (start, end) = range
            .strip_prefix("bytes=")
            .and_then(|range| range.split_once('-'))
            .map(|(start, end)| (parse_part(start), parse_part(end)))
            .ok_or(format!("failed to parse range: {range}"))?;

        match (start, end) {
            (Some(start), Some(end)) if end >= start => {
                Ok(BucketRange::RangeInclusive { start, end })
            }
            (Some(start), None) => Ok(BucketRange::Start { start }),
            (None, Some(suffix)) => Ok(BucketRange::Suffix { suffix }),
            _ => Err(format!("failed to parse range: {range}")),
        }
    }
}

pub trait BucketGetOptionsBuilder {
    fn range(self, range: BucketRange) -> Self;
    fn execute(self) -> impl Future<Output = Result<Option<BucketObject>, BucketError>> + Send;
}

pub trait BucketPutOptionsBuilder {
    fn custom_metadata(self, metadata: HashMap<String, String>) -> Self;
    fn execute(self) -> impl Future<Output = Result<Option<BucketObject>, BucketError>> + Send;
}

pub trait BucketListOptionsBuilder {
    fn limit(self, limit: u32) -> Self;
    fn prefix(self, prefix: &str) -> Self;
    fn include_custom_metadata(self) -> Self;
    fn execute(self) -> impl Future<Output = Result<Vec<BucketObject>, BucketError>> + Send;
}

pub struct BucketObject {
    pub key: String,
    pub body: Option<Vec<u8>>,
    pub custom_metadata: Option<HashMap<String, String>>,
}

impl BucketObject {
    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }
}

pub trait Bucket: Send + Sync {
    fn head(
        &self,
        key: &str,
    ) -> impl Future<Output = Result<Option<BucketObject>, BucketError>> + Send;
    fn get(&self, key: &str) -> impl BucketGetOptionsBuilder;
    fn put(&self, key: &str, data: &[u8]) -> impl BucketPutOptionsBuilder;
    fn delete(&self, key: &str) -> impl Future<Output = Result<(), BucketError>> + Send;
    fn list(&self) -> impl BucketListOptionsBuilder;
}

// impl<B: Bucket + ?Sized> Bucket for &B {
//     fn head(
//         &self,
//         key: &str,
//     ) -> impl Future<Output = Result<Option<BucketObject>, BucketError>> + Send {
//         (*self).head(key)
//     }
//
//     fn get(&self, key: &str) -> impl BucketGetOptionsBuilder {
//         (*self).get(key)
//     }
//
//     fn put(&self, key: &str, data: &[u8]) -> impl BucketPutOptionsBuilder {
//         (*self).put(key, data)
//     }
//
//     fn delete(&self, key: &str) -> impl Future<Output = Result<(), BucketError>> + Send {
//         (*self).delete(key)
//     }
//
//     fn list(&self) -> impl BucketListOptionsBuilder {
//         (*self).list()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_range_to_bucket_range() {
        assert!("bytes=0-1024".parse::<BucketRange>().is_ok());
        assert!("bytes=-1024".parse::<BucketRange>().is_ok());
        assert!("bytes=1024-".parse::<BucketRange>().is_ok());
        assert!("bytes=-".parse::<BucketRange>().is_err());
        assert!("bytes=1024-0".parse::<BucketRange>().is_err());
        assert!("bytes=0-0".parse::<BucketRange>().is_ok());
        assert!("".parse::<BucketRange>().is_err());
    }
}
