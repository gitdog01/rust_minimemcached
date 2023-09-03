use std::collections::HashMap;
use std::io::{self, Write};
mod action;
mod command;

struct Minimemcached {
    items: HashMap<String, String>,
}

fn main() {
    let mut mm: Minimemcached = Minimemcached {
        items: HashMap::new(),
    };

    println!("hello");

    command_line(&mut mm);

}

fn command_line(mm :&mut Minimemcached) {
    loop {
        print!("> ");  // 프롬프트 출력
        io::stdout().flush().unwrap();  // 화면에 바로 표시되게 하기 위해 출력 버퍼를 flush합니다.

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let buffer = input.trim();  // 입력의 앞뒤 공백 제거
                // get은 {커맨드} {키} {값} 형태로 입력받습니다.
                let mut iter = buffer.split_whitespace();
                let cmd = iter.next();
                match cmd {
                    Some("get") => {
                        // 커맨드가 get 일때
                        let key = iter.next();
                        command::get(&mm, key);
                    }
                    Some("set") => {
                        let key = iter.next();
                        let value = iter.next();
                        command::set(mm, key, value)
                    }
                    Some("flush") => {
                        command::flush(mm);
                    }
                    Some("quit") => {
                        command::quit();
                    }
                    _ => {
                        println!("unknown command");
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
}