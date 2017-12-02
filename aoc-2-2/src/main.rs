use std::io::{stdin, BufRead, BufReader};

fn main() {
    let br = BufReader::new(stdin());

    let mut sum = 0;

    for l in br.lines().map(|l| l.unwrap()) {

        let mut numbers = l.split('\u{0009}').map(|p| p.parse::<u32>().unwrap()).collect::<Vec<_>>();
        numbers.sort();

        'outer: for i in 0..numbers.len() {
            let i = numbers.len() - i - 1;
            
            for j in 0..i {
                let j = i - j - 1;

                if numbers[i] % numbers[j] == 0 {
                    sum += numbers[i] / numbers[j];
                    break 'outer;
                }
            }
        }

        println!("{}", sum);
    }
}
