use crate::framebuffer::Framebuffer;
use rand::Rng;

pub struct Game {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    tick_counter: usize,
    current_color: u32,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Game {
            width,
            height,
            cells: vec![false; width * height],
            tick_counter: 0,
            current_color: 0xFF4000, // Color inicial
        }
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = true;
        }
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x]
        } else {
            false
        }
    }

    pub fn update(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.alive_neighbors(x, y);
                let idx = y * self.width + x;
                if self.cells[idx] {
                    new_cells[idx] = alive_neighbors == 2 || alive_neighbors == 3;
                } else {
                    new_cells[idx] = alive_neighbors == 3;
                }
            }
        }

        self.cells = new_cells;
        self.tick_counter += 1;

        if self.tick_counter % 5 == 0 {
            let mut rng = rand::thread_rng();
            self.current_color = rng.gen_range(0x000000..0xFFFFFF);
        }
    }

    fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in [-1, 0, 1].iter().cloned() {
            for dx in [-1, 0, 1].iter().cloned() {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && nx < self.width as isize && ny < self.height as isize {
                    if self.is_alive(nx as usize, ny as usize) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        framebuffer.clear();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_alive(x, y) {
                    framebuffer.set_current_color(self.current_color);
                    framebuffer.point(x, y);
                }
            }
        }
    }
}
