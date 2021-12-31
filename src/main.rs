use std::{cmp, env, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: u128 = if args.len() > 1 {
        args[1].parse::<i32>().unwrap().try_into().unwrap()
    } else {
        10000
    };

    let mut max_steps: (u128, u128) = (0, 0);
    let mut max_num: (u128, u128) = (0, 0);

    let now = Instant::now();

    for i in 1..count {
        let mut steps: u128 = 0;
        let mut max: u128 = 0;
        let mut num: u128 = i;
        while num != 1 {
            num = if num & 1 == 0 {
                num >> 1
            } else {
                num + (num << 1 | 1)
            };
            steps += 1;
            max = cmp::max(num, max);
        }

        if max_steps.0 < steps {
            max_steps = (steps, i);
        }
        if max_num.0 < max {
            max_num = (max, i);
        }
    }

    let elapsed_ms = now.elapsed().as_millis();

    println!(
        "max steps: {} for {}, max num: {} for {}",
        max_steps.0, max_steps.1, max_num.0, max_num.1
    );

    println!("elapsed: {}ms", elapsed_ms);
}
