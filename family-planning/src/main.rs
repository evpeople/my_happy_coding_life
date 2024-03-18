fn main() {
    let mut child = vec![(0, 0)];
    // let parent = (100, 100);
    // for _ in 0..=100000 {
    //     child.push(birth())
    // }

    for _ in 0..=100000 {
        child.push(birth_second())
    }
    let spn = child.iter().fold((0, 0), |mut b_a_g, (a, b)| {
        b_a_g.0 += a;
        b_a_g.1 += b;
        return b_a_g;
    });
    let spn_bg: Vec<(i32, i32)> = child.iter().filter(|b_g| b_g.1 > 1).cloned().collect();
    // let spn = child.iter().filter(|b_and_g| b_and_g.1 > 1).count();
    // .collect::<&(i32, i32)>();
    // println!("{:#?}", child);
    println!("{:#?}", spn_bg);
    println!("{:#?}", spn.1 as f64 / spn.0 as f64)
}
// 一直生,直到生到男孩
fn birth() -> (i32, i32) {
    use rand::Rng; // 引入Rng trait，以便使用随机数生成方法
    let mut rng = rand::thread_rng();
    let mut is_boy = rng.gen_bool(0.5);
    let mut b_and_g = (0, 0);
    loop {
        if is_boy {
            b_and_g.0 += 1;
            break;
        } else {
            b_and_g.1 += 1
        }
        is_boy = rng.gen_bool(0.5);
    }
    b_and_g
}
// second birth 杀婴
fn birth_second() -> (i32, i32) {
    use rand::Rng; // 引入Rng trait，以便使用随机数生成方法
    let mut rng = rand::thread_rng();
    let mut is_boy = rng.gen_bool(0.5);
    let mut b_and_g = (0, 0);
    loop {
        if is_boy {
            b_and_g.0 += 1;
            break;
        } else {
            let shall_we_kill = rng.gen_bool(0.2);
            if !shall_we_kill {
                b_and_g.1 += 1
            }
        }
        is_boy = rng.gen_bool(0.5);
    }
    b_and_g
}
