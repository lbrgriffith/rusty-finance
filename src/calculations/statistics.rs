//! Statistical calculation functions

use crate::{FinanceError, FinanceResult};
use std::collections::HashMap;

/// Calculates the arithmetic mean (average) of a series of numbers
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_mean;
/// 
/// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let mean = calculate_mean(&numbers).unwrap();
/// assert_eq!(mean, 3.0);
/// ```
pub fn calculate_mean(numbers: &[f64]) -> FinanceResult<f64> {
    if numbers.is_empty() {
        return Err(FinanceError::InvalidInput("Cannot calculate mean of empty dataset".into()));
    }
    
    for (i, &num) in numbers.iter().enumerate() {
        if !num.is_finite() {
            return Err(FinanceError::InvalidInput(format!("Invalid number at index {}: {}", i, num)));
        }
    }
    
    let sum: f64 = numbers.iter().sum();
    Ok(sum / numbers.len() as f64)
}

/// Calculates the median of a series of numbers
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_median;
/// 
/// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let median = calculate_median(&numbers).unwrap();
/// assert_eq!(median, 3.0);
/// ```
pub fn calculate_median(numbers: &[f64]) -> FinanceResult<f64> {
    if numbers.is_empty() {
        return Err(FinanceError::InvalidInput("Cannot calculate median of empty dataset".into()));
    }
    
    for (i, &num) in numbers.iter().enumerate() {
        if !num.is_finite() {
            return Err(FinanceError::InvalidInput(format!("Invalid number at index {}: {}", i, num)));
        }
    }
    
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    let len = sorted.len();
    if len % 2 == 0 {
        // Even number of elements - average of middle two
        Ok((sorted[len / 2 - 1] + sorted[len / 2]) / 2.0)
    } else {
        // Odd number of elements - middle element
        Ok(sorted[len / 2])
    }
}

/// Calculates the mode of a series of numbers
/// 
/// Returns the most frequently occurring number(s). If there are multiple modes,
/// returns the smallest one.
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_mode;
/// 
/// let numbers = vec![1.0, 2.0, 2.0, 3.0, 4.0];
/// let mode = calculate_mode(&numbers).unwrap();
/// assert_eq!(mode, Some(2.0));
/// ```
pub fn calculate_mode(numbers: &[f64]) -> FinanceResult<Option<f64>> {
    if numbers.is_empty() {
        return Ok(None);
    }
    
    for (i, &num) in numbers.iter().enumerate() {
        if !num.is_finite() {
            return Err(FinanceError::InvalidInput(format!("Invalid number at index {}: {}", i, num)));
        }
    }
    
    let mut frequency_map = HashMap::new();
    
    for &number in numbers {
        // Use string representation to handle floating point comparison
        let key = format!("{:.10}", number);
        *frequency_map.entry(key).or_insert(0) += 1;
    }
    
    if let Some(max_frequency) = frequency_map.values().max() {
        if *max_frequency == 1 {
            // No mode (all numbers appear once)
            return Ok(None);
        }
        
        // Find all numbers with maximum frequency and return the smallest
        let modes: Result<Vec<f64>, _> = frequency_map
            .iter()
            .filter(|(_, &freq)| freq == *max_frequency)
            .map(|(key, _)| key.parse::<f64>())
            .collect();
        
        match modes {
            Ok(mode_values) => {
                Ok(mode_values.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)))
            }
            Err(_) => Err(FinanceError::InvalidInput("Failed to parse mode values".into()))
        }
    } else {
        Ok(None)
    }
}

/// Calculates the variance of a series of numbers (population variance)
/// 
/// Formula: σ² = Σ(x - μ)² / N
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_variance;
/// 
/// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let variance = calculate_variance(&numbers).unwrap();
/// assert_eq!(variance, 2.0);
/// ```
pub fn calculate_variance(numbers: &[f64]) -> FinanceResult<f64> {
    if numbers.len() < 2 {
        return Err(FinanceError::InvalidInput("At least two numbers are required to calculate variance".into()));
    }
    
    let mean = calculate_mean(numbers)?;
    
    let sum_squared_diff: f64 = numbers.iter()
        .map(|&x| (x - mean).powi(2))
        .sum();
    
    Ok(sum_squared_diff / numbers.len() as f64)
}

/// Calculates the sample variance of a series of numbers
/// 
/// Formula: s² = Σ(x - x̄)² / (n - 1)
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_sample_variance;
/// 
/// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let variance = calculate_sample_variance(&numbers).unwrap();
/// assert_eq!(variance, 2.5);
/// ```
pub fn calculate_sample_variance(numbers: &[f64]) -> FinanceResult<f64> {
    if numbers.len() < 2 {
        return Err(FinanceError::InvalidInput("At least two numbers are required to calculate sample variance".into()));
    }
    
    let mean = calculate_mean(numbers)?;
    
    let sum_squared_diff: f64 = numbers.iter()
        .map(|&x| (x - mean).powi(2))
        .sum();
    
    Ok(sum_squared_diff / (numbers.len() - 1) as f64)
}

/// Calculates the standard deviation of a series of numbers (population)
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_standard_deviation;
/// 
/// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let std_dev = calculate_standard_deviation(&numbers).unwrap();
/// assert!((std_dev - 1.4142135623730951).abs() < 1e-10);
/// ```
pub fn calculate_standard_deviation(numbers: &[f64]) -> FinanceResult<f64> {
    let variance = calculate_variance(numbers)?;
    Ok(variance.sqrt())
}

