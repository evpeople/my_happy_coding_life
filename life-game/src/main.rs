use std::cell::RefCell;
use std::io;
use std::io::{stdout, Stdout};

use std::rc::Rc;
use std::time::{Duration, Instant};

use clap::Parser;

use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, ClearType};
use crossterm::{event, execute, terminal, ExecutableCommand};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
    is_alive: bool,
    world: Rc<RefCell<Vec<Vec<bool>>>>,
}

struct World {
    x: usize,
    y: usize,
    seed: Vec<Point>,
    world: Rc<RefCell<Vec<Vec<bool>>>>,
    dead: String,
    alive: String,
    stdout: Stdout,
}

impl Point {
    fn new(x: usize, y: usize, is_alive: bool, world: Rc<RefCell<Vec<Vec<bool>>>>) -> Self {
        Self {
            x,
            y,
            is_alive,
            world,
        }
    }
    fn new_xy(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            is_alive: false,
            world: Rc::new(RefCell::new(vec![])),
        }
    }
    fn check_around(&self) -> bool {
        let alive = self.get_neighbor();
        if self.is_alive && alive < 2 {
            false
        } else if self.is_alive && alive > 3 {
            false
        } else if !self.is_alive && alive == 3 {
            true
        } else if self.is_alive && (alive == 3 || alive == 2) {
            true
        } else {
            self.is_alive
        }
    }
    fn get_neighbor(&self) -> i64 {
        let mut alive = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    // 排除当前点本身
                    continue;
                }
                if (self.x == 0 && dx == -1) || (self.y == 0 && dy == -1) {
                    continue;
                }
                let nx = self.x as isize + dx;
                let ny = self.y as isize + dy;
                let world_borrowed = self.world.borrow();
                if world_borrowed[nx as usize][ny as usize] {
                    alive += 1;
                }
            }
        }
        alive
    }
}

impl World {
    fn new(x: usize, y: usize, seed: Vec<Point>, dead: String, alive: String) -> Self {
        let world = vec![vec![false; y + 1]; x + 1];
        Self {
            x,
            y,
            seed,
            world: Rc::new(RefCell::new(world)),
            dead,
            alive,
            stdout: stdout(),
        }
    }
    fn init(&mut self) {
        for y in 0..self.y {
            for x in 0..self.x {
                let point = Point::new_xy(x, y);
                let mut world_borrowed = self.world.borrow_mut();
                if self.seed.contains(&point) {
                    world_borrowed[point.x][point.y] = true;
                    execute!(
                        self.stdout,
                        MoveTo(x as u16 * 2, y as u16),
                        Print(&self.alive)
                    );
                } else {
                    world_borrowed[point.x][point.y] = false;
                    execute!(
                        self.stdout,
                        MoveTo(x as u16 * 2, y as u16),
                        Print(&self.dead)
                    );
                }
            }
        }
    }
    fn clean(&self) {
        std::io::stdout()
            .execute(terminal::Clear(ClearType::All))
            .expect("清屏失败");
    }
    fn update(&mut self) {
        // 首先，计算所有点的下一个状态，但不立即修改world, 不仅保证了rust的借用规则，而且还避免了更新的状态影响原本的值
        let mut updates = Vec::new();
        for y in 0..self.y {
            for x in 0..self.x {
                // 直接访问当前状态，避免在这里借用self.world
                let is_alive = self.world.borrow()[x][y];
                let point = Point::new(x, y, is_alive, Rc::clone(&self.world));
                let should_live = point.check_around();
                // 后续考虑只记录变化更改的point
                // 将更新存储为坐标和目标状态，而不是立即应用
                updates.push((x, y, should_live));
            }
        }
        // println!("updates: {:#?}", updates);

        // 然后，在所有读取完成后，更新world状态
        for (x, y, should_live) in updates {
            let mut world_borrowed = self.world.borrow_mut();
            world_borrowed[x][y] = should_live;
        }
    }

