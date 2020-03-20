use std::collections::HashMap;
use std::io::{self, BufRead};
pub fn run() {
    let mut definitions = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<String> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if nums[0] == "clear" {
            definitions.clear();
        } else if nums[0] == "def" && nums.capacity() > 2 {
            let number: i64 = nums[2].parse().unwrap();
            definitions.insert(nums[1].clone(), number);
        } else {
            //calc
            let mut sum = 0;
            let mut operator = 1;
            let mut calculation_is_valid = true;
            let mut range = 1..nums.capacity() - 1;

            loop {
                match range.next() {
                    Some(x) => {
                        if x as i64 % 2 != 0 {
                            if !definitions.contains_key(&nums[x]) {
                                calculation_is_valid = false;
                                break;
                            } else {
                                sum += operator * definitions.get(&nums[x]).unwrap();
                            }
                        } else {
                            if x != nums.capacity() {
                                match nums[x].as_ref() {
                                    "+" => operator = 1,
                                    "-" => operator = -1,
                                    "=" => break,
                                    _ => {}
                                }
                            }
                        }

                        }

                    None => break,
                }



            }

                        for word in &nums {
                            if word != "calc" {
                                print!("{} ", word);    
                            }
                        }
                        if calculation_is_valid {
                            let mut found = false;
                            for (name, number) in &definitions {
                                if sum == *number {
                                    found = true;
                                    println!("{}", name);
                                }
                            }
                            if !found {
                            
                            println!("unknown");
                            }

                        } else {
                            println!("unknown");
                        }
        }
    }
}
