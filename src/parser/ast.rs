
pub enum Expression {
    IntegerExpression(i64),
    PlusExpression(Box<Expression>, Box<Expression>),
    MultiplicationExpression(Box<Expression>, Box<Expression>),
}

impl Expression {

    #[allow(dead_code)]
    pub fn evaluate(&self) -> i64 {
        match self {
            Expression::IntegerExpression(value) => *value,
            Expression::PlusExpression(left, right) => left.evaluate() + right.evaluate(),
            Expression::MultiplicationExpression(left, right) => left.evaluate() * right.evaluate(),
        }
    }

    #[allow(dead_code)]
    pub fn is_integer_expression(&self) -> bool {
        match self {
            Expression::IntegerExpression(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_plus_expression(&self) -> bool {
        match self {
            Expression::PlusExpression(_, _) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_multiplication_expression(&self) -> bool {
        match self {
            Expression::MultiplicationExpression(_, _) => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Expression::IntegerExpression(value) => value.to_string(),
            Expression::PlusExpression(left, right) => format!("({}+{})", left.to_string(), right.to_string()),
            Expression::MultiplicationExpression(left, right) => format!("({}*{})", left.to_string(), right.to_string()),
        }
    }

    pub fn to_string_pretty(&self) -> String {

        // TODO: Teilaufgabe 3: SYNTAX: Implement this method
        // Hint: Use `match self { ... }` to match the type of the expression
        // Hint: Use the methods `is_integer_expression()`, `is_plus_expression()`, `is_multiplication_expression()` to check the type of the expression

        self.to_string() // <- Replace this with the correct implementation

        /*** Possible Solution: ***/
        /*
         * match self {
         *     Expression::IntegerExpression(value) => value.to_string_pretty(),
         *     Expression::PlusExpression(left, right) => format!("{}+{}", left.to_string_pretty(), right.to_string_pretty()),
         *     Expression::MultiplicationExpression(left, right) => {
         *         if left.is_plus_expression() && right.is_plus_expression() {
         *             format!("({})*({})", left.to_string_pretty(), right.to_string_pretty())
         *         } else if left.is_plus_expression() {
         *             format!("({})*{}", left.to_string_pretty(), right.to_string_pretty())
         *         } else if right.is_plus_expression() {
         *             format!("{}*({})", left.to_string_pretty(), right.to_string_pretty())
         *         } else {
         *             format!("{}*{}", left.to_string_pretty(), right.to_string_pretty())
         *         }
         *     }
         * }
         */
    }
}

pub fn new_integer_expression(value: i64) -> Expression {
    Expression::IntegerExpression(value)
}

pub fn new_plus_expression(left: Expression, right: Expression) -> Expression {
    Expression::PlusExpression(Box::new(left), Box::new(right))
}

pub fn new_multiplication_expression(left: Expression, right: Expression) -> Expression {
    Expression::MultiplicationExpression(Box::new(left), Box::new(right))
}
