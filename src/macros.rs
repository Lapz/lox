#[macro_export]
macro_rules! debug {
        ($($p:tt)*) => {if cfg!(feature = "debug") { println!($($p)*) } else { }}
}

macro_rules! binary_op {
    ($op:tt, $_self:ident) => {{

       let a = $_self.pop();
        let b = $_self.pop();

        $_self.push(a $op b)
    }};
}
