use anyhow::{Result, anyhow};
use evalexpr::*;
use crate::data::{DataPoint, Dataset};

/// A mathematical function that can be evaluated and plotted
pub struct Function {
    expression: String,
}

impl Function {
    pub fn new(expression: &str) -> Self {
        Self {
            expression: expression.trim().to_string(),
        }
    }

    /// Generate a dataset by evaluating the function over a range
    pub fn generate_dataset(&self, x_min: f64, x_max: f64, num_points: Option<usize>) -> Result<Dataset> {
        let points_count = num_points.unwrap_or(200);
        
        if points_count == 0 {
            return Err(anyhow!("Number of points must be greater than 0"));
        }

        let mut points = Vec::new();
        let step = if points_count == 1 {
            0.0
        } else {
            (x_max - x_min) / (points_count - 1) as f64
        };

        for i in 0..points_count {
            let x = x_min + i as f64 * step;
            match self.evaluate(x) {
                Ok(y) if y.is_finite() => {
                    points.push(DataPoint { x, y });
                }
                _ => {
                    // Skip points where function evaluation fails or returns infinite/NaN
                    continue;
                }
            }
        }

        if points.is_empty() {
            return Err(anyhow!("Function evaluation failed for all points in range"));
        }

        Ok(Dataset {
            points,
            x_label: "x".to_string(),
            y_label: format!("f(x) = {}", self.expression),
        })
    }

    /// Evaluate the function at a given x value
    fn evaluate(&self, x: f64) -> Result<f64> {
        // Pre-process expression to add simple function aliases
        let expression = self.add_function_aliases(&self.expression);
        
        // Use evalexpr to evaluate the expression with x variable
        let mut context = HashMapContext::<evalexpr::DefaultNumericTypes>::new();
        context.set_value("x".into(), Value::Float(x))?;
        context.set_value("pi".into(), Value::Float(std::f64::consts::PI))?;
        context.set_value("e".into(), Value::Float(std::f64::consts::E))?;
        
        match eval_with_context(&expression, &context) {
            Ok(value) => {
                // Try to convert to f64 using the evalexpr API
                if let Ok(num) = value.as_float() {
                    Ok(num)
                } else {
                    Err(anyhow!("Expression did not evaluate to a number: {:?}", value))
                }
            },
            Err(e) => Err(anyhow!("Failed to evaluate expression: {}", e)),
        }
    }

    /// Add simple function aliases (sin -> math::sin, etc.)
    fn add_function_aliases(&self, expr: &str) -> String {
        let aliases = &[
            ("sin(", "math::sin("),
            ("cos(", "math::cos("),
            ("tan(", "math::tan("),
            ("exp(", "math::exp("),
            ("ln(", "math::ln("),
            ("log(", "math::log10("),
            ("log10(", "math::log10("),
            ("log2(", "math::log2("),
            ("sqrt(", "math::sqrt("),
            ("abs(", "math::abs("),
            ("asin(", "math::asin("),
            ("acos(", "math::acos("),
            ("atan(", "math::atan("),
        ];
        
        let mut result = expr.to_string();
        for (alias, full_name) in aliases {
            result = result.replace(alias, full_name);
        }
        result
    }

}

/// Detect intelligent default range for different function types
pub fn detect_range(expression: &str) -> (f64, f64) {
    let expr = expression.to_lowercase();
    
    if expr.contains("exp") {
        (-5.0, 5.0)
    } else if expr.contains("ln") || expr.contains("log") {
        (0.1, 10.0)
    } else if expr.contains("sqrt") {
        (0.0, 10.0)
    } else if expr.contains("tan") {
        (-1.5, 1.5)
    } else if expr.contains("1/x") || expr.contains("/x") {
        (-10.0, 10.0)
    } else {
        (-10.0, 10.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let func = Function::new("x + 1");
        assert_eq!(func.evaluate(0.0).unwrap(), 1.0);
        assert_eq!(func.evaluate(1.0).unwrap(), 2.0);
        assert_eq!(func.evaluate(-1.0).unwrap(), 0.0);
    }

    #[test]
    fn test_quadratic() {
        let func = Function::new("x^2");
        assert_eq!(func.evaluate(2.0).unwrap(), 4.0);
        assert_eq!(func.evaluate(-3.0).unwrap(), 9.0);
    }

    #[test]
    fn test_constants() {
        let func = Function::new("pi");
        assert!((func.evaluate(0.0).unwrap() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_dataset_generation() {
        let func = Function::new("x^2");
        let dataset = func.generate_dataset(-2.0, 2.0, Some(5)).unwrap();
        assert_eq!(dataset.points.len(), 5);
    }

    #[test]
    fn test_range_detection() {
        assert_eq!(detect_range("sin(x)"), (-10.0, 10.0));
        assert_eq!(detect_range("exp(x)"), (-5.0, 5.0));
        assert_eq!(detect_range("ln(x)"), (0.1, 10.0));
    }
}