use macroquad::{color, prelude::*};
use macroquad::rand;
mod global;

#[derive(Clone)]
enum Cell {
    Bomb(bool),
    Num(bool, u8),
}
impl Cell {
    fn set_show_flag(&mut self, new_show_flag: bool) {
        match self {
            Cell::Bomb(show_flag) => *show_flag = new_show_flag,
            Cell::Num(show_flag, _) => *show_flag = new_show_flag
        }
    }
    fn flip_show_flag(&mut self) {
        match self {
            Cell::Bomb(show_flag) => *show_flag = !*show_flag,
            Cell::Num(show_flag, _) => *show_flag = !*show_flag
        }
    }
    fn get_show_flag(&self) -> bool {
        return match self {
            Cell::Bomb(show_flag) => *show_flag,
            Cell::Num(show_flag, _) => *show_flag
        };
    }
    fn increment_num(&mut self) {
        match self {
            Cell::Bomb(_) => {},
            Cell::Num(_, num) => *num += 1
        }
    }
    fn get_num(&self) -> u8 {
        return match self {
            Cell::Bomb(_) => 0,
            Cell::Num(_, num) => *num
        }
    }
    fn is_bomb(&self) -> bool {
        return match self {
            Cell::Bomb(_) => true,
            Cell::Num(_, _) => false
        }
    }
}
struct Hexagon {
    x: f32,
    y: f32,
    size: f32,
    border: f32,
}
impl Hexagon {
    fn contains_point(&self, p: (f32, f32)) -> bool {
        // Removed the sqrt so it is faster
        if ((self.x - p.0) * (self.x - p.0) + (self.y - p.1) * (self.y - p.1)) < self.size * self.size * 3.0_f32/4.0 - self.border/2.0 {
            return true;
        }
        false
    }
}

fn add_bomb(grid: &mut Vec<Vec<Cell>>, max_row: usize, max_col: usize, num_of_bomb: &mut i32) {
    for _ in 0..*num_of_bomb {
        let row = rand::gen_range(0, max_row-1);
        let col = rand::gen_range(0, max_col-1);

        if grid[row][col].is_bomb() { *num_of_bomb -= 1; } 
        else { grid[row][col] = Cell::Bomb(false); };
    }
}
fn update_neighbor(grid: &mut Vec<Vec<Cell>>, max_row: usize, max_col: usize) {
    for i in 0..max_row {
        for j in 0..max_col {
            // println!("{} {}", i, j);
            if grid[i][j].is_bomb() {
                let col = j+((i%2==1) as usize);
                if j >= 1                                               { grid[i][j-1].increment_num(); };
                if j + 1 < max_col                                      { grid[i][j+1].increment_num(); };                
                if i >= 1          && col < max_col                     { grid[i-1][col].increment_num(); };
                if i >= 1          && 1 <= col && col < max_col+1       { grid[i-1][col-1].increment_num(); };
                if i + 1 < max_row && col < max_col                     { grid[i+1][col].increment_num(); };
                if i + 1 < max_row && 1 <= col && col < max_col+1       { grid[i+1][col-1].increment_num(); };
            }
        }
    }
}
fn reset_grid(grid: &mut Vec<Vec<Cell>>, is_visited_grid: &mut Vec<Vec<bool>>, is_visited_count: &mut usize, correct_flags_count: &mut usize, is_lose: &mut bool, max_row: usize, max_col: usize, num_of_bomb: &mut i32) {
    for i in 0..max_row {
        for j in 0..max_col {
            grid[i][j] = Cell::Num(false, 0);
            is_visited_grid[i][j] = false;
        }
    }

    *is_visited_count = 0;
    *correct_flags_count = 0;
    *is_lose = false;
    add_bomb(grid, max_row, max_col, num_of_bomb);
    update_neighbor(grid, max_row, max_col);
}

struct Button {
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    color: Color,
    text_content: String,
    text_font_size: f32,
    text_color: Color,
}

impl Button {
    pub fn new(cx: f32, cy: f32, w: f32, h: f32, color: Color, text_content: String, text_font_size: f32, text_color: Color) -> Button {
        Self {cx, cy, w, h, color, text_content, text_font_size, text_color}
    }
    fn draw(&self) {
        draw_rectangle(self.cx-self.w/2.0, self.cy-self.h/2.0, self.w, self.h, self.color);
        let TextDimensions {width: text_width, height: text_height, ..} = &measure_text(self.text_content.clone(), None, self.text_font_size.round() as u16, 1.0);
        draw_text(self.text_content.clone(), self.cx-text_width/2.0, self.cy+text_height/2.0, self.text_font_size, self.text_color);
    }
    fn get_rect(&self) -> Rect {
        Rect {
            x: self.cx-self.w/2.0,
            y: self.cy-self.h/2.0,
            w: self.w,
            h: self.h
        }
    }
    fn contains_point(&self, p: (f32, f32)) -> bool {
        self.get_rect().contains(p.into())
    }
}

