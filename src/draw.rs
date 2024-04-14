use piston_window:: {
    rectangle, text, types::Color, Context, G2d, Glyphs, Transformed
};

const BLOCK_SIZE: f64 = 10.0;
const FONT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

pub fn to_coordinate(coordinate: f64) -> f64 {
    coordinate * BLOCK_SIZE
}

pub fn draw_block(color: Color, position: [f64;2], con: &Context, g: &mut G2d) {
    let x = to_coordinate(position[0]);
    let y = to_coordinate(position[1]);

    rectangle(
        color,
        [x, y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    )
}

pub fn draw_rectangle(color: Color, position: [f64;2], width: f64, height: f64, con: &Context, g: &mut G2d) {
    let x = to_coordinate(position[0]);
    let y = to_coordinate(position[1]);

    rectangle(
        color,
        [x, y, BLOCK_SIZE*width, BLOCK_SIZE*height],
        con.transform,
        g,
    )
}

pub fn draw_text(text: &str, position: [f64; 2], con: &Context, g: &mut G2d, cache: &mut Glyphs) {
    let transform = con.transform.trans(to_coordinate(position[0]), to_coordinate(position[1]));

    text::Text::new_color(FONT_COLOR, 20)
        .draw(text, cache, &con.draw_state, transform, g)
        .unwrap();
}

