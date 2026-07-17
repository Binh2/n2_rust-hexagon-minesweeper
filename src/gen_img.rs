use macroquad::color::hsl_to_rgb;
use macroquad::rand;
use macroquad::{color, prelude::*};
mod global;

const BOLD_FONT: &[u8] = include_bytes!("../assets/minesweeper-sans.otf");
// const BOLD_FONT: &[u8] = include_bytes!("../assets/Fraunces_72pt_SuperSoft-Bold.ttf");
// const BOLD_FONT: &[u8] = include_bytes!("../assets/NotoSerif-ExtraBold.ttf");

fn export_shape(filename: &str, draw_shape: impl Fn() -> ()) {
    let width = global::CELL_WIDTH;
    let height = global::CELL_HEIGHT;

    let render_target = render_target(width as u32, height as u32);
    let camera = Camera2D {
        render_target: Some(render_target.clone()),
        zoom: vec2(2.0 / width, -2.0 / height),
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
fn export_num(num: &str, color: Color, bold_font: &Font, x_offset: f32, y_offset: f32) {
    export_shape(format!("{}.png", num).as_str(), || {
        let TextDimensions { width, height, .. } =
            measure_text(num, None, global::FONT_SIZE as u16, 1.0);
        draw_text_ex(
            num,
            -width / 2.0 + x_offset,
            height / 2.0 + y_offset,
            TextParams {
                font: Some(bold_font),
                font_size: global::FONT_SIZE as u16,
                color,
                ..Default::default()
            },
        );
    });
}

#[macroquad::main("Hexagon Export")]
async fn main() {
    export_shape("clicked_cell.png", || {
        draw_hexagon(0.0, 0.0, 60.0, 4.0, true, LIGHTGRAY, GRAY);
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

    let bold_font = load_ttf_font_from_bytes(BOLD_FONT).unwrap();
    let x_offset = -10.0;
    let y_offset = 8.0;
    export_num("1", Color::from_hex(0x004CFF), &bold_font, -2.0, y_offset);

    export_num(
        "2",
        Color::from_hex(0x26AD21),
        &bold_font,
        x_offset,
        y_offset,
    );
    export_num(
        "3",
        Color::from_hex(0xFF0000),
        &bold_font,
        x_offset,
        y_offset,
    );

    export_num(
        "4",
        Color::from_hex(0x284275),
        &bold_font,
        x_offset,
        y_offset,
    );

    export_num(
        "5",
        Color::from_hex(0x5E281B),
        &bold_font,
        x_offset,
        y_offset,
    );

    export_num(
        "6",
        Color::from_hex(0x2E979E),
        &bold_font,
        x_offset,
        y_offset,
    );
}
