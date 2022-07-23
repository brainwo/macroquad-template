use macroquad::prelude::*;

#[macroquad::main("MyProject")] // TODO: name your project
async fn main() {
    loop {
        //  clear_background(RED);
        //
        //  draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //  draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //  draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //  draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
        //
        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });

        set_default_camera();

        next_frame().await
    }
}
