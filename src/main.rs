use macroquad::{
    prelude::*,
    ui::{root_ui, widgets},
};
use rand::gen_range;

#[macroquad::main(window_conf)]
async fn main() {
    let mut dir = Direction::RIGHT;
    let (init_head_x, init_head_y) = generate_rand_cords(500.0, 300.0);
    let (init_berry_x, init_berry_y) = generate_rand_cords(500.0, 300.0);
    let mut head = Rect::new(init_head_x, init_head_y, 20.0, 20.0);
    let mut berry = Rect::new(init_berry_x, init_berry_y, 20.0, 20.0);
    let mut segments = [Rect::new(0.0, 0.0, 20.0, 20.0); 256];
    let mut points = 0;
    let mut frame_count = 0;
    let mut move_interval = 10;
    let mut game_loop = true;

    loop {

        if game_loop {
            clear_background(WHITE);

            draw_borders();

            draw_text(format!("{}", points).as_str(), 300.0, 200.0, 50.0, BLACK);

            frame_count += 1;

            let key = get_last_key_pressed();

            match key {
                Some(KeyCode::Up) if dir != Direction::DOWN => dir = Direction::UP,
                Some(KeyCode::Right) if dir != Direction::LEFT => dir = Direction::RIGHT,
                Some(KeyCode::Down) if dir != Direction::UP => dir = Direction::DOWN,
                Some(KeyCode::Left) if dir != Direction::RIGHT => dir = Direction::LEFT,
                _ => (),
            }

            if frame_count >= move_interval {
                frame_count = 0;

                let last_head = head;

                match dir {
                    Direction::UP => head.y += -21.0,
                    Direction::DOWN => head.y += 21.0,
                    Direction::RIGHT => head.x += 21.0,
                    Direction::LEFT => head.x += -21.0,
                }

                for n in (1..points).rev() {
                    segments[n].x = segments[n - 1].x;
                    segments[n].y = segments[n - 1].y;
                }

                segments[0].x = last_head.x;
                segments[0].y = last_head.y;
            }

            if head.overlaps(&berry) {
                let (new_berry_x, new_berry_y) = generate_rand_cords(500.0, 300.0);
                berry.x = new_berry_x;
                berry.y = new_berry_y;
                points += 1;

                if move_interval > 4 {
                    move_interval -= 1;
                }
            }

            draw_rectangle(head.x, head.y, 20.0, 20.0, GREEN);
            draw_rectangle(berry.x, berry.y, 20.0, 20.0, RED);

            if head.y > 400.0 || head.y < 0.0 || head.x > 600.0 || head.x < 0.0 {
                game_loop = false;
            }

            for n in 0..points {
                draw_rectangle(segments[n].x, segments[n].y, 20.0, 20.0, GRAY);
                if head.overlaps(&segments[n]) {
                    game_loop = false;
                }
            }
        } else {
            clear_background(BLACK);

            draw_borders();

            let retry = widgets::Button::new("retry?")
                .position(Vec2 { x: 280.0, y: 200.0 })
                .ui(&mut root_ui());

            if retry {
                points = 0;
                game_loop = true;
                let (init_head_x, init_head_y) = generate_rand_cords(500.0, 300.0);
                let (init_berry_x, init_berry_y) = generate_rand_cords(500.0, 300.0);
                head.x = init_head_x;
                head.y = init_head_y;
                berry.x = init_berry_x;
                berry.y = init_berry_y;
                move_interval = 10;
            }
        }

        next_frame().await
    }
}

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

fn generate_rand_cords(max_x: f32, max_y: f32) -> (f32, f32) {
    (gen_range(0.0, max_x), gen_range(0.0, max_y))
}

fn draw_borders() {
    draw_line(0.0, 0.0, 600.0, 0.0, 10.0, BLACK);
    draw_line(0.0, 400.0, 600.0, 400.0, 10.0, BLACK);
    draw_line(600.0, 400.0, 600.0, 0.0, 10.0, BLACK);
    draw_line(0.0, 0.0, 0.0, 400.0, 10.0, BLACK);
}

fn window_conf() -> Conf {
    Conf {
        fullscreen: false,
        window_title: "snake".to_string(),
        window_width: 600,
        window_height: 400,
        window_resizable: false,
        ..Default::default()
    }
}
