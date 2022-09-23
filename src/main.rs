use macroquad::prelude::*;
use macroquad::input::*;
use macroquad::window;


struct PlayerParticle {
    x: f32,
    y: f32,
    size: f32,
    deflate_speed: f32,
}
struct Player {
    x: f32,
    y: f32,
    speed: f32,
    particles: Vec<PlayerParticle>,
}

struct Egg{
    x: f32,
    y: f32,
    size: f32,
    color: Color,
}

impl Egg {
    fn draw(&self) {
        draw_circle(self.x, self.y, self.size, self.color);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "My game".to_owned(),
        fullscreen: true,
        window_height: 500,
        window_width: 500,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let pattern_size = 80.0;
    let mut pattern = DynamicPattern {
        max_size: 70.0,
        min_size: 40.0,
        offset: 20.0,
        direction: 1.0,
        size: 70.0,
        speed: 0.01,
        color: Color::new(1.0, 1.0, 1.0, 0.2),
    };


    let mut player = Player {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        speed: 5.0,
        particles: Vec::new(),
    };

    let mut egg = Egg{
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        size: 20.0,
        color: Color::new(1.0, 1.0, 1.0, 1.0),

    };
    // set window to fullscreen


    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Right) {
            player.x += player.speed;
        }
        if is_key_down(KeyCode::Left) {
            player.x -= player.speed;
        }
        if is_key_down(KeyCode::Up) {
            player.y -= player.speed;
        }
        if is_key_down(KeyCode::Down) {
            player.y += player.speed;
        }

        if is_key_down(KeyCode::Q){
            panic!();
        }

        pattern = draw_checkered_pattern(pattern);

        player.particles.push(PlayerParticle {
            x: player.x,
            y: player.y,
            size: 80.0,
            deflate_speed: 1.0,
        });

        egg.draw();

        for i in 0..player.particles.len() {
            let particle = &mut player.particles[i];
            particle.size -= particle.deflate_speed;
            draw_circle(particle.x, particle.y, particle.size, Color::new(0.8, 0.8, 0.8, 1.0));
        }

        // filter player particles by size greater than 0 and remove all particles smaller than 0
        player.particles.retain(|particle| particle.size > 2.0);

        // print player particles length
        draw_text(&format!("Particles: {}", player.particles.len()), 50.0, 50.0, 50.0, WHITE);
        // draw fps on the screen
        draw_text(&format!("FPS: {}", get_fps()), 50.0, 100.0, 50.0, WHITE);
        draw_circle(player.x, player.y, 80.0, Color::new(0.8, 0.8, 0.8, 1.0));

        next_frame().await
    }
}




struct DynamicPattern {
    max_size: f32,
    min_size: f32,
    offset: f32,
    direction: f32,
    size: f32,
    speed: f32,
    color: Color,
}

impl DynamicPattern {
    fn next_color(&mut self){
        if self.color == PURPLE{
            self.color = BLUE;
        } else{
            self.color = PURPLE;
        }
    }

    fn next_size(&mut self){
        self.size = self.max_size  * get_time().sin() as f32;
    }
}

fn draw_checkered_pattern(mut pattern: DynamicPattern) -> DynamicPattern {
    for y in 0..(screen_height() / pattern.max_size) as i32 +2 {
        for x in 0..(screen_width() / pattern.max_size) as i32 +2 {
            
            // pattern.next_color();
            pattern.next_size();
            if (x+y) % 2 == 0{
                draw_rectangle(
                    x as f32 * pattern.max_size - pattern.size/2.0,
                    y as f32 * pattern.max_size - pattern.size/2.0,
                    pattern.size,
                    pattern.size,
                    pattern.color
                );
            }

        }
    }
    pattern
}