#[macro_export]
macro_rules! debug {
        ($($p:tt)*) => {if cfg!(feature = "debug") { println!($($p)*) } else { }}
}

macro_rules! binary_op {
    ($op:tt,$return_ty:ident,$_self:ident) => {{

        if !$_self.peek(1).is_number() || !$_self.peek(2).is_number() {            
            return $_self.runtime_error(&format!("`{}` operands must be numbers.",stringify!($op)))
        }

        let b = $_self.pop().as_number();
        let a = $_self.pop().as_number();

        

        $_self.push(Value::$return_ty(a $op b))
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