    fn draw(&mut self) -> bool {
        let mut alive = false;
        for y in 0..self.y {
            for x in 0..self.x {
                let is_alive = self.world.borrow()[x][y];
                if is_alive {
                    alive = true;
                    execute!(
                        self.stdout,
                        MoveTo(x as u16 * 2, y as u16),
                        Print(&self.alive)
                    );
                } else {
                    execute!(
                        self.stdout,
                        MoveTo(x as u16 * 2, y as u16),
                        Print(&self.dead)
                    );
                }
            }
        }
        alive
    }
}

fn make_seed(x: usize, y: usize, n: usize, basic_seed: Option<usize>) -> Vec<Point> {
    let mut rng_seed = rand::thread_rng();
    let mut begin_seed: usize = rng_seed.gen();
    if let Some(seed) = basic_seed {
        begin_seed = seed;
    }
    // 使用一个固定的种子
    println!("begin_seed:{}", begin_seed);
    let seed_bytes: [u8; 8] = begin_seed.to_ne_bytes(); // 将i
                                                        // 创建一个32字节的数组用作种子，开始时用0填充
    let mut seed = [0u8; 32];
    // 将seed_i64的字节复制到种子数组的开始位置
    seed[..8].copy_from_slice(&seed_bytes);

    let mut rng = StdRng::from_seed(seed);

    let mut seed = Vec::new();
    for _i in 0..n {
        let nx: i32 = rng.gen_range(0..x) as i32; // 生成一个0到9之间的整数
        let ny: i32 = rng.gen_range(0..y) as i32; // 生成一个0到9之间的整数
        let point = Point::new_xy(nx as usize, ny as usize);
        seed.push(point);
    }
    seed
}

#[derive(Parser, Debug)]
#[command(name = "My CLI Program")]
#[command(version = "1.0")]
#[command(author = "Your Name <email@example.com>")]
#[command(about = "生命游戏", long_about = None)]
struct Args {
    /// Sets the width of the program
    #[arg(long, value_name = "WIDTH", default_value_t = 70)]
    width: usize,

    /// Sets the height of the program
    #[arg(long, value_name = "HEIGHT", default_value_t = 70)]
    height: usize,

    /// Sets the total number of cells
    #[arg(long = "total-cell", value_name = "TOTAL_CELL", default_value_t = 2500)]
    total_cell: usize,

    /// Sets the basic seed for the program
    #[arg(long = "basic-seed", value_name = "BASIC_SEED")]
    basic_seed: Option<usize>,
    /// Sets the basic speed for the program
    #[arg(long, value_name = "speed", default_value_t = 100)]
    speed: usize,
    /// Sets the symbol for dead cells
    #[arg(long, value_name = "DEAD", default_value = " ")]
    dead: String,

    /// Sets the symbol for alive cells
    #[arg(long, value_name = "ALIVE", default_value = "X")]
    alive: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let width = args.width;
    let height = args.height;
    let total_cell = args.total_cell;
    let basic_seed = args.basic_seed;
    let alive = args.alive;
    let dead = args.dead;
    let seed = make_seed(width, height, total_cell, basic_seed);
    let mut cound = 0;
    let mut world = World::new(width, height, seed, dead, alive);
    world.clean();
    world.init();
    terminal::enable_raw_mode().expect("TODO: panic message");
    let mut pasue = false;
    loop {
        let last_tick = Instant::now();
        let tick_rate = Duration::from_millis(args.speed as u64);
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('p') => pasue = true,
                    _ => {}
                }
            }
        }
        if pasue {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('p') {
                    pasue = false; // 切换回非暂停状态
                    continue; // 继续执行下一个循环迭代
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            world.update();
            if world.draw() {
                cound += 1;
                println!(
                    "\n\n q键退出，p键暂停，暂停后输入任意键，手动执行， 当前迭代count:{} 次",
                    cound
                );
            } else {
                break;
            }
        }
    }
    println!("\n 总共迭代 count:{} 次", cound);
    disable_raw_mode().expect("TODO: panic message");
    Ok(())
}
