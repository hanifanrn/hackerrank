// problem source:
// https://hackerrank.com/challenges/plus-minus/problem

/*
intuition

approach

complexity
*/

use std::io::{self, BufRead};

/*
 *  Complete the 'plusMinus' function below.
 *
 *  The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let length_arr = arr.len() as f32;
    let mut count_positive: f32 = 0.0;
    let mut count_negative: f32 = 0.0;
    let mut count_zero: f32 = 0.0;

    for i in arr {
        if i > &0 {
            count_positive += 1.0;
        } else if i < &0 {
            count_negative += 1.0;
        } else {
            count_zero += 1.0;
        }
    }

    println!("{:.6}", count_positive / length_arr);
    println!("{:.6}", count_negative / length_arr);
    println!("{:.6}", count_zero / length_arr);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
