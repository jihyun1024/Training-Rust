// if 조건문 연습: 화씨온도와 섭씨온도 변환
// (화씨온도) = 1.8 * (섭씨온도) + 32
// (섭씨온도) = ((화씨온도) - 32) / 1.8
use std::io;

fn main() {
    println!("Start: Enter 'Fahrenheit' or 'Celsius'");

    let mut t_unit = String::new();
    io::stdin()
        .read_line(&mut t_unit)
        .expect("Failed to read your input");
    let t_unit = t_unit.trim(); // 개행 문자 제거

    if t_unit == "Fahrenheit" {
        println!("You choose {}", t_unit);
        println!("\nEnter temperature for convert to Celsius Unit");

        let mut temperature= String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to input temperature");

        // 실수형으로 받아야 하므로 f64로 명시적으로 타입 선언
        // trim(): 개행 문자 제거
        // parse(): 문자열을 파싱해서 숫자로 변환
        let mut temperature:f64 = temperature.trim().parse().expect("Invalid Number");
        
        temperature = (temperature - 32.0) / 1.8;
        println!("Converted result(Fahrenheit -> Celsius): {:.2}", temperature); // {:.2}: 소수 둘째자리까지 출력

    } else if t_unit == "Celsius" {
        println!("You choose {}", t_unit);
        println!("\nEnter temperature for convert to Fahrenheit Unit");

        // 동일하게 코드 입력 (식만 바꿔서)
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to input temperature");

        let mut temperature:f64 = temperature.trim().parse().expect("Invalid Number");

        temperature = (1.8 * temperature) + 32.0;
        println!("Converted result(Celsius -> Fahrenheit): {:.2}", temperature);
    }
}
