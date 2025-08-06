use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent, KeyboardInput, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use std::time::{Duration, Instant};

trait CellBehavior {
    fn next_state(&self, alive_neighbors: u8) -> bool;
}

struct StandardCell;

impl CellBehavior for StandardCell {
    fn next_state(&self, alive_neighbors: u8) -> bool {
        match alive_neighbors {
            2 => true,
            3 => true,
            _ => false,
        }
    }
}

struct GameOfLife {
    grid: Vec<Vec<bool>>,
    cell: Box<dyn CellBehavior>,
}

impl GameOfLife {
    fn new(initial: Vec<Vec<bool>>) -> Self {
        Self {
            grid: initial,
            cell: Box::new(StandardCell),
        }
    }

    fn step(&mut self) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut next = vec![vec![false; cols]; rows];

        for r in 0..rows {
            for c in 0..cols {
                let alive = self.grid[r][c];
                let neighbors = self.alive_neighbors(r, c);
                next[r][c] = if alive {
                    self.cell.next_state(neighbors)
                } else {
                    neighbors == 3
                };
            }
        }
        self.grid = next;
    }

    fn alive_neighbors(&self, row: usize, col: usize) -> u8 {
        let dirs = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];
        let rows = self.grid.len() as isize;
        let cols = self.grid[0].len() as isize;
        let mut count = 0;
        for (dr, dc) in dirs.iter() {
            let nr = row as isize + dr;
            let nc = col as isize + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                if self.grid[nr as usize][nc as usize] {
                    count += 1;
                }
            }
        }
        count
    }

    fn draw(&self, frame: &mut [u8], cell_size: usize) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let width = cols * cell_size;
        let height = rows * cell_size;
        for y in 0..height {
            for x in 0..width {
                let cell_x = x / cell_size;
                let cell_y = y / cell_size;
                let idx = (y * width + x) * 4;
                let alive = self.grid[cell_y][cell_x];
                let color = if alive { [0, 0, 0, 255] } else { [255, 255, 255, 255] };
                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let initial = vec![
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, true,  false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, true,  false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, true,  false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, true, false, false],
        vec![false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false],
        vec![false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, true, false, false],
        vec![false, false, false, false, false, true, true, true, false, false, false, false, false, false, true, true, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
    ];
    let mut game = GameOfLife::new(initial);
    let cell_size = 19; // smaller cell size for larger boards
    let rows = game.grid.len();
    let cols = game.grid[0].len();
    let width = cols * cell_size;
    let height = rows * cell_size;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Game of Life")
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)
        .unwrap();
    let mut pixels = Pixels::new(width as u32, height as u32, SurfaceTexture::new(width as u32, height as u32, &window))?;

    let mut last_update = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                game.draw(pixels.frame_mut(), cell_size);
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::ExitWithCode(0);
                }
            }
            Event::MainEventsCleared => {
                if last_update.elapsed() >= Duration::from_millis(200) {
                    game.step();
                    window.request_redraw();
                    last_update = Instant::now();
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::ExitWithCode(0);
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), state: ElementState::Pressed, .. }, .. }, .. } => {
                *control_flow = ControlFlow::ExitWithCode(0);
            }
            _ => {}
        }
    });
}
