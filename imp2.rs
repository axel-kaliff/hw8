use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();

    let mut number_of_measurements = 1.0;
    let mut time_interval = 0.0;
    let mut last_time = 0.0;
    let mut real_locations = Vec::new();
    let mut imperfect_locations = Vec::new();
    let mut x = 0.0;
    let mut y = 0.0;
    let mut distance_traveled = 0.0;
    let mut counter = 0.0;
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<f64> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();


        if number_of_measurements > 0.0 {
            if nums.capacity() == 2 as usize {
                time_interval = nums[1];
                number_of_measurements = nums[0];
            } else {
                if nums[2] == 0.0 {
                    x = nums[0];
                    y = nums[1];
                    real_locations.push((0.0, 0.0));
                    imperfect_locations.push((0.0, 0.0));
                } else {
                    counter += 1.0;
                    let time_difference = nums[2] - last_time;

                    let x_square = (nums[0] as f64 - x).powi(2);
                    let y_square = (nums[1] as f64 - y).powi(2);

                    distance_traveled += (x_square + y_square).sqrt();

                    //räkna ut distance per t
                    //ta det gånger delta_t / time_interval
                    //lägg det på förra mätningen, lägg till den koordinaten i vec


                    let times_measured = time_difference as u64 / time_interval as u64;

                    let delta_x_per_t = (nums[0] - x) / time_difference;
                    let delta_y_per_t = (nums[1] - y) / time_difference;

                    
                    let seconds_real_measurements_happened = nums[2] - time_interval * counter;
                    let maybe_delta = (nums[0] - x) - delta_x_per_t * seconds_real_measurements_happened;
                    let maybe_delta_y = (nums[1] - y) - delta_y_per_t * seconds_real_measurements_happened;

                    println!("maaaybe:  {}, y: {}", maybe_delta, maybe_delta_y);
                    imperfect_locations.push((x + times_measured as f64 * delta_x_per_t, y + times_measured as f64 * delta_y_per_t));

                    last_time = nums[2];
                    x = nums[0] as f64;
                    y = nums[1] as f64;
                }

                number_of_measurements -= 1.0;
                if number_of_measurements == 0.0 {
                    imperfect_locations.push((x, y));
                    for (x, y) in &imperfect_locations {
                println!("{} : {}", x, y);
                    }
                }
            }
        }
    }
}
