//! Form input validation functions.
//!
//! This module provides validation for user input in forms,
//! returning descriptive errors instead of silently using defaults.

use crate::error::AppError;

/// Parse and validate partition count input.
///
/// Returns an error if the input is not a valid positive integer.
pub fn parse_partitions(input: &str) -> Result<i32, AppError> {
    let value: i32 = input.trim().parse().map_err(|_| AppError::Validation {
        field: "partitions".into(),
        message: format!("'{}' is not a valid number", input),
    })?;

    if value < 1 {
        return Err(AppError::Validation {
            field: "partitions".into(),
            message: "Partition count must be at least 1".into(),
        });
    }

    Ok(value)
}

/// Parse and validate replication factor input.
///
/// Returns an error if the input is not a valid positive integer.
pub fn parse_replication_factor(input: &str) -> Result<i32, AppError> {
    let value: i32 = input.trim().parse().map_err(|_| AppError::Validation {
        field: "replication_factor".into(),
        message: format!("'{}' is not a valid number", input),
    })?;

    if value < 1 {
        return Err(AppError::Validation {
            field: "replication_factor".into(),
            message: "Replication factor must be at least 1".into(),
        });
    }

    Ok(value)
}

/// Parse and validate new partition count for adding partitions.
///
/// Returns an error if the new count is not greater than the current count.
pub fn parse_new_partition_count(input: &str, current: i32) -> Result<i32, AppError> {
    let value: i32 = input.trim().parse().map_err(|_| AppError::Validation {
        field: "new_count".into(),
        message: format!("'{}' is not a valid number", input),
    })?;

    if value <= current {
        return Err(AppError::Validation {
            field: "new_count".into(),
            message: format!("New count must be greater than current ({})", current),
        });
    }

    Ok(value)
}

/// Parse and validate offset input for purging.
///
/// Returns an error if the input is not a valid non-negative integer.
pub fn parse_offset(input: &str) -> Result<i64, AppError> {
    let value: i64 = input.trim().parse().map_err(|_| AppError::Validation {
        field: "offset".into(),
        message: format!("'{}' is not a valid offset", input),
    })?;

    if value < 0 {
        return Err(AppError::Validation {
            field: "offset".into(),
            message: "Offset must be non-negative".into(),
        });
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_partitions_valid() {
        assert_eq!(parse_partitions("3").unwrap(), 3);
        assert_eq!(parse_partitions(" 10 ").unwrap(), 10);
        assert_eq!(parse_partitions("1").unwrap(), 1);
    }

    #[test]
    fn test_parse_partitions_invalid() {
        assert!(parse_partitions("").is_err());
        assert!(parse_partitions("abc").is_err());
        assert!(parse_partitions("0").is_err());
        assert!(parse_partitions("-1").is_err());
    }

    #[test]
    fn test_parse_replication_factor_valid() {
        assert_eq!(parse_replication_factor("1").unwrap(), 1);
        assert_eq!(parse_replication_factor("3").unwrap(), 3);
    }

    #[test]
    fn test_parse_replication_factor_invalid() {
        assert!(parse_replication_factor("0").is_err());
        assert!(parse_replication_factor("abc").is_err());
    }

    #[test]
    fn test_parse_new_partition_count_valid() {
        assert_eq!(parse_new_partition_count("5", 3).unwrap(), 5);
        assert_eq!(parse_new_partition_count("10", 1).unwrap(), 10);
    }

    #[test]
    fn test_parse_new_partition_count_invalid() {
        assert!(parse_new_partition_count("3", 3).is_err()); // equal
        assert!(parse_new_partition_count("2", 5).is_err()); // less than
        assert!(parse_new_partition_count("abc", 3).is_err());
    }

    #[test]
    fn test_parse_offset_valid() {
        assert_eq!(parse_offset("0").unwrap(), 0);
        assert_eq!(parse_offset("100").unwrap(), 100);
        assert_eq!(parse_offset(" 50 ").unwrap(), 50);
    }

    #[test]
    fn test_parse_offset_invalid() {
        assert!(parse_offset("-1").is_err());
        assert!(parse_offset("abc").is_err());
    }
}
