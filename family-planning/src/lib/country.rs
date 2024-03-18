use std::{cmp::min, collections::VecDeque, vec};
//人均寿命60岁,只有相同的年龄的人会结婚,假设全部能结婚,每对新人只会结婚一次,生育一次.18岁没能结婚就孤独终老.boy 的下标代表年龄
#[derive(Debug)]
struct Country {
    boy: VecDeque<i32>,
    girl: VecDeque<i32>,
    count: i64,
    live_boy: i32,
    dead_boy: i32,
    live_girl: i32,
    dead_girl: i32,
    birth_method: Vec<fn(&Country) -> (i32, i32)>,
}
// use lazy_static::lazy_static;
// use std::sync::Mutex;
// lazy_static! {
//     static ref COUNT: Mutex<i32> = Mutex::new(0);
// }

impl Country {
    fn new(methods: Vec<fn(&Country) -> (i32, i32)>) -> Self {
        let mut boy: VecDeque<i32> = vec![0; 18].into();
        boy.push_back(50);
        let mut girl: VecDeque<i32> = vec![0; 18].into();
        girl.push_back(50);
        Self {
            boy,
            girl,
            count: 0,
            live_boy: 10000,
            dead_boy: 0,
            live_girl: 10000,
            dead_girl: 0,
            birth_method: methods,
        }
    }
    fn dead(&mut self) {
        if self.boy.len() == 60 {
            if let Some(boy) = self.boy.pop_back() {
                self.live_boy -= boy;
                self.dead_boy += boy;
            }
            if let Some(girl) = self.girl.pop_back() {
                self.live_girl -= girl;
                self.dead_girl += girl;
            }
        }
    }
    fn birth(&mut self) {
        self.count += 1;

        let groom = self.boy[18];
        let bride = self.girl[18];
        let mut little_boy = 0;
        let mut little_girl = 0;
        let birth_method = self.choose_birth_method();
        for _ in 0..=min(groom, bride) {
            let (b, g) = birth_method(&self);
            little_boy += b;
            little_girl += g
        }
        self.boy.push_front(little_boy);
        self.live_boy += little_boy;
        self.girl.push_front(little_girl);
        self.live_girl += little_girl;
    }
    fn choose_birth_method(&mut self) -> fn(&Country) -> (i32, i32) {
        if self.birth_method.len() == 1 {
            return self.birth_method[0];
        }
        use rand::Rng; // 引入Rng trait，以便使用随机数生成方法
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.birth_method.len());
        return self.birth_method[index];
    }
}

fn normal_birth_method(_: &Country) -> (i32, i32) {
    use rand::Rng; // 引入Rng trait，以便使用随机数生成方法
    let mut rng = rand::thread_rng();
    let is_boy = rng.gen_bool(0.5);
    if is_boy {
        (1, 0)
    } else {
        (0, 1)
    }
}
fn normal_birth_method_basic_natural(_: &Country) -> (i32, i32) {
    use rand::Rng; // 引入Rng trait，以便使用随机数生成方法
    let mut rng = rand::thread_rng();
    let is_boy = rng.gen_bool(0.5);
    let is_girl = rng.gen_bool(0.5);
    match (is_boy, is_girl) {
        (true, true) => (1, 1),
        (false, false) => (0, 0),
        (true, false) => (1, 0),
        (false, true) => (0, 1),
    }
}
fn normal_birth_method_without_one_plan(_: &Country) -> (i32, i32) {
    use rand::Rng; // 引入Rng trait，以便使用随机数生成方法
    let mut rng = rand::thread_rng();
    let is_boy = rng.gen_bool(0.5);
    let is_boy2 = rng.gen_bool(0.5);
    match (is_boy, is_boy2) {
        (true, true) => (2, 0),
        (false, false) => (0, 2),
        (true, false) => (1, 1),
        (false, true) => (1, 1),
    }
}
fn kill_girl_birth_method(_: &Country) -> (i32, i32) {
    use rand::Rng; // 引入Rng trait，以便使用随机数生成方法
    let mut rng = rand::thread_rng();
    let is_boy = rng.gen_bool(0.5);
    if is_boy {
        return (1, 0);
    }
    let kill_girl = rng.gen_range(0..10);

    if kill_girl > 6 {
        (0, 0)
    } else {
        (0, 1)
    }
}

fn kill_girl_when_stupid(c: &Country) -> (i32, i32) {
    if c.live_boy as f64 / c.live_girl as f64 > 1.7 {
        normal_birth_method_without_one_plan(c)
    } else {
        kill_girl_birth_method(c)
    }
}