/// Calculates the sample standard deviation of a series of numbers
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_sample_standard_deviation;
/// 
/// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let std_dev = calculate_sample_standard_deviation(&numbers).unwrap();
/// assert!((std_dev - 1.5811388300841898).abs() < 1e-10);
/// ```
pub fn calculate_sample_standard_deviation(numbers: &[f64]) -> FinanceResult<f64> {
    let variance = calculate_sample_variance(numbers)?;
    Ok(variance.sqrt())
}

/// Calculates simple probability (successes / trials)
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_probability;
/// 
/// let prob = calculate_probability(3, 5).unwrap();
/// assert_eq!(prob, 0.6);
/// ```
pub fn calculate_probability(successes: u32, trials: u32) -> FinanceResult<f64> {
    if trials == 0 {
        return Err(FinanceError::DivisionByZero);
    }
    
    if successes > trials {
        return Err(FinanceError::InvalidInput("Successes cannot exceed trials".into()));
    }
    
    Ok(successes as f64 / trials as f64)
}

/// Calculates weighted average of a series of numbers
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_weighted_average;
/// 
/// let numbers = vec![10.0, 20.0, 30.0];
/// let weights = vec![0.2, 0.3, 0.5];
/// let weighted_avg = calculate_weighted_average(&numbers, &weights).unwrap();
/// assert_eq!(weighted_avg, 23.0);
/// ```
pub fn calculate_weighted_average(numbers: &[f64], weights: &[f64]) -> FinanceResult<f64> {
    if numbers.is_empty() || weights.is_empty() {
        return Err(FinanceError::InvalidInput("Numbers and weights cannot be empty".into()));
    }
    
    if numbers.len() != weights.len() {
        return Err(FinanceError::InvalidInput("Numbers and weights must have the same length".into()));
    }
    
    let mut sum = 0.0;
    let mut total_weight = 0.0;
    
    for (i, (&number, &weight)) in numbers.iter().zip(weights.iter()).enumerate() {
        if !number.is_finite() {
            return Err(FinanceError::InvalidInput(format!("Invalid number at index {}: {}", i, number)));
        }
        if !weight.is_finite() || weight < 0.0 {
            return Err(FinanceError::InvalidInput(format!("Invalid weight at index {}: {}", i, weight)));
        }
        
        sum += number * weight;
        total_weight += weight;
    }
    
    if total_weight == 0.0 {
        return Err(FinanceError::DivisionByZero);
    }
    
    Ok(sum / total_weight)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_mean() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = calculate_mean(&numbers).unwrap();
        assert_eq!(mean, 3.0);
    }

    #[test]
    fn test_calculate_mean_empty() {
        let numbers = vec![];
        assert!(calculate_mean(&numbers).is_err());
    }

    #[test]
    fn test_calculate_mean_invalid_number() {
        let numbers = vec![1.0, f64::NAN, 3.0];
        assert!(calculate_mean(&numbers).is_err());
    }

    #[test]
    fn test_calculate_median_odd() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let median = calculate_median(&numbers).unwrap();
        assert_eq!(median, 3.0);
    }

    #[test]
    fn test_calculate_median_even() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0];
        let median = calculate_median(&numbers).unwrap();
        assert_eq!(median, 2.5);
    }

    #[test]
    fn test_calculate_mode_single() {
        let numbers = vec![1.0, 2.0, 2.0, 3.0, 4.0];
        let mode = calculate_mode(&numbers).unwrap();
        assert_eq!(mode, Some(2.0));
    }

    #[test]
    fn test_calculate_mode_none() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0];
        let mode = calculate_mode(&numbers).unwrap();
        assert_eq!(mode, None);
    }

    #[test]
    fn test_calculate_variance() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let variance = calculate_variance(&numbers).unwrap();
        assert_eq!(variance, 2.0);
    }

    #[test]
    fn test_calculate_sample_variance() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let variance = calculate_sample_variance(&numbers).unwrap();
        assert_eq!(variance, 2.5);
    }

    #[test]
    fn test_calculate_standard_deviation() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let std_dev = calculate_standard_deviation(&numbers).unwrap();
        assert!((std_dev - 1.4142135623730951).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_probability() {
        let prob = calculate_probability(3, 5).unwrap();
        assert_eq!(prob, 0.6);
    }

    #[test]
    fn test_calculate_probability_invalid() {
        assert!(calculate_probability(6, 5).is_err());
        assert!(calculate_probability(1, 0).is_err());
    }

    #[test]
    fn test_calculate_weighted_average() {
        let numbers = vec![10.0, 20.0, 30.0];
        let weights = vec![0.2, 0.3, 0.5];
        let weighted_avg = calculate_weighted_average(&numbers, &weights).unwrap();
        assert_eq!(weighted_avg, 23.0);
    }

    #[test]
    fn test_calculate_weighted_average_mismatched_lengths() {
        let numbers = vec![10.0, 20.0];
        let weights = vec![0.2, 0.3, 0.5];
        assert!(calculate_weighted_average(&numbers, &weights).is_err());
    }

    #[test]
    fn test_calculate_weighted_average_zero_weights() {
        let numbers = vec![10.0, 20.0, 30.0];
        let weights = vec![0.0, 0.0, 0.0];
        assert!(calculate_weighted_average(&numbers, &weights).is_err());
    }
}