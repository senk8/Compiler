use::std::env;

fn main() {
    let arg = env::args().nth(1).unwrap();

    let mut p = arg.split(" ");

    let n = {
        let s = p.next().unwrap();
        usize::from_str_radix(s, 10).unwrap()
    };

    print!(".intel_syntax noprefix\n");
    print!(".globl main\n");
    print!("main:\n");
    print!("  mov rax, {}\n" ,&n);

    while let Some(op) = p.next() {
        let n = {
            let s = p.next().unwrap();
            usize::from_str_radix(s, 10).unwrap()
        };
        match op {
            "+" => println!("  add rax, {}", &n),
            "-" => println!("  sub rax, {}", &n),
             _  => panic!("予期しない文字です: '{}'\n", &op),
        }
    }

    print!("  ret\n");
    return;
}