fn visit(grid: &mut Vec<Vec<Cell>>, is_visited_grid: &mut Vec<Vec<bool>>, is_visited_count: &mut usize, i: usize, j: usize, max_row: usize, max_col: usize) {
    if !(i < max_row && j < max_col) { return; };
    if is_visited_grid[i][j] { return; };
    match grid[i][j] {
        Cell::Bomb(_) => { return; },
        Cell::Num(_, num) => {
            is_visited_grid[i][j] = true; 
            *is_visited_count += 1;
            grid[i][j].set_show_flag(false);
            if num > 0 { return; }
        }
    };

    let col = j+((i%2==1) as usize);
    if j >= 1               { visit(grid, is_visited_grid, is_visited_count, i, j-1, max_row, max_col); };
                              visit(grid, is_visited_grid, is_visited_count, i, j+1, max_row, max_col);
    if i >= 1               { visit(grid, is_visited_grid, is_visited_count, i-1, col, max_row, max_col); };
    if i >= 1 && col >= 1   { visit(grid, is_visited_grid, is_visited_count, i-1, col-1, max_row, max_col); };
                              visit(grid, is_visited_grid, is_visited_count, i+1, col, max_row, max_col);
    if col >= 1             { visit(grid, is_visited_grid, is_visited_count, i+1, col-1, max_row, max_col); };
}

