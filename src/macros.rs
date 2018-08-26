#[macro_export]
macro_rules! debug {
        ($($p:tt)*) => {if cfg!(feature = "debug") { println!($($p)*) } else { }}
}

macro_rules! binary_op {
    ($op:tt, $_self:ident) => {{

        let b = $_self.pop().as_number();
        let a = $_self.pop().as_number();

        $_self.push(Value::number(a $op b))
    }};
}

macro_rules! eof_error {
    ($_self:ident) => {{
        let msg = format!("Unexpected EOF");
        let end = $_self.reporter.end();
        $_self.reporter.error(msg, end);
        Err(())
    }};
}
