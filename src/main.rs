use std::{cmp, collections::HashMap, env, time::Instant};

#[derive(Copy, Clone)]
struct NumResult {
    steps_to_one: u128,
    max_value: u128,
}

fn memoize(map: &mut HashMap<u128, NumResult>, path: &Vec<u128>, res: &NumResult) {
    // update results
    let mut steps_to_one = res.steps_to_one;
    let mut max_value = res.max_value;

    for num in path.iter().rev() {
        map.insert(
            *num,
            NumResult {
                max_value,
                steps_to_one,
            },
        );

        steps_to_one += 1;
        max_value = cmp::max(max_value, *num);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: u128 = if args.len() > 1 {
        args[1].parse::<i32>().unwrap().try_into().unwrap()
    } else {
        10000000
    };

    let mut max_steps: (u128, u128) = (0, 0);
    let mut max_num: (u128, u128) = (0, 0);

    let now = Instant::now();

    let mut map = HashMap::<u128, NumResult>::new();

    for i in 1..count {
        let mut steps: u128 = 0;
        let mut max: u128 = 0;
        let mut num: u128 = i;

        let mut path = Vec::<u128>::new();

        let mut last = NumResult {
            steps_to_one: 0,
            max_value: 1,
        };

        loop {
            num = if num & 1 == 0 {
                num >> 1
            } else {
                num + (num << 1 | 1)
            };
            steps += 1;
            max = cmp::max(num, max);

            if num == 1 {
                break;
            }

            if let Some(result) = map.get(&num) {
                //println!("{} Cache hit at {}", i, num);
                steps += result.steps_to_one;
                max = cmp::max(max, result.max_value);
                last = result.clone();
                break;
            } else {
                //println!("{} Cache miss at {}", i, num);
                path.push(num);
            }
        }

        memoize(&mut map, &path, &last);

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