#[macroquad::main("Hex Minesweeper")]
async fn main() {
    let max_row = 8;
    let max_col = 8;
    let mut num_of_bomb = 16;

    let cell_size = global::CELL_SIZE * global::SCALE;
    let cell_border = global::CELL_BORDER * global::SCALE;
    let cell_width = global::CELL_WIDTH * global::SCALE;
    let cell_height = global::CELL_HEIGHT * global::SCALE;
    // let bomb_size = global::BOMB_SIZE * global::SCALE;
    // let font_size = global::FONT_SIZE * global::SCALE;

    let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Num(false, 0); max_col]; max_row];
    add_bomb(&mut grid, max_row, max_col, &mut num_of_bomb);
    update_neighbor(&mut grid, max_row, max_col);
    let x_texture_offset =  -cell_width/2.0;
    let y_texture_offset = -cell_height/2.0;
    let x_offset = cell_size * 3.0_f32.sqrt()/2.0 + cell_border;
    let y_offset = cell_size + cell_border;
    let mut collision_map: Vec<Vec<Hexagon>> = vec![];

    for i in 0..max_row {
        collision_map.push(vec![]);
        for j in 0..max_col {
            let x = ((j) as f32) * cell_size * 3.0_f32.sqrt() + j as f32 * cell_border + ((i % 2 == 1) as i32 as f32) * (cell_size + cell_border/2.0) * 3.0_f32.sqrt() / 2.0 + x_offset;
            let y = ((i) as f32) * cell_size * 3.0/2.0        + i as f32 * cell_border + y_offset;
            collision_map[i].push(Hexagon { x, y, size: cell_size, border: cell_border });
        }
    }

    let reset_grid_button = Button::new( 500.0, 800.0, 120.0, 30.0, color::YELLOW, String::from("Reset grid"), 24.0, color::BLACK );
    let cell_texture = load_texture("assets/cell.png").await.expect("Failed to load texture");
    let clicked_cell_texture = load_texture("assets/clicked_cell.png").await.expect("Failed to load texture");
    let bomb_texture = load_texture("assets/bomb.png").await.expect("Failed to load texture");
    let wrong_bomb_texture = load_texture("assets/wrong_bomb.png").await.expect("Failed to load texture");
    let flag_texture = load_texture("assets/flag.png").await.expect("Failed to load texture");
    let wrong_flag_texture = load_texture("assets/wrong_flag.png").await.expect("Failed to load texture");
    let num_texture = [
        load_texture("assets/1.png").await.expect("Failed to load texture"),
        load_texture("assets/2.png").await.expect("Failed to load texture"),
        load_texture("assets/3.png").await.expect("Failed to load texture"),
        load_texture("assets/4.png").await.expect("Failed to load texture"),
        load_texture("assets/5.png").await.expect("Failed to load texture"),
        load_texture("assets/6.png").await.expect("Failed to load texture"),
    ];

    // For winners
    let mut is_visited_grid: Vec<Vec<bool>> = vec![vec![false; max_col]; max_row];
    let mut is_visited_count: usize = 0;
    let mut correct_flags_count: usize = 0;
    let mut is_win =  false;

    // For losers
    let mut is_lose = false;
    let mut bomb_clicked_i: Option<usize> = None;
    let mut bomb_clicked_j: Option<usize> = None; 
    
    loop {
        clear_background(color::BEIGE);

        for i in 0..max_row {
            for j in 0..max_col {
                let x = collision_map[i][j].x + x_texture_offset;
                let y = collision_map[i][j].y + y_texture_offset;
                let is_hovering = collision_map[i][j].contains_point(mouse_position());
                
                if !is_win && !is_lose {
                    if is_hovering && is_mouse_button_pressed(MouseButton::Right) { 
                        grid[i][j].flip_show_flag(); 
                        if grid[i][j].is_bomb() {
                            if grid[i][j].get_show_flag() {
                                correct_flags_count += 1;
                            } else {
                                correct_flags_count -= 1;
                            };
                        }
                    };
                    if is_hovering && is_mouse_button_pressed(MouseButton::Left) { 
                        if !grid[i][j].get_show_flag() { 
                            if grid[i][j].is_bomb() {
                                is_lose = true;
                                bomb_clicked_i = Some(i);
                                bomb_clicked_j = Some(j);
                            } else {
                                visit(&mut grid, &mut is_visited_grid, &mut is_visited_count, i, j, max_row, max_col); 
                            }
                        };
                    };
                    
                    if is_visited_grid[i][j] { 
                        draw_texture_ex(&clicked_cell_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() }); 
                    }
                    else { 
                        draw_texture_ex(&cell_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });
                    };
                    
                    if grid[i][j].get_show_flag() { 
                        draw_texture_ex(&flag_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });
                        
                        // draw_texture(&flag_texture, x-32.0, y-32.0, WHITE); 
                    };
                    
                    // if grid[i][j].is_bomb() {
                        //     draw_texture_ex(&bomb_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });
                    //     // draw_texture(&bomb_texture, x-32.0, y-32.0, WHITE);
                    // }
                    let num = grid[i][j].get_num();
                    if num != 0 && is_visited_grid[i][j] {
                        draw_texture_ex(&num_texture[(num-1) as usize], x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });
                    };
                    reset_grid_button.draw();

                    is_win = is_visited_count + correct_flags_count == max_row * max_col || 
                        is_visited_count + num_of_bomb as usize == max_row * max_col;
                    // println!("is_visited_count {} correct_flags_count {} num_of_bomb {}", is_visited_count, correct_flags_count, num_of_bomb);
                }
                
                // Check win/lose
                else if is_win { 
                    if grid[i][j].is_bomb() { draw_texture_ex(&cell_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() }); }
                    else { draw_texture_ex(&clicked_cell_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() }); }; 
                    let num = grid[i][j].get_num();
                    if num != 0 { draw_texture_ex(&num_texture[(num-1) as usize], x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() }); }
                    if grid[i][j].is_bomb() { draw_texture_ex(&flag_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() }); };
                    draw_text("You win!", 500.0, 500.0, 200.0, RED); 
                }
                else { // Lose 
                    if (grid[i][j].is_bomb() && !grid[i][j].get_show_flag()) || is_visited_grid[i][j] {
                        draw_texture_ex(&clicked_cell_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });
                    } else {
                        draw_texture_ex(&cell_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });
                    };

                    if grid[i][j].is_bomb() { 
                        if bomb_clicked_i == Some(i) && bomb_clicked_j == Some(j) { 
                            draw_texture_ex(&wrong_bomb_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() }); 
                        } else if !grid[i][j].get_show_flag() {
                            draw_texture_ex(&bomb_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });
                        }
                    };

                    if grid[i][j].get_show_flag() {
                        if grid[i][j].is_bomb() {
                            draw_texture_ex(&flag_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });                        
                        } else {
                            draw_texture_ex(&wrong_flag_texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });                        
                        }
                    }

                    
                    let num = grid[i][j].get_num();
                    if num != 0 && is_visited_grid[i][j] {
                        draw_texture_ex(&num_texture[(num-1) as usize], x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(cell_width, cell_height)), ..Default::default() });
                    };
                    draw_text("You lose!", 500.0, 500.0, 200.0, RED); 
                };
                
            }
            
        }
        if is_mouse_button_pressed(MouseButton::Left) && reset_grid_button.contains_point(mouse_position()) ||
        is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::R) { reset_grid(&mut grid, &mut is_visited_grid, &mut is_visited_count, &mut correct_flags_count, &mut is_lose, max_row, max_col, &mut num_of_bomb); };

        next_frame().await
    }

}

