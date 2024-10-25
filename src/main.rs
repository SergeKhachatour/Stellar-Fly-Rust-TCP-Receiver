use std::sync::mpsc;
use std::thread;
use sha2::{Sha256, Digest};
use chrono::Utc;
//use mysql::*;
//use mysql::prelude::*;
use serde_json::json;
//use std::net::{TcpListener, TcpStream};
//use std::io::{BufReader, BufRead};
use tokio::net::TcpListener as TokioTcpListener;
use tokio::io::{AsyncBufReadExt, BufReader as TokioBufReader};
use std::error::Error;

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: String,
    data: Vec<SensorData>,
    previous_hash: String,
    hash: String,
}

#[derive(Debug, Clone)]
struct SensorData {
    sensor_id: String,
    wallet_id: String,  // New field
    latitude: f64,
    longitude: f64,
    altitude: f64,
    speed: f64,
    direction: f64,
    timestamp: String,
}

impl Block {
    fn new(index: u64, timestamp: String, data: Vec<SensorData>, previous_hash: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{:?}{}", index, timestamp, data, previous_hash));
        let hash = format!("{:x}", hasher.finalize());
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, Utc::now().to_rfc3339(), vec![], "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: Vec<SensorData>) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            Utc::now().to_rfc3339(),
            data,
            previous_block.hash.clone(),
        );
        self.chain.push(new_block);
    }

    // Add a method to get the latest block
    fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
}

// Comment out the save_block_to_db function
/*
async fn save_block_to_db(block: &Block, pool: &Pool) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        r"INSERT INTO blocks (index, timestamp, previous_hash, hash)
          VALUES (:index, :timestamp, :previous_hash, :hash)",
        params! {
            "index" => block.index,
            "timestamp" => &block.timestamp,
            "previous_hash" => &block.previous_hash,
            "hash" => &block.hash,
        },
    )?;

    for data in &block.data {
        conn.exec_drop(
            r"INSERT INTO sensor_data (block_index, sensor_id, latitude, longitude, altitude, speed, direction, timestamp)
              VALUES (:block_index, :sensor_id, :latitude, :longitude, :altitude, :speed, :direction, :timestamp)",
            params! {
                "block_index" => block.index,
                "sensor_id" => &data.sensor_id,
                "latitude" => data.latitude,
                "longitude" => data.longitude,
                "altitude" => data.altitude,
                "speed" => data.speed,
                "direction" => data.direction,
                "timestamp" => &data.timestamp,
            },
        )?;
    }

    Ok(())
}
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a TCP listener
    let listener = TokioTcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on 127.0.0.1:8080");

    // Accept connections and process them
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = process_stream(stream).await {
                eprintln!("Error processing stream: {}", e);
            }
        });
    }

    Ok(())
}

async fn process_stream(stream: tokio::net::TcpStream) -> Result<(), Box<dyn Error>> {
    let mut reader = TokioBufReader::new(stream);
    let mut data_str = String::new();

    // Read the data from the stream
    reader.read_line(&mut data_str).await?;

    // Remove any trailing newline characters
    data_str = data_str.trim_end().to_string();
    
    println!("Received data: {}", data_str);
    // Create a channel
    let (tx, rx) = mpsc::channel();
    
    // Spawn a thread to process the data
    let data_str_clone = data_str.clone();
    thread::spawn(move || {
        // Use data_str_clone inside the thread
        let parts: Vec<&str> = data_str_clone.split('*').filter(|&s| !s.is_empty()).collect();
        
        let mut json_data = Vec::new();
        for chunk in parts.chunks(2) {
            if chunk.len() == 2 {
                let sensor_id = chunk[0].to_string();
                let data_values: Vec<&str> = chunk[1].split(',').collect();
                if data_values.len() == 6 {  // Changed from 5 to 6
                    let sensor_data = json!({
                        "sensor_id": sensor_id,
                        "wallet_id": data_values[0],  // New field
                        "latitude": data_values[1].parse::<f64>().unwrap(),
                        "longitude": data_values[2].parse::<f64>().unwrap(),
                        "altitude": data_values[3].parse::<f64>().unwrap(),
                        "speed": data_values[4].parse::<f64>().unwrap(),
                        "direction": data_values[5].parse::<f64>().unwrap(),
                        "timestamp": Utc::now().to_rfc3339(),
                    });
                    json_data.push(sensor_data);
                }
            }
        }
        tx.send(json_data).unwrap();
    });
    
    // Receive the processed data
    let received_json_data = rx.recv().unwrap();
    
    // Convert the JSON data to SensorData structs
    let sensor_data: Vec<SensorData> = received_json_data.iter().map(|json| {
        SensorData {
            sensor_id: json["sensor_id"].as_str().unwrap().to_string(),
            wallet_id: json["wallet_id"].as_str().unwrap().to_string(),
            latitude: json["latitude"].as_f64().unwrap(),
            longitude: json["longitude"].as_f64().unwrap(),
            altitude: json["altitude"].as_f64().unwrap(),
            speed: json["speed"].as_f64().unwrap(),
            direction: json["direction"].as_f64().unwrap(),
            timestamp: json["timestamp"].as_str().unwrap().to_string(),
        }
    }).collect();
    
    // Initialize the blockchain
    let mut blockchain = Blockchain::new();
    
    // Add the received data as a new block
    blockchain.add_block(sensor_data.clone());
    
    // Print only the latest block
    println!("Latest block: {:?}", blockchain.latest_block());
    
    // Comment out the database connection and save operation
    /*
    // Database connection pool
    let url = "mysql://username:password@localhost:3306/sensor_blockchain";
    let pool = Pool::new(url)?;
    
    // Save the block to the database
    save_block_to_db(&blockchain.chain.last().unwrap(), &pool).await?;
    */
    
    // Print the JSON payload
    println!("JSON Payload: {}", serde_json::to_string_pretty(&received_json_data)?);
    
    Ok(())
}
