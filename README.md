# Sailboat Exhibit

An interactive sailboat exhibit featuring a scrolling star map display, controlled by a rudder-mounted potentiometer. This project is part of a science museum installation.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)

## Overview

This repository contains the Rust code for the display system of the Sailboat Exhibit. The exhibit allows visitors to control a scrolling star map by manipulating a sailboat rudder, providing an interactive celestial navigation experience.

## Features

- Real-time star map scrolling based on potentiometer input
- UDP communication for sensor data transmission
- Fullscreen display using Macroquad game engine
- Smooth, wraparound scrolling of large star map image

## Architecture

The project consists of three main Rust modules:

1. `main.rs`: Entry point and main loop
2. `sensor_server.rs`: UDP server for receiving sensor data
3. `sky_image.rs`: Star map image handling and rendering

### Hardware Components

- Display System: PC running Rust application
- Sensor System: Raspberry Pi Pico W with attached potentiometer
- Exhibit: Large sailboat model with manipulable rudder

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/your-username/sailboat-exhibit.git
   cd sailboat-exhibit
   ```

2. Ensure you have Rust and Cargo installed. If not, install from [rustup.rs](https://rustup.rs/).

3. Install dependencies:
   ```
   cargo build
   ```

## Usage

1. Start the Rust application:

   ```
   cargo run --release
   ```

2. Ensure the Raspberry Pi Pico W is running the sensor code and sending UDP packets to the correct IP and port.

3. The application will start in fullscreen mode. Use the rudder to control the star map scrolling.

## Configuration

Key constants in `sky_image.rs`:

- `MAX_VALUE`: Maximum potentiometer value (default: 33400)
- `MIN_VALUE`: Minimum potentiometer value (default: 2000)
- `DEAD_ZONE`: No-scroll range around center (default: 3000)
- `SCROLL_SPEED_MULTIPLIER`: Scroll speed adjustment (default: 2.5)

Modify these values to fine-tune the exhibit's behavior.
