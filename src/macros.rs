#[macro_export]
macro_rules! debug {
        ($($p:tt)*) => {if cfg!(feature = "debug") { println!($($p)*) } else { }}
}

macro_rules! binary_op {
    ($op:tt, $_self:ident) => {{

        let b = $_self.pop();
        let a = $_self.pop();

        $_self.push(a $op b)
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

macro_rules! parse_prefix {
    ($parser:ident,$struct_name:ident,$token:ident) => {{
        #[derive(Debug)]
        struct $struct_name;

        {
            $parser.prefix(&$struct_name, $token);
        }

        impl PrefixParser for $struct_name {
            fn parse(&self, compiler: &mut Compiler) -> Result<(), ()> {
                compiler.grouping()?;
                Ok(())
            }
        }
    }};
}
