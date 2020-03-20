use std::io;
fn main() {
    let mut stars = 0;
    let mut history = String::new();
    let mut rank = 25;
    let mut streak = 0;
    let mut legend = false;

    io::stdin()
        .read_line(&mut history)
        .expect("Failed to read history");

    let mut stars_required = 2;

    for c in history.into_bytes() {
        if c == 87 && !legend{
            //win

            match rank {
                20 => stars_required = 3,
                16 => stars_required = 3,
                15 => stars_required = 4,
                11 => stars_required = 4,
                10 => stars_required = 5,
                _ => {}
            }
            streak += 1;

            if streak > 2 && rank >= 6{
                stars += 2;
            } else {
                stars += 1;
            }

            if stars - stars_required == 1 {
                stars = 1;
                rank -= 1;
            } else if stars - stars_required == 2 {
                stars = 2;
                rank -= 1;
            }

            if rank == 0 {
                legend = true;
            }

        } else if c == 76 && !legend {
            //lose
            streak = 0;

            if rank <= 20  {
                stars -= 1;
            }

            if stars == -1 {
                if rank == 20 {
                    stars = 0;
                } else {
                    rank += 1;
            match rank {
                20 => stars_required = 3,
                16 => stars_required = 3,
                15 => stars_required = 4,
                11 => stars_required = 4,
                10 => stars_required = 5,
                _ => {}
            }
                    stars = stars_required -1; 
                }
            }

            
        }
    }

    if legend {
        println!("Legend");
    } else {
        println!("{}", rank);
    }
}
