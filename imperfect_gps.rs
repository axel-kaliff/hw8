use std::io::{self, BufRead};
pub fn main() {
    let stdin = io::stdin();
    let mut time_interval = 0.0;
    let mut number_of_meaurements = 1.0;
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut last_time = 0.0;
    let mut distance_traveled = 0.0;
    let mut gps_distance = 0.0;
    let mut gps_x = 0.0;
    let mut gps_y = 0.0;
    
    let mut locations = Vec::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<f64> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if number_of_meaurements > 0.0 {
            if nums.capacity() == 2 as usize {
                time_interval = nums[1];
                number_of_meaurements = nums[0];
            } else {
                if nums[2] == 0.0 {
                    x = nums[0];
                    y = nums[1];
                    locations.push((0.0, 0.0));
                } else {
                    let time_difference = nums[2] - last_time;

                    for time in 1..time_difference as u64 + 1 {
                            let change_x = x + (nums[0] - x) * time as f64 / time_difference;
                            let change_y = y + (nums[1] - y) * time as f64 / time_difference;
                            locations.push((change_x, change_y));                
                    }

                    let x_square = (nums[0] as f64 - x).powi(2);
                    let y_square = (nums[1] as f64 - y).powi(2);

                    distance_traveled += (x_square + y_square).sqrt();

                    last_time = nums[2];
                    x = nums[0] as f64;
                    y = nums[1] as f64;
                }

                number_of_meaurements -= 1.0;
                if number_of_meaurements == 0.0 {

            for i in 0..locations.len() {
                if i  as f64% time_interval == 0.0 {
                    let gps_x_square = (locations[i].0 as f64 - gps_x).powi(2);
                    let gps_y_square = (locations[i].1 as f64 - gps_y).powi(2);
                    gps_distance += (gps_x_square + gps_y_square).sqrt();
                    gps_x = locations[i].0;
                    gps_y = locations[i].1;
                }
            }

            if number_of_meaurements as f64 % time_interval == 0.0 {
                    let gps_x_square = (locations[locations.len() - 1].0 as f64 - gps_x).powi(2);
                    let gps_y_square = (locations[locations.len() - 1].1 as f64 - gps_y).powi(2);
                    gps_distance += (gps_x_square + gps_y_square).sqrt();
            } 

            println!("{}", (distance_traveled - gps_distance) / distance_traveled * 100.0);
                }
            }
        } 
        
    }
}
