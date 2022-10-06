use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use serde_json;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

struct Message {
    title: String,
    greeting: String,
    purpose: String,
    main_1: String,
    main_2: String,
    main_3: String,
    thanks: String,
    ps: String,
}

impl Message {
    pub fn generate(self) -> String {
        let mut list: Vec<&str> = Vec::with_capacity(9);
        list.push(&self.title);
        list.push("");
        list.push(&self.greeting);
        list.push(&self.purpose);
        list.push(&self.main_1);
        list.push(&self.main_2);
        list.push(&self.main_3);
        list.push(&self.thanks);
        list.push(&self.ps);

        return list.join("\n\n")
    }
}

#[derive(Deserialize, Debug)]
struct Database {
    title: Vec<String>,
    greeting: Vec<String>,
    purpose: Vec<String>,
    main_1: Vec<String>,
    main_2: Vec<String>,
    main_3: Vec<String>,
    thanks: Vec<String>,
    ps: Vec<String>,
}

impl Database {
    pub fn random_combination(&mut self) -> Message {
        return Message {
            title: self.title.choose(&mut thread_rng()).unwrap().clone(),
            greeting: self.greeting.choose(&mut thread_rng()).unwrap().clone(),
            purpose: self.purpose.choose(&mut thread_rng()).unwrap().clone(),
            main_1: self.main_1.choose(&mut thread_rng()).unwrap().clone(),
            main_2: self.main_2.choose(&mut thread_rng()).unwrap().clone(),
            main_3: self.main_3.choose(&mut thread_rng()).unwrap().clone(),
            thanks: self.thanks.choose(&mut thread_rng()).unwrap().clone(),
            ps: self.ps.choose(&mut thread_rng()).unwrap().clone(),
        };
    }
}

fn main() {
    print!("메시지 데이터베이스 파일 이름을 입력해주세요: ");
    io::stdout().flush().expect("flush failed!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("readline failed!");

    let path = Path::new(input.trim());
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut data: Database = serde_json::from_reader(reader).unwrap();
    println!("{}", data.random_combination().generate())
}
