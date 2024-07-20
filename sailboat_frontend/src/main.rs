mod sensor_server;
mod sky_image;

use sensor_server::SensorServer;
use sky_image::SkyImage;

use macroquad::prelude::*;

#[macroquad::main("Potentiometer Sky Scrolling")]
async fn main() {
    set_fullscreen(true);

    show_mouse(false);

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
