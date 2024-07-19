use macroquad::prelude::*;

const SKY_IMAGE: &[u8] = include_bytes!("../assets/big_star_map.png");

const MAX_VALUE: u16 = 33400;
const MID_VALUE: u16 = MAX_VALUE / 2;
const MIN_VALUE: u16 = 2000;
const DEAD_ZONE: u16 = 3000;
const SCROLL_SPEED_MULTIPLIER: f32 = 2.5; // New constant to adjust scroll speed

pub struct SkyImage {
    texture: Texture2D,
    offset: f32,
}

impl SkyImage {
    pub async fn new() -> Self {
        // Load the texture from the included bytes
        let sky_texture = Texture2D::from_file_with_format(SKY_IMAGE, None);

        SkyImage {
            texture: sky_texture,
            offset: 0.0,
        }
    }

    pub fn draw(&self) {
        let texture_width = self.texture.width();
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Draw the first part of the texture
        let source_rect1 = Rect::new(self.offset, 0.0, screen_width, self.texture.height());
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width, screen_height)),
                source: Some(source_rect1),
                ..Default::default()
            },
        );

        // Draw the wrapping part of the texture if needed
        if self.offset + screen_width > texture_width {
            let overlap = (self.offset + screen_width) - texture_width;
            let source_rect2 = Rect::new(0.0, 0.0, overlap, self.texture.height());
            draw_texture_ex(
                &self.texture,
                screen_width - overlap,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(overlap, screen_height)),
                    source: Some(source_rect2),
                    ..Default::default()
                },
            );
        }
    }

    pub fn update(&mut self, sensor_value: u16) {
        let scroll_speed = self.calculate_scroll_speed(sensor_value);
        self.scroll(scroll_speed);
    }

    fn calculate_scroll_speed(&self, sensor_value: u16) -> f32 {
        if sensor_value > MID_VALUE + DEAD_ZONE {
            // Scroll left (negative speed)
            let range = MAX_VALUE - (MID_VALUE + DEAD_ZONE);
            let normalized_value = (sensor_value - (MID_VALUE + DEAD_ZONE)) as f32 / range as f32;
            -normalized_value * SCROLL_SPEED_MULTIPLIER // Use the new constant here
        } else if sensor_value < MID_VALUE - DEAD_ZONE {
            // Scroll right (positive speed)
            let range = MID_VALUE - DEAD_ZONE - MIN_VALUE;
            let normalized_value = ((MID_VALUE - DEAD_ZONE) - sensor_value) as f32 / range as f32;
            normalized_value * SCROLL_SPEED_MULTIPLIER // Use the new constant here
        } else {
            // Within dead zone, no scrolling
            0.0
        }
    }

    fn scroll(&mut self, speed: f32) {
        self.offset += speed;
        if self.offset >= self.texture.width() {
            self.offset -= self.texture.width();
        } else if self.offset < 0.0 {
            self.offset += self.texture.width();
        }
    }
}
