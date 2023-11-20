pub struct Calculator {
    acc: i64,
}

impl Calculator {
    pub fn new(initial_value: i64) -> Self {
        Calculator { acc: initial_value }
    }

    pub fn add(&mut self, value: i64) -> Result<(), &'static str> {
        self.acc += value;

        Ok(())
    }

    pub fn subtract(&mut self, value: i64) -> Result<(), &'static str> {
        self.acc -= value;

        Ok(())
    }

    pub fn multiply(&mut self, value: i64) -> Result<(), &'static str> {
        self.acc *= value;

        Ok(())
    }

    pub fn divide(&mut self, value: i64) -> Result<(), &'static str> {
        if value != 0 {
            self.acc /= value;
            Ok(())
        } else {
            Err("You can't divide by zero!")
        }
    }

    pub fn get_result(&mut self) -> i64 {
        let result = self.acc;
        self.acc = 0;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers() {
        let mut calc = Calculator::new(40);

        calc.add(2).unwrap();

        let result = calc.get_result();

        assert_eq!(result, 42);
    }

    #[test]
    fn subtract_two_numbers() {
        let mut calc = Calculator::new(10);

        calc.subtract(5).unwrap();

        let result = calc.get_result();

        assert_eq!(result, 5);
    }

    #[test]
    fn divide_two_numbers() {
        let mut calc = Calculator::new(50);

        calc.divide(2).unwrap();

        let result = calc.get_result();

        assert_eq!(result, 25);
    }


    #[test]
    fn cant_divide_by_zero() {
        let mut calc = Calculator::new(10000);

        let result = calc.divide(0);

        assert!(result.is_err());
    }

    #[test]
    fn use_many_operations() {
        let mut calc = Calculator::new(10);

        calc.add(10).unwrap();
        calc.subtract(5).unwrap();
        calc.divide(3).unwrap();
        calc.multiply(10).unwrap();

        let result = calc.get_result();

        assert_eq!(result, 50);
    }
}
