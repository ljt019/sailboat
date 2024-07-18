use macroquad::prelude::*;
use std::io::{self, ErrorKind};
use std::net::UdpSocket;

// host ip - 192.168.1.161:5656

const MAX_VALUE: u16 = 33400;
const MID_VALUE: u16 = MAX_VALUE / 2;
const MIN_VALUE: u16 = 2000;
const DEAD_ZONE: u16 = 3000;
const SCROLL_SPEED_MULTIPLIER: f32 = 2.5; // New constant to adjust scroll speed

// Include the image file directly in the binary
const SKY_IMAGE: &[u8] = include_bytes!("../assets/big_star_map.png");

struct SkyImage {
    texture: Texture2D,
    offset: f32,
}

impl SkyImage {
    async fn new() -> Self {
        // Load the texture from the included bytes
        let sky_texture = Texture2D::from_file_with_format(SKY_IMAGE, None);

        SkyImage {
            texture: sky_texture,
            offset: 0.0,
        }
    }

    fn draw(&self) {
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

    fn update(&mut self, sensor_value: u16) {
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

struct SensorServer {
    value: u16,
    udp_socket: UdpSocket,
}

impl SensorServer {
    fn new() -> Self {
        let udp_socket = UdpSocket::bind("0.0.0.0:5656").expect("Could not bind to address");

        udp_socket
            .set_nonblocking(true)
            .expect("Could not set non-blocking mode");

        SensorServer {
            value: 0,
            udp_socket: udp_socket,
        }
    }

    fn listen_for_new_data(&mut self) {
        let mut buf: [u8; 10] = [0; 10];
        match self.udp_socket.recv_from(&mut buf) {
            Ok((amt, _src)) => {
                let buf = std::str::from_utf8(&buf[..amt]).unwrap().trim();
                if let Ok(value) = buf.parse::<u16>() {
                    println!("Received value: {}", value);

                    self.value = value;
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // No data available right now, continue the loop
            }
            Err(e) => {
                eprintln!("Encountered an error: {}", e);
            }
        }
    }

    fn get_value(&self) -> u16 {
        self.value
    }
}

#[macroquad::main("Potentiometer Sky Scrolling")]
async fn main() {
    set_fullscreen(true);

    let mut sensor = SensorServer::new();

    let mut sky = SkyImage::new().await;

    loop {
        // listen to the sensor data
        sensor.listen_for_new_data();

        // adjust the sky based on the sensor data
        sky.update(sensor.get_value());

        // Clear the background
        clear_background(BLACK);

        // draw the sky
        sky.draw();

        next_frame().await;
    }
}
