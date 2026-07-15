use macroquad::{color, prelude::*};
use macroquad::rand;

#[derive(Clone)]
enum Cell {
    Bomb(bool),
    Num(bool, u8),
}
struct Hexagon {
    x: f32,
    y: f32,
    size: f32,
}
impl Hexagon {
    fn contains_point(&self, p: (f32, f32)) -> bool {
        if ((self.x - p.0) * (self.x - p.0) + (self.y - p.1) * (self.y - p.1)).sqrt() < self.size * 3.0_f32.sqrt()/2.0 - 1.0 {
            return true;
        }
        false
    }
}

fn add_bomb(grid: &mut Vec<Vec<Cell>>, num_of_bomb: i32) {
    for i in 0..num_of_bomb {
        let row = rand::gen_range(0, 15);
        let col = rand::gen_range(0, 15);

        grid[row][col] = Cell::Bomb(false);
    }
}
fn update_neighbor(grid: &mut Vec<Vec<Cell>>) {

}
fn cell_clicked(cell_size: f32, cell_border: f32, x_offset: f32, y_offset: f32) -> (usize, usize) {

    (0, 0)
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let max_row = 16;
    let max_col = 16;
    let num_of_bomb = 16;

    let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Num(false, 0); max_row]; max_col];
    add_bomb(&mut grid, num_of_bomb);
    let cell_size = 20.0;
    let cell_border = 2.0;
    let x_offset = cell_size * 3.0_f32.sqrt()/2.0 + cell_border;
    let y_offset = cell_size + cell_border;
    let mut collision_map: Vec<Vec<Hexagon>> = vec![];

    for i in 0..16 {
        collision_map.push(vec![]);
        for j in 0..16 {
            let x = ((i) as f32) * cell_size * 3.0_f32.sqrt() + i as f32 * cell_border + ((j % 2 == 1) as i32 as f32) * (cell_size + cell_border/2.0) * 3.0_f32.sqrt() / 2.0 + x_offset;
            let y = ((j) as f32) * cell_size * 3.0/2.0        + j as f32 * cell_border + y_offset;
            collision_map[i].push(Hexagon { x, y, size: cell_size + cell_border });
        }
    }

    loop {
        clear_background(color::BEIGE);

        for i in 0..16 {
            for j in 0..16 {
                let x = ((i) as f32) * cell_size * 3.0_f32.sqrt() + i as f32 * cell_border + ((j % 2 == 1) as i32 as f32) * (cell_size + cell_border/2.0) * 3.0_f32.sqrt() / 2.0 + x_offset;
                let y = ((j) as f32) * cell_size * 3.0/2.0        + j as f32 * cell_border + y_offset;
                let is_hovering = collision_map[i][j].contains_point(mouse_position());
                draw_hexagon(
                    x, 
                    y, 
                    cell_size, 
                    cell_border, 
                    true, 
                    color::BLACK, 
                    if is_hovering { color::GRAY } else { color::WHITE });
            }
        }

        // draw_line(
        //     40.0,
        //     40.0,
        //     100.0,
        //     200.0,
        //     15.0,
        //     color::hsl_to_rgb(200.0, 1.0, 1.0),
        // );
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        // draw_hexagon(100.0, 100.0, 100.0, 2.0, true, color::RED, color::BLUE);

        next_frame().await
    }
}
