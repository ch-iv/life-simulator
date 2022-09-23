use macroquad::prelude::*;


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
    size: f32,
    particles: Vec<PlayerParticle>,
    collision_box: Rect,
}

impl Player{
    fn new(x: f32, y: f32, speed: f32, size: f32) -> Self{
        let egg_collision_box = Rect::new(x, y, size, size);
        Self {x, y, speed, size, particles: vec![], collision_box: egg_collision_box }
    }
}

struct Egg{
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    collision_box: Rect,
}

impl Egg {
    fn new(x: f32, y: f32, size: f32, color: Color) -> Self{
        let egg_collision_box = Rect::new(x, y, size, size);
        Self {x, y, size, color, collision_box: egg_collision_box }
    }

    fn regenerate(&mut self){
        self.x = rand::gen_range(0.0+self.size/2.0, screen_width()-self.size/2.0);
        self.y = rand::gen_range(0.0+self.size/2.0, screen_height()-self.size/2.0);
        self.collision_box.x = self.x;
        self.collision_box.y = self.y;
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.size, self.color);
    }
}

struct DynamicPattern {
    max_size: f32,
    size: f32,
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
    let mut score: i64 = 0;

    let mut pattern = DynamicPattern {
        max_size: 70.0,
        size: 70.0,
        color: Color::new(1.0, 1.0, 1.0, 0.2),
    };


    let mut player = Player::new (screen_width() / 2.0, screen_height() / 2.0, 5.0, 80.0);

    let mut egg = Egg::new(screen_width() / 2.0, screen_height() / 2.0, 20.0, Color::new(1.0, 1.0, 1.0, 1.0));


    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Right) {
            player.x += player.speed;
            player.collision_box.x = player.x;
        }
        if is_key_down(KeyCode::Left) {
            player.x -= player.speed;
            player.collision_box.x = player.x;
        }
        if is_key_down(KeyCode::Up) {
            player.y -= player.speed;
            player.collision_box.y = player.y;
        }
        if is_key_down(KeyCode::Down) {
            player.y += player.speed;
            player.collision_box.y = player.y;
        }

        if is_key_down(KeyCode::Q){
            panic!();
        }

        pattern = draw_checkered_pattern(pattern);

        player.particles.push(PlayerParticle {
            x: player.x,
            y: player.y,
            size: player.size,
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
        // draw_text(&format!("Particles: {}", player.particles.len()), 50.0, 50.0, 50.0, WHITE);
        // draw fps on the screen
        draw_text(&format!("FPS: {}", get_fps()), 50.0, 100.0, 50.0, WHITE);
        // draw_text(&format!("Colliding: {}", player.collision_box.overlaps(&egg.collision_box)), 50.0, 150.0, 50.0, WHITE);

        if player.collision_box.overlaps(&egg.collision_box){
            score += 1;
            egg.regenerate();
        }

        draw_text(&format!("Score: {}", score), 50.0, 150.0, 50.0, WHITE);

        draw_circle(player.x, player.y, player.size, Color::new(0.8, 0.8, 0.8, 1.0));

        next_frame().await
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