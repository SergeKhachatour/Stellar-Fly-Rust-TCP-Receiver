# Stellar-Fly-Rust-TCP-Receiver

A Rust-based TCP receiver designed to work with Unity projects, specifically for the Stellar Fly game or simulation.

## Overview

Stellar-Fly-Rust-TCP-Receiver is a high-performance TCP receiver implemented in Rust. It's designed to efficiently handle incoming data from a Unity-based game or simulation, particularly for the Stellar Fly project.

## Features

- Fast and efficient TCP data reception
- Optimized for use with Unity projects
- Low-latency communication
- Robust error handling and connection management

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/SergeKhachatour/Stellar-Fly-Rust-TCP-Receiver.git
   ```
2. Navigate to the project directory:
   ```
   cd Stellar-Fly-Rust-TCP-Receiver
   ```
3. Build the project:
   ```
   cargo build --release
   ```

## Usage

1. Start the TCP receiver:
   ```
   cargo run --release
   ```
2. In your Unity project, configure the TCP client to connect to the address and port used by this receiver.
3. Sample data received from the Unity project:
    Received data: *sensor1*GCHR6NVFBHK7XEGUSHVHSKWXAXVOKGHPKUZZEPY4XIB4TW3EB3GXVHO2,37.7749,-122.4194,30.0,50.0,90.0*sensor2*GDBDXBAQI5SZ7CCZPZT2UT4M72CJVQPV2WENRW2XBIJYX7IAUM6PCE3F,37.7750,-122.4195,31.0,51.0,91.0*
Latest block: Block { index: 1, timestamp: "2024-10-26T20:13:40.509123500+00:00", data: [SensorData { sensor_id: "sensor1", wallet_id: "GCHR6NVFBHK7XEGUSHVHSKWXAXVOKGHPKUZZEPY4XIB4TW3EB3GXVHO2", latitude: 37.7749, longitude: -122.4194, altitude: 30.0, speed: 50.0, direction: 90.0, timestamp: "2024-10-26T20:13:40.508993800+00:00" }, SensorData { sensor_id: "sensor2", wallet_id: "GDBDXBAQI5SZ7CCZPZT2UT4M72CJVQPV2WENRW2XBIJYX7IAUM6PCE3F", latitude: 37.775, longitude: -122.4195, altitude: 31.0, speed: 51.0, direction: 91.0, timestamp: "2024-10-26T20:13:40.509018600+00:00" }], previous_hash: "3e021f09267d2bfabc596fb200f6ea1d05c7ad9f77b519bd9047cebc34cc002f", hash: "f97061eadaec0951b7a4dc42b0a4d24fc9b1418f5f699f3093edf71d30d0c161" }
JSON Payload: [
  {
    "altitude": 30.0,
    "direction": 90.0,
    "latitude": 37.7749,
    "longitude": -122.4194,
    "sensor_id": "sensor1",
    "speed": 50.0,
    "timestamp": "2024-10-26T20:13:40.508993800+00:00",
    "wallet_id": "GCHR6NVFBHK7XEGUSHVHSKWXAXVOKGHPKUZZEPY4XIB4TW3EB3GXVHO2"
  },
  {
    "altitude": 31.0,
    "direction": 91.0,
    "latitude": 37.775,
    "longitude": -122.4195,
    "sensor_id": "sensor2",
    "speed": 51.0,
    "timestamp": "2024-10-26T20:13:40.509018600+00:00",
    "wallet_id": "GDBDXBAQI5SZ7CCZPZT2UT4M72CJVQPV2WENRW2XBIJYX7IAUM6PCE3F"
  }
]

## Configuration

More details coming soon.

## Integration with Unity

More details coming soon.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Contact

Serge Khachatour
