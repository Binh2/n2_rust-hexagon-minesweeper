use macroquad::rand;
use macroquad::{color, prelude::*};
mod global;

fn export_shape(filename: &str, draw_shape: impl Fn() -> ()) {
    let width = 64;
    let height = 64;

    let render_target = render_target(width, height);
    let camera = Camera2D {
        render_target: Some(render_target.clone()),
        zoom: vec2(2.0 / width as f32, -2.0 / height as f32),
        target: vec2(0.0, 0.0),
        ..Default::default()
    };
    set_camera(&camera);

    clear_background(BLANK);
    draw_shape();
    set_default_camera();
    let texture = render_target.clone().texture;
    let image = texture.get_texture_data();
    image.export_png(format!("assets/generated/{}", filename).as_str());
}
fn export_num(num: &str, color: Color) {
    export_shape(format!("{}.png", num).as_str(), || {
        let TextDimensions { width, height, .. } =
            measure_text(num, None, global::FONT_SIZE as u16, 1.0);
        draw_text(num, -width / 2.0, height / 2.0, global::FONT_SIZE, color);
    });
}

#[macroquad::main("Hexagon Export")]
async fn main() {
    export_shape("clicked_cell.png", || {
        draw_hexagon(
            0.0,
            0.0,
            global::CELL_SIZE,
            global::CELL_BORDER,
            true,
            LIGHTGRAY,
            GRAY,
        );
    });
    export_shape("cell.png", || {
        draw_hexagon(
            0.0,
            0.0,
            global::CELL_SIZE,
            global::CELL_BORDER,
            true,
            WHITE,
            LIGHTGRAY,
        );
    });
    export_shape("bomb.png", || {
        draw_circle(0.0, 0.0, global::BOMB_SIZE, RED);
    });
    export_shape("flag.png", || {
        draw_triangle(vec2(16.0, -32.0), vec2(16.0, 32.0), vec2(-16.0, 0.0), RED);
    });
    export_num("1", BLUE);
    export_num("2", GREEN);
    export_num("3", RED);
    export_num("4", PINK);
    export_num("5", PURPLE);
    export_num("6", ORANGE);

    // clear_background(BLANK);
    // let texture = render_target.texture;
    // let image = texture.get_texture_data();
    // image.export_png("circle.png");

    // clear_background(BLANK);
    // draw_text("1", 0.0, 0.0, 64.0, RED);
    // let texture = render_target(width, height).texture;
    // let image = texture.get_texture_data();
    // image.export_png("1.png");
    // draw_line(
    //     40.0,
    //     40.0,
    //     100.0,
    //     200.0,
    //     15.0,
    //     color::hsl_to_rgb(200.0, 1.0, 1.0),
    // );
    // draw_rectangle(screen_w() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    // draw_circle(screen_w() - 30.0, screen_h() - 30.0, 15.0, YELLOW);

    // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

    // draw_hexagon(100.0, 100.0, 100.0, 2.0, true, color::RED, color::BLUE);
}
