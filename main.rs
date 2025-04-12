// for 반복문 연습: n번째 피보나치 수 생성
// 피보나치 수: 첫째 항, 둘째 항 = 1, 그 뒤의 모든 항은 바로 앞 두 항의 합인 수열

use std::io;

// 재귀함수를 이용해서 계산
fn fibonacci(n:i64) -> i64 {
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

// 반복문을 이용해서 계산
fn fibonacci2(n:i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut n1: i64 = 0;
    let mut n2: i64 = 1;
    let mut number: i64 = 0;

    for _ in 2..=n {
        number = n1 + n2;
        n1 = n2;
        n2 = number;
    }

    return number;
}

fn main() {
    println!("Enter n-th number to calculate fibonacci number");
    
    // String 타입으로 받고 나서 나중에 타입 변환
    let mut n: String = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to input");

    // 정수형으로 변환해야 하므로 i64로 명시적 타입 선언
    let n:i64 = n.trim().parse().expect("Failed to convert string to integer");

    println!("(Recursive Method) {}th fibonacci number is {}", n, fibonacci(n));
    println!("(Iterative Method) {}th fibonacci number is {}", n, fibonacci2(n));
}