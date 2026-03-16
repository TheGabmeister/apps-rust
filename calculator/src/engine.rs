#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn symbol(self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "−",
            Operator::Multiply => "×",
            Operator::Divide => "÷",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum InputPhase {
    FirstOperand,
    OperatorPressed,
    SecondOperand,
    ResultDisplayed,
}

const MAX_DISPLAY_LEN: usize = 16;

pub struct CalcEngine {
    display: String,
    accumulator: Option<f64>,
    pending_op: Option<Operator>,
    phase: InputPhase,
    error: bool,
}

impl CalcEngine {
    pub fn new() -> Self {
        Self {
            display: "0".into(),
            accumulator: None,
            pending_op: None,
            phase: InputPhase::FirstOperand,
            error: false,
        }
    }

    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn expression_display(&self) -> String {
        match (&self.accumulator, &self.pending_op) {
            (Some(acc), Some(op)) if self.phase != InputPhase::ResultDisplayed => {
                format!("{} {}", Self::format_result(*acc), op.symbol())
            }
            _ => String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        self.error
    }

    pub fn input_digit(&mut self, digit: char) {
        if self.error || !digit.is_ascii_digit() {
            return;
        }

        match self.phase {
            InputPhase::OperatorPressed => {
                self.display = if digit == '0' {
                    "0".into()
                } else {
                    digit.to_string()
                };
                self.phase = InputPhase::SecondOperand;
            }
            InputPhase::ResultDisplayed => {
                self.accumulator = None;
                self.pending_op = None;
                self.display = if digit == '0' {
                    "0".into()
                } else {
                    digit.to_string()
                };
                self.phase = InputPhase::FirstOperand;
            }
            _ => {
                if self.display.len() >= MAX_DISPLAY_LEN {
                    return;
                }
                if self.display == "0" || self.display == "-0" {
                    if digit == '0' {
                        return;
                    }
                    let negative = self.display.starts_with('-');
                    self.display = if negative {
                        format!("-{}", digit)
                    } else {
                        digit.to_string()
                    };
                } else {
                    self.display.push(digit);
                }
            }
        }
    }

    pub fn input_decimal(&mut self) {
        if self.error {
            return;
        }

        match self.phase {
            InputPhase::OperatorPressed => {
                self.display = "0.".into();
                self.phase = InputPhase::SecondOperand;
            }
            InputPhase::ResultDisplayed => {
                self.accumulator = None;
                self.pending_op = None;
                self.display = "0.".into();
                self.phase = InputPhase::FirstOperand;
            }
            _ => {
                if !self.display.contains('.') && self.display.len() < MAX_DISPLAY_LEN {
                    self.display.push('.');
                }
            }
        }
    }

    pub fn input_operator(&mut self, op: Operator) {
        if self.error {
            return;
        }

        match self.phase {
            InputPhase::OperatorPressed => {
                // Replace pending operator
                self.pending_op = Some(op);
            }
            InputPhase::SecondOperand => {
                // Chain: evaluate pending operation first
                self.evaluate_pending();
                if self.error {
                    return;
                }
                self.accumulator = Some(self.display_value());
                self.pending_op = Some(op);
                self.phase = InputPhase::OperatorPressed;
            }
            _ => {
                self.accumulator = Some(self.display_value());
                self.pending_op = Some(op);
                self.phase = InputPhase::OperatorPressed;
            }
        }
    }

    pub fn input_equals(&mut self) {
        if self.error {
            return;
        }

        if self.phase == InputPhase::SecondOperand || self.phase == InputPhase::OperatorPressed {
            self.evaluate_pending();
        }
    }

    pub fn backspace(&mut self) {
        if self.error || self.phase == InputPhase::ResultDisplayed {
            return;
        }

        if self.display.len() <= 1
            || (self.display.len() == 2 && self.display.starts_with('-'))
        {
            self.display = "0".into();
        } else {
            self.display.pop();
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn clear_entry(&mut self) {
        if self.error {
            self.clear();
            return;
        }
        self.display = "0".into();
    }

    pub fn toggle_sign(&mut self) {
        if self.error || self.display == "0" {
            return;
        }

        if self.display.starts_with('-') {
            self.display.remove(0);
        } else {
            self.display.insert(0, '-');
        }
    }

    pub fn percentage(&mut self) {
        if self.error {
            return;
        }

        let value = self.display_value();

        let result = if let (Some(acc), Some(_)) = (self.accumulator, self.pending_op) {
            // e.g. "200 + 10%" means "200 + 20"
            acc * value / 100.0
        } else {
            value / 100.0
        };

        self.display = Self::format_result(result);
    }

    // --- Private helpers ---

    fn display_value(&self) -> f64 {
        self.display.parse::<f64>().unwrap_or(0.0)
    }

    fn evaluate_pending(&mut self) {
        if let (Some(left), Some(op)) = (self.accumulator, self.pending_op) {
            let right = self.display_value();
            match Self::evaluate(left, op, right) {
                Some(result) if result.is_finite() => {
                    self.display = Self::format_result(result);
                    self.accumulator = Some(result);
                    self.pending_op = None;
                    self.phase = InputPhase::ResultDisplayed;
                }
                _ => {
                    self.display = "Error".into();
                    self.error = true;
                }
            }
        }
    }

    fn evaluate(left: f64, op: Operator, right: f64) -> Option<f64> {
        let result = match op {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            Operator::Multiply => left * right,
            Operator::Divide => {
                if right == 0.0 {
                    return None;
                }
                left / right
            }
        };
        Some(result)
    }

    fn format_result(value: f64) -> String {
        // Normalize negative zero
        let value = if value == 0.0 { 0.0 } else { value };

        if value.fract() == 0.0 && value.abs() < 1e15 {
            format!("{}", value as i64)
        } else {
            // Use up to 12 significant digits, strip trailing zeros
            let s = format!("{:.12}", value);
            let s = s.trim_end_matches('0');
            let s = s.trim_end_matches('.');
            s.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn press_digits(engine: &mut CalcEngine, digits: &str) {
        for ch in digits.chars() {
            if ch == '.' {
                engine.input_decimal();
            } else {
                engine.input_digit(ch);
            }
        }
    }

    #[test]
    fn test_addition() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "2");
        e.input_operator(Operator::Add);
        press_digits(&mut e, "3");
        e.input_equals();
        assert_eq!(e.display(), "5");
    }

    #[test]
    fn test_subtraction() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "9");
        e.input_operator(Operator::Subtract);
        press_digits(&mut e, "4");
        e.input_equals();
        assert_eq!(e.display(), "5");
    }

    #[test]
    fn test_multiplication() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "3");
        e.input_operator(Operator::Multiply);
        press_digits(&mut e, "7");
        e.input_equals();
        assert_eq!(e.display(), "21");
    }

    #[test]
    fn test_division() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "8");
        e.input_operator(Operator::Divide);
        press_digits(&mut e, "2");
        e.input_equals();
        assert_eq!(e.display(), "4");
    }

    #[test]
    fn test_division_by_zero() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "5");
        e.input_operator(Operator::Divide);
        press_digits(&mut e, "0");
        e.input_equals();
        assert!(e.is_error());
        assert_eq!(e.display(), "Error");
    }

    #[test]
    fn test_chained_operations() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "2");
        e.input_operator(Operator::Add);
        press_digits(&mut e, "3");
        e.input_operator(Operator::Multiply);
        // 2+3=5 should be evaluated, then 5*...
        assert_eq!(e.display(), "5");
        press_digits(&mut e, "4");
        e.input_equals();
        assert_eq!(e.display(), "20");
    }

    #[test]
    fn test_decimal_input() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "1.5");
        e.input_operator(Operator::Add);
        press_digits(&mut e, "2.3");
        e.input_equals();
        assert_eq!(e.display(), "3.8");
    }

    #[test]
    fn test_multiple_decimals() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "1.5");
        e.input_decimal();
        e.input_decimal();
        assert_eq!(e.display(), "1.5");
    }

    #[test]
    fn test_leading_zeros() {
        let mut e = CalcEngine::new();
        e.input_digit('0');
        e.input_digit('0');
        e.input_digit('5');
        assert_eq!(e.display(), "5");
    }

    #[test]
    fn test_backspace() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "123");
        e.backspace();
        assert_eq!(e.display(), "12");
    }

    #[test]
    fn test_backspace_to_zero() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "5");
        e.backspace();
        assert_eq!(e.display(), "0");
    }

    #[test]
    fn test_clear() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "123");
        e.input_operator(Operator::Add);
        e.clear();
        assert_eq!(e.display(), "0");
        assert!(!e.is_error());
    }

    #[test]
    fn test_toggle_sign() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "5");
        e.toggle_sign();
        assert_eq!(e.display(), "-5");
        e.toggle_sign();
        assert_eq!(e.display(), "5");
    }

    #[test]
    fn test_toggle_sign_zero() {
        let mut e = CalcEngine::new();
        e.toggle_sign();
        assert_eq!(e.display(), "0");
    }

    #[test]
    fn test_percentage_standalone() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "50");
        e.percentage();
        assert_eq!(e.display(), "0.5");
    }

    #[test]
    fn test_percentage_with_operator() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "200");
        e.input_operator(Operator::Add);
        press_digits(&mut e, "10");
        e.percentage();
        // 10% of 200 = 20
        assert_eq!(e.display(), "20");
        e.input_equals();
        // 200 + 20 = 220
        assert_eq!(e.display(), "220");
    }

    #[test]
    fn test_equals_then_digit() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "2");
        e.input_operator(Operator::Add);
        press_digits(&mut e, "3");
        e.input_equals();
        assert_eq!(e.display(), "5");
        press_digits(&mut e, "7");
        assert_eq!(e.display(), "7");
    }

    #[test]
    fn test_equals_then_operator() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "2");
        e.input_operator(Operator::Add);
        press_digits(&mut e, "3");
        e.input_equals();
        assert_eq!(e.display(), "5");
        e.input_operator(Operator::Add);
        press_digits(&mut e, "1");
        e.input_equals();
        assert_eq!(e.display(), "6");
    }

    #[test]
    fn test_operator_replacement() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "5");
        e.input_operator(Operator::Add);
        e.input_operator(Operator::Subtract);
        press_digits(&mut e, "3");
        e.input_equals();
        assert_eq!(e.display(), "2");
    }

    #[test]
    fn test_result_formatting() {
        assert_eq!(CalcEngine::format_result(4.0), "4");
        assert_eq!(CalcEngine::format_result(3.1), "3.1");
        assert_eq!(CalcEngine::format_result(-0.0), "0");
    }

    #[test]
    fn test_expression_display() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "12");
        e.input_operator(Operator::Add);
        assert_eq!(e.expression_display(), "12 +");
    }

    #[test]
    fn test_error_locks_input() {
        let mut e = CalcEngine::new();
        press_digits(&mut e, "5");
        e.input_operator(Operator::Divide);
        press_digits(&mut e, "0");
        e.input_equals();
        assert!(e.is_error());
        // All input should be ignored
        e.input_digit('3');
        assert_eq!(e.display(), "Error");
        e.input_operator(Operator::Add);
        assert_eq!(e.display(), "Error");
        // Only clear resets
        e.clear();
        assert!(!e.is_error());
        assert_eq!(e.display(), "0");
    }
}
