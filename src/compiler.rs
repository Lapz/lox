use scanner::Scanner;


pub fn compile(source:&str) {
    let scanner = Scanner::new();

    loop {
        let token = scanner.scan();
    }
}