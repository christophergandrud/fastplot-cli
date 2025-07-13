use anyhow::{Result, anyhow};
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
        // Replace 'x' with the value, being careful with word boundaries
        let expr = self.replace_variable(&self.expression, "x", x);
        self.evaluate_expression(&expr)
    }

    /// Replace variable with value, handling word boundaries properly
    fn replace_variable(&self, expr: &str, var: &str, value: f64) -> String {
        let mut result = String::new();
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if i <= chars.len() - var.len() {
                let substr: String = chars[i..i + var.len()].iter().collect();
                if substr == var {
                    // Check if this is a complete variable (not part of a larger identifier)
                    let prev_ok = i == 0 || !chars[i - 1].is_alphanumeric();
                    let next_ok = i + var.len() >= chars.len() || !chars[i + var.len()].is_alphanumeric();
                    
                    if prev_ok && next_ok {
                        // Add parentheses around all numbers to avoid parsing issues
                        result.push_str(&format!("({})", value));
                        i += var.len();
                        continue;
                    }
                }
            }
            result.push(chars[i]);
            i += 1;
        }
        
        result
    }

    /// Basic expression evaluator supporting common math functions
    fn evaluate_expression(&self, expr: &str) -> Result<f64> {
        let expr = expr.replace(" ", "");
        
        // Handle parentheses recursively
        if let Some(result) = self.handle_parentheses(&expr)? {
            return Ok(result);
        }
        
        // Handle basic operations with precedence
        if let Some(result) = self.handle_addition_subtraction(&expr)? {
            return Ok(result);
        }
        
        if let Some(result) = self.handle_multiplication_division(&expr)? {
            return Ok(result);
        }
        
        if let Some(result) = self.handle_exponentiation(&expr)? {
            return Ok(result);
        }
        
        // Handle functions
        if let Some(result) = self.handle_functions(&expr)? {
            return Ok(result);
        }
        
        // Handle unary minus
        if expr.starts_with('-') && expr.len() > 1 {
            let inner = &expr[1..];
            return Ok(-self.evaluate_expression(inner)?);
        }
        
        // Handle constants and numbers
        self.parse_number_or_constant(&expr)
    }

    fn handle_parentheses(&self, expr: &str) -> Result<Option<f64>> {
        if let Some((start, end)) = find_innermost_parentheses(expr) {
            let before = &expr[..start];
            let inside = &expr[start + 1..end];
            let after = &expr[end + 1..];
            
            let inner_result = self.evaluate_expression(inside)?;
            let new_expr = format!("{}{}{}", before, inner_result, after);
            return Ok(Some(self.evaluate_expression(&new_expr)?));
        }
        Ok(None)
    }

    fn handle_addition_subtraction(&self, expr: &str) -> Result<Option<f64>> {
        if let Some((left, op, right)) = find_operator_outside_parens(expr, &['+', '-']) {
            let left_val = self.evaluate_expression(left)?;
            let right_val = self.evaluate_expression(right)?;
            return Ok(Some(match op {
                '+' => left_val + right_val,
                '-' => left_val - right_val,
                _ => unreachable!(),
            }));
        }
        Ok(None)
    }

    fn handle_multiplication_division(&self, expr: &str) -> Result<Option<f64>> {
        if let Some((left, op, right)) = find_operator_outside_parens(expr, &['*', '/']) {
            let left_val = self.evaluate_expression(left)?;
            let right_val = self.evaluate_expression(right)?;
            return Ok(Some(match op {
                '*' => left_val * right_val,
                '/' => {
                    if right_val == 0.0 {
                        f64::INFINITY
                    } else {
                        left_val / right_val
                    }
                }
                _ => unreachable!(),
            }));
        }
        Ok(None)
    }

    fn handle_exponentiation(&self, expr: &str) -> Result<Option<f64>> {
        if let Some((left, _, right)) = find_operator_outside_parens(expr, &['^']) {
            let left_val = self.evaluate_expression(left)?;
            let right_val = self.evaluate_expression(right)?;
            return Ok(Some(left_val.powf(right_val)));
        }
        Ok(None)
    }

    fn handle_functions(&self, expr: &str) -> Result<Option<f64>> {
        let functions: &[(&str, fn(f64) -> f64)] = &[
            ("sin", f64::sin),
            ("cos", f64::cos),
            ("tan", f64::tan),
            ("exp", f64::exp),
            ("ln", f64::ln),
            ("log", f64::log10),
            ("sqrt", f64::sqrt),
            ("abs", f64::abs),
        ];
        
        for (func_name, func) in functions {
            if expr.starts_with(func_name) && expr.len() > func_name.len() {
                let rest = &expr[func_name.len()..];
                if rest.starts_with('(') && rest.ends_with(')') {
                    let inner = &rest[1..rest.len() - 1];
                    let arg = self.evaluate_expression(inner)?;
                    return Ok(Some(func(arg)));
                }
            }
        }
        
        Ok(None)
    }

    fn parse_number_or_constant(&self, expr: &str) -> Result<f64> {
        match expr {
            "pi" | "PI" => Ok(std::f64::consts::PI),
            "e" | "E" => Ok(std::f64::consts::E),
            _ => expr.parse::<f64>().map_err(|_| anyhow!("Invalid expression: {}", expr)),
        }
    }
}

/// Find innermost parentheses in an expression
fn find_innermost_parentheses(expr: &str) -> Option<(usize, usize)> {
    let mut deepest_start = None;
    let mut level = 0;
    let mut max_level = 0;
    
    for (i, ch) in expr.char_indices() {
        match ch {
            '(' => {
                level += 1;
                if level > max_level {
                    max_level = level;
                    deepest_start = Some(i);
                }
            }
            ')' => {
                if level == max_level && deepest_start.is_some() {
                    return Some((deepest_start.unwrap(), i));
                }
                level -= 1;
            }
            _ => {}
        }
    }
    
    None
}

/// Find operator outside parentheses, searching right to left for correct precedence
fn find_operator_outside_parens<'a>(expr: &'a str, operators: &[char]) -> Option<(&'a str, char, &'a str)> {
    let mut level = 0;
    
    for (i, ch) in expr.char_indices().rev() {
        match ch {
            ')' => level += 1,
            '(' => level -= 1,
            _ if level == 0 && operators.contains(&ch) => {
                let left = &expr[..i];
                let right = &expr[i + 1..];
                if !left.is_empty() && !right.is_empty() {
                    return Some((left, ch, right));
                }
            }
            _ => {}
        }
    }
    
    None
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