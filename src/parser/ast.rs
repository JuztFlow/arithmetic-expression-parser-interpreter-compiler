
pub enum Expression {
    Integer(i64),
    Plus(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

impl Expression {

    #[allow(dead_code)]
    pub fn evaluate(&self) -> i64 {
        match self {
            Expression::Integer(value) => *value,
            Expression::Plus(left, right) => left.evaluate() + right.evaluate(),
            Expression::Multiply(left, right) => left.evaluate() * right.evaluate(),
        }
    }

    #[allow(dead_code)]
    pub fn is_integer_expression(&self) -> bool {
        match self {
            Expression::Integer(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_plus_expression(&self) -> bool {
        match self {
            Expression::Plus(_, _) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_multiplication_expression(&self) -> bool {
        match self {
            Expression::Multiply(_, _) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Expression::Integer(value) => value.to_string(),
            Expression::Plus(left, right) => format!("({}+{})", left.to_string(), right.to_string()),
            Expression::Multiply(left, right) => format!("({}*{})", left.to_string(), right.to_string()),
        }
    }

    #[allow(dead_code, unused_variables)]
    pub fn to_string_pretty(&self) -> String {

        /*   
         *   TODO: Teilaufgabe 3: SYNTAX: Implement this method
         *
         *   Hints:
         *      - Use `match self { ... }` to match the type of the expression, and return the corresponding string per case
         *      - This function needs to be recursive, because the expression can be arbitrarily nested
         *      - The methods `is_integer_expression()`, `is_plus_expression()` and/or `is_multiplication_expression()` might be useful to determine the correct parentheses
         */

        self.to_string() // <- Replace this with you solution!

        /*   
         *   Possible Solution:
         *
         *   match self {
         *       Expression::Integer(value) => value.to_string_pretty(),
         *       Expression::Plus(left, right) => format!("{}+{}", left.to_string_pretty(), right.to_string_pretty()),
         *       Expression::Multiply(left, right) => {
         *           if left.is_plus_expression() && right.is_plus_expression() {
         *               format!("({})*({})", left.to_string_pretty(), right.to_string_pretty())
         *           } else if left.is_plus_expression() {
         *               format!("({})*{}", left.to_string_pretty(), right.to_string_pretty())
         *           } else if right.is_plus_expression() {
         *               format!("{}*({})", left.to_string_pretty(), right.to_string_pretty())
         *           } else {
         *               format!("{}*{}", left.to_string_pretty(), right.to_string_pretty())
         *           }
         *       }
         *   }
         */
    }
}

pub fn new_integer_expression(value: i64) -> Expression {
    Expression::Integer(value)
}

pub fn new_plus_expression(left: Expression, right: Expression) -> Expression {
    Expression::Plus(Box::new(left), Box::new(right))
}

pub fn new_multiplication_expression(left: Expression, right: Expression) -> Expression {
    Expression::Multiply(Box::new(left), Box::new(right))
}

