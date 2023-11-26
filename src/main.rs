use std::{cmp, array};
use std::io;

const SIZE: usize = 15;
const STEP: usize = 5;

fn main() {
    let mut array = [0; SIZE];
    let mut array_sorting = [0; SIZE];
    let mut min_array = [i32::MAX; 3];
    let mut k_array = [0; 3];
    let mut sum_array = [0; 3];
    let mut temp = 0;

    println!("Enter your array");
    read_array(&mut array);
    println!("Your final array");
    for i in 0..SIZE{
        print!("{} ", array[i]);
    }

    for i in (0..).step_by(1).take_while(|&i| i < {SIZE / STEP}){
        let mut section_start = i * STEP;
        let mut section_end = section_start + STEP;
        for j in section_start..section_end{
            if array[j] < min_array[i]{
                min_array[i] = array[j];
            }
            if array[j] == 0{
                k_array[i] += 1;
            }
            if array[j] > 0{
                sum_array[i] += array[j];
            }
        }
    }

    for i in 0..SIZE{
        array_sorting[i] = array[i];
    }
    println!();
    for i in 0..SIZE{
        for i in 0..(SIZE - 1){
            if array_sorting[i] > array_sorting[i+1]{
                temp = array_sorting[i];
                array_sorting[i] = array_sorting[i+1];
                array_sorting[i+1] = temp;
            }
        }
    }

    for i in 0..SIZE{
        print!("{} ", array_sorting[i]);

    }
    print!("\nMIN: {}, {}, {}", min_array[0], min_array[1], min_array[2]);
    println!();

    let mut section_with_most_zeroes_index = 0;
    if k_array[2] > k_array[1]{
        section_with_most_zeroes_index = 2;
    } else if k_array [1] > k_array[0] {
        section_with_most_zeroes_index = 1;
    }
    let mut section_start = section_with_most_zeroes_index * STEP;
    let mut section_end = section_start  + STEP;
    for i in section_start..section_end{
        print!("{} ", array[i]);
    }

    let mut biggest_sum_index = 1;
    if sum_array[2] > sum_array[1] {
        biggest_sum_index = 3;
    } else if sum_array[1] > sum_array[0] {
        biggest_sum_index = 2;
    }
    print!("\n{} ", biggest_sum_index);
}


fn read_line() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line :(");
    input.trim().to_string()

}

fn read_array(array:&mut [i32;15]){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line :(");
    for (i,x) in input.trim().split_whitespace().enumerate().take(15){
        array[i] = x.parse().unwrap();
    }
}


