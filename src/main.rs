use std::{cmp, collections::HashMap, env, time::Instant};
use rayon::prelude::*;

#[derive(Copy, Clone)]
struct NumResult {
    steps_to_one: u128,
    max_value: u128,
}

#[derive(Copy, Clone)]
struct Result {
    max_steps: u32,
    max_steps_num: u128,
    highest_peak: u128,
    highest_peak_num: u128,
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
    let orders_of_magnitude: u32 = if args.len() > 1 {
        args[1].parse::<i32>().unwrap().try_into().unwrap()
    } else {
        10
    };

    let now = Instant::now();

    #[cfg(feature="cache")] {
        let mut map = HashMap::<u128, NumResult>::new();
    }

    println!("magnitude,max_steps,max_steps_num,highest_peak,highest_peak_num,elapsed_ms");

    for magnitude in 0..orders_of_magnitude {

        let curr_min = u128::pow(10, magnitude);
        let curr_max = u128::pow(10, magnitude + 1) / 2;

        //println!("from {} to {}", curr_min, curr_max);

        #[cfg(feature="parallel")] {
            let range = curr_min..curr_max;
            let result = range.into_par_iter().map(|x| {
                let i = x * 2 + 1;
                let mut max_steps: u32 = 0;
                let mut highest_peak: u128 = 0;
                let mut num: u128 = i;
                loop {
                    num = if num & 1 == 0 {
                        num >> 1
                    } else {
                        num + (num << 1 | 1)
                    };
                    max_steps += 1;
                    highest_peak = cmp::max(num, highest_peak);
    
                    if num == 1 {
                        break;
                    }
                }
                return Result { highest_peak, highest_peak_num: i, max_steps, max_steps_num: i };
            }).reduce(
                || Result { highest_peak: 0, highest_peak_num: 0, max_steps: 0, max_steps_num: 0 }, 
                |a, b| {
                    if a.max_steps >= b.max_steps && a.highest_peak > b.highest_peak {
                        return a;
                    }
                    let mut ret = a.clone();
                    if ret.highest_peak < b.highest_peak {
                        ret.highest_peak = b.highest_peak;
                        ret.highest_peak_num = b.highest_peak_num;
                    }
                    if ret.max_steps < b.max_steps {
                        ret.max_steps = b.max_steps;
                        ret.max_steps_num = b.max_steps_num;
                    }
                    ret
                
            });

            println!(
                "{},{},{},{},{},{}",
                magnitude, result.max_steps, result.max_steps_num, result.highest_peak, result.highest_peak_num, now.elapsed().as_millis()
            );
        }

        #[cfg(not(feature="parallel"))] {
            let mut max_steps: (u32, u128) = (0, 0);
            let mut highest_peak: (u128, u128) = (0, 0);

            for x in curr_min..curr_max {
                let i = x * 2 + 1;
                let mut steps: u32 = 0;
                let mut max: u128 = 0;
                let mut num: u128 = i;

                #[cfg(feature="cache")] {
                    let mut path = Vec::<u128>::new();

                    let mut last = NumResult {
                        steps_to_one: 0,
                        max_value: 1,
                    };
                }

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

                    #[cfg(feature="cache")] {
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
                }

                #[cfg(feature="cache")] {
                    memoize(&mut map, &path, &last);
                }

                if max_steps.0 < steps {
                    max_steps = (steps, i);
                }
                if highest_peak.0 < max {
                    highest_peak = (max, i);
                }
            }
            println!(
                "{},{},{},{},{},{}",
                magnitude, max_steps.0, max_steps.1, highest_peak.0, highest_peak.1, now.elapsed().as_millis()
            );
        }
    }
}
