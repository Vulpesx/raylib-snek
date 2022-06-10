use raylib::prelude::*;

const HEAD_COLOR: Color = Color::new(178, 178, 178, 255);
const SEGMENT_COLOR: Color = Color::new(76, 76, 76, 255);
const FOOD_COLOR: Color = Color::new(255, 0, 255, 255);

const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 600;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

struct Game<const WIDTH: u32 = 10, const HEIGHT: u32 = 10> {
    segments: Vec<Position>,
    food: Vec<Position>,
    tail: Position,
    dir: Direction,
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl<const WIDTH: u32, const HEIGHT: u32> Game<WIDTH, HEIGHT> {
    fn new(rl: RaylibHandle, thread: RaylibThread) -> Self {
        Self {
            segments: Vec::new(),
            food: Vec::new(),
            tail: Position { x: 0, y: 0 },
            dir: Direction::Up,
            rl,
            thread,
        }
    }

    fn init(&mut self) {
        self.segments.clear();
        self.food.clear();
        self.segments.push(Position {
            x: WIDTH as i32 / 2,
            y: HEIGHT as i32 / 2,
        });
        self.segments.push(Position {
            x: WIDTH as i32 / 2,
            y: HEIGHT as i32 / 2,
        });
    }

    fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::GRAY);

        let tile_w = WINDOW_WIDTH / WIDTH as i32;
        let tile_h = WINDOW_HEIGHT / HEIGHT as i32;

        for pos in self.food.iter() {
            d.draw_rectangle(
                pos.x * tile_w + (tile_w - 26 / 2) - (tile_w / 2),
                pos.y * tile_h + (tile_h - 26 / 2) - (tile_h / 2),
                26,
                26,
                FOOD_COLOR,
            );
        }

        for (i, pos) in self.segments.iter().enumerate() {
            d.draw_rectangle(
                pos.x * tile_w + (tile_w - if i == 0 { 46 / 2 } else { 36 / 2 }) - (tile_h / 2),
                pos.y * tile_h + (tile_h - if i == 0 { 46 / 2 } else { 36 / 2 }) - (tile_h / 2),
                if i == 0 { 46 } else { 36 },
                if i == 0 { 46 } else { 36 },
                if i == 0 {HEAD_COLOR} else {SEGMENT_COLOR},
            );
        }

        d.draw_text(&format!("score: {}", self.segments.len() - 2), 0, 0, 20, Color::BLACK);
    }

    fn input(&mut self) {
        if let Some(k) = self.rl.get_key_pressed() {
            let d = match k {
                KeyboardKey::KEY_UP => Direction::Up,
                KeyboardKey::KEY_DOWN => Direction::Down,
                KeyboardKey::KEY_LEFT => Direction::Left,
                KeyboardKey::KEY_RIGHT => Direction::Right,
                _ => self.dir,
            };
            if d != self.dir.opposite() {
                self.dir = d;
            }
        }
    }

    fn movement(&mut self) {
        let pos = self.segments.iter().map(|e| *e).collect::<Vec<Position>>();
        let mut head_pos = pos[0];

        match self.dir {
            Direction::Up => head_pos.y -= 1,
            Direction::Down => head_pos.y += 1,
            Direction::Left => head_pos.x -= 1,
            Direction::Right => head_pos.x += 1,
        }
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= WIDTH
            || head_pos.y as u32 >= HEIGHT
        {
            self.init();
            return;
        }
        if pos.contains(&head_pos) {
            self.init();
            return;
        }
        for i in 1..self.segments.len() {
            self.segments[i] = pos[i - 1];
        }

        self.tail = *pos.last().unwrap();
        self.segments[0] = head_pos;
    }

    fn food(&mut self) {
        self.food.push(Position { x: get_random_value(0, WIDTH as i32 - 1), y: get_random_value(0, HEIGHT as i32 - 1)});
    }

    fn eat(&mut self) {
        for i in 0..self.food.len() {
            if self.food[i] == self.segments[0] {
                self.food.remove(i);
                self.segments.push(self.tail);
                break;
            }
        }
    }

    fn run(&mut self) {
        let start = std::time::Instant::now();
        let mut last_frame = 0.;

        let mut movement_acc = 0.;
        let movement_rate = 0.150;

        let mut food_acc = 0.;
        let food_rate = 1.;

        while !self.rl.window_should_close() {
            let curr_frame = start.elapsed().as_secs_f32();
            let delta = curr_frame - last_frame;

            self.input();

            if movement_acc >= movement_rate {
                self.eat();
                self.movement();
                movement_acc = 0.;
            } else {
                movement_acc += delta;
            }

            if food_acc >= food_rate {
                self.food();
                food_acc = 0.;
            } else {
                food_acc += delta;
            }

            self.render();

            last_frame = curr_frame;
        }
    }
}

fn main() {
    //window init
    let (rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Snake..!")
        .vsync()
        .msaa_4x()
        .build();

    let mut game: Game = Game::new(rl, thread);
    game.init();
    game.run();
}
