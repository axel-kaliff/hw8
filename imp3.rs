use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut n = 0.0;
    let mut t = 0.0;
    let mut real_locations = Vec::new();
    let mut gps_locations = Vec::new();

    let mut accTIndex = 1.0;
    let mut lastAccT = 0.0;
    let mut currT = t.clone();

    for (count, line) in stdin.lock().lines().map(|l| l.unwrap()).enumerate() {
        let nums: Vec<f64> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if nums.len() != 0 {
        if count == 0 {
            t = nums[1];
            n = nums[0];
        } else if count == 1 {
            real_locations.push((0.0, 0.0, 0.0));
            gps_locations.push((0.0, 0.0, 0.0));
        } else {
            real_locations.push((nums[0], nums[1], nums[2]));
        }
        }
    }

    let timeFinished = real_locations.last().unwrap().2; 
    while currT < timeFinished {
        let mut accT = real_locations[accTIndex as usize].2;

        while currT > accT{
            accTIndex += 1.0;
            lastAccT = accT;
            accT = real_locations[accTIndex as usize].2;
        }

        let delta = (currT - lastAccT) as f64 / (accT - lastAccT) as f64;
        let currX = real_locations[accTIndex as usize].0;
        let currY = real_locations[accTIndex as usize].1;
        let prevX = real_locations[(accTIndex - 1.0) as usize].0;
        let prevY = real_locations[(accTIndex - 1.0) as usize].1;
        gps_locations.push(((currX - prevX) as f64 * delta, (currY - prevY) as f64 * delta, currT));
        let xOff = real_locations[(accTIndex- 1.0) as usize].0;
        let yOff = real_locations[(accTIndex- 1.0) as usize].1;
        let lastX = gps_locations.last().unwrap().0 + xOff;
        let lastY = gps_locations.last().unwrap().1 + yOff;
        let lastT = gps_locations.last().unwrap().2;
        gps_locations.remove(gps_locations.len() -1);
        gps_locations.push((lastX, lastY, lastT));
        currT += t;
    }
    gps_locations.push(real_locations.last().unwrap().clone());

    let mut total = 0.0;
    let mut gps_total = 0.0;

    for (c, x) in real_locations.iter().enumerate().filter(|(count, item)| *count != real_locations.len() -1 ){
       let  x_square = (real_locations[c + 1].0 - x.0).powi(2);
       let y_square =  (real_locations[c + 1].1 - x.1).powi(2);
       total += (y_square + x_square).sqrt();
    }


    for (c, x) in gps_locations.iter().enumerate().filter(|(count, item)| *count != gps_locations.len() -1 ){
       let  x_square = (gps_locations[c + 1].0 - x.0).powi(2);
       let y_square =  (gps_locations[c + 1].1 - x.1).powi(2);
       gps_total += (y_square + x_square).sqrt();
    }

    
    println!("{}", (total - gps_total) / total * 100.0 ); 
}
