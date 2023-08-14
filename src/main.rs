use std::collections::HashMap;
use std::io::{self, Write};

struct Minimemcached {
    items: HashMap<String, i32>,
}

fn main() {
    let mut mm = Minimemcached {
        items: HashMap::new(),
    };

    mm.items.insert("first_key".to_string(), 42);
    mm.items.insert("second_key".to_string(), 123);

    command_line();

    println!("hello");
}


fn command_line() {
    loop {
        print!("> ");  // 프롬프트 출력
        io::stdout().flush().unwrap();  // 화면에 바로 표시되게 하기 위해 출력 버퍼를 flush합니다.

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cmd = input.trim();  // 입력의 앞뒤 공백 제거
                if cmd == "exit" {
                    break;  // 'exit' 입력 시 루프 종료
                } else {
                    // TODO: cmd에 대한 처리 로직
                    println!("You entered: {}", cmd);
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
}