use std::collections::HashMap;

pub fn go()  {
    println!("Day 15");

    let mut dict: HashMap<i32, i32> = HashMap::new();
    let mut count = 0i32;

    add_to_list(&mut dict, &mut count, 6);
    add_to_list(&mut dict, &mut count, 3);
    add_to_list(&mut dict, &mut count, 15);
    add_to_list(&mut dict, &mut count, 13);
    add_to_list(&mut dict, &mut count, 1);

    let mut next: i32 = 0;

    loop {
        let newval = match dict.get(&next) {
            None => {
                0
            },
            Some(index) => {
                count - index + 1
            }
        };
        add_to_list(&mut dict, &mut count, next);
        next = newval;

        if count==30000000 - 1 {
            break;
        }
    }

    println!("output {}", next);
}

fn add_to_list(dict: &mut HashMap<i32, i32>, count: &mut i32, num: i32) {
    *count += 1;
    dict.insert(num, *count);
}

