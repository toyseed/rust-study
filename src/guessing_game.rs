use std::cmp::Ordering;
use std::io;
use std::io::{Read, Write};

use rand::distributions::Alphanumeric;
use rand::Rng;
/**
연습 문제 1회
컴퓨터가 미리 아스키 문자 1개를 랜덤하게 생성해서 기억하고 있고
터미널로 한 문자를 사용자로부터 입력 받아서 기억한 값보다 크면 `high` 작으면 `low`라고 찍어주고
만약 문자를 맟추면 'Good job' 이라고 찍고 프로그램을 종료하는 기능을 작성해 보세요.
**/
pub fn guess_ascii() {
    let secret = rand::thread_rng().sample(&Alphanumeric);
    println!("secret: {}, {}", secret, secret as u8);

    let mut input: [u8; 1] = [0; 1];
    let mut result_printed = false;

    print!("guess ascii character: ");
    loop {
        io::stdout().flush().expect("flush fail");
        io::stdin().read(&mut input).expect("read fail");
        let answer = input[0] as char;

        if result_printed && answer != '\n' {
            continue;
        } else if result_printed && answer == '\n' {
            result_printed = false;
            continue;
        }

        if !answer.is_alphanumeric() {
            print!("please enter an alphanumeric value: ");
            result_printed = answer != '\n';
            continue;
        }

        print!("you entered the '{}': ", answer);
        match secret.cmp(&answer) {
            Ordering::Greater => println!("high"),
            Ordering::Less => println!("low"),
            Ordering::Equal => {
                println!("good job");
                break;
            }
        }

        result_printed = true;
        print!("try agin: ");
    }
}

pub fn copied_guess() {
    let random = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .next()
        .expect("error");
    loop {
        println!("print something");
        let mut input: [u8; 1] = [0; 1];
        io::stdin().read(&mut input).expect("error");
        let input = input[0] as char;
        if !input.is_alphanumeric() {
            continue;
        }
        match random.cmp(&input) {
            Ordering::Less => println!("low"),
            Ordering::Greater => println!("high"),
            Ordering::Equal => {
                println!("Good job");
                break;
            }
        }
        println!("random is {:?}", random);
        println!("input is {:?}", input);
    }
}

pub fn copied_guess_num() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
