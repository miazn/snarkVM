// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use rdkafka::{
    producer::{BaseProducer, BaseRecord, Producer},
    ClientConfig,
};
use std::{thread, time::Duration};


const MAX_QUEUE_SIZE: usize = 1000; 

pub struct KafkaProducer {
    producer: Arc<BaseProducer>,
    queue: Arc<Mutex<Vec<(String, String, String)>>>,   // Tuple of key, value, topic
}

impl Default for KafkaProducer {
    fn default() -> Self {
        Self::new()
    }
}

impl KafkaProducer {
    pub fn new() -> Self {
        let producer: BaseProducer =
            ClientConfig::new().set("bootstrap.servers", "localhost:9092").create().expect("Producer creation error");
            let kp = KafkaProducer {
                producer: Arc::new(producer),
                queue: Arc::new(Mutex::new(Vec::new())),
            };
            kp.start_background_emitter();
            kp
    }

    // original kafka background emitter function

    //pub fn start_background_emitter(&self) {
    //    let producer = Arc::new(self.producer.clone());
    //    let queue = Arc::clone(&self.queue);

    //    thread::spawn(move || {
    //        loop {
    //            {
    //                let queue_guard = queue.lock().unwrap();

    //                if queue_guard.len() >= MAX_QUEUE_SIZE {
    //                    drop(queue_guard); // Drop the lock before calling emit_queue
    //                    Self::emit_queue_for_thread(&producer, &queue);
    //                }
    //            }
    //            thread::sleep(Duration::from_secs(10));
    //        }
    //    });
    //}

    pub fn start_background_emitter(&self) {
        let producer = Arc::new(self.producer.clone());
        let queue = Arc::clone(&self.queue);
    
        thread::spawn(move || {
            loop {
                {
                    let queue_guard = queue.lock().unwrap();
                    if queue_guard.len() >= MAX_QUEUE_SIZE || !queue_guard.is_empty() {
                        drop(queue_guard); // Drop the lock before calling emit_queue
                        Self::emit_queue_for_thread(&producer, &queue);
                    }
                }
                thread::sleep(Duration::from_secs(10));
            }
        });
    }
    

    // Enqueue a message to the internal buffer
    //pub fn enqueue(&mut self, key: String, value: String, topic: String) {
    //    let mut queue = self.queue.lock().unwrap();
    //    queue.push((key, value, topic));
    //}

    pub fn enqueue(&mut self, key: String, value: String, topic: String) {
        match self.queue.lock() {
            Ok(mut queue) => {
                queue.push((key, value, topic));
            },
            Err(poisoned_error) => {
                println!("Failed to lock the queue due to a poisoned mutex. Error: {:?}", poisoned_error);
            }
        }
    }

    fn emit_queue_for_thread(producer: &BaseProducer, queue: &Arc<Mutex<Vec<(String, String, String)>>>) {
        let mut queue_guard = queue.lock().unwrap();

        while let Some((key, value, topic)) = queue_guard.pop() {
            producer
                .send(BaseRecord::to(&topic).key(&key).payload(&value))
                .expect("failed to send message");
        }

        let _ = producer.flush(Duration::from_secs(10));  // Adjust the timeout as necessary
    }


}



lazy_static! {
    pub static ref KAFKA_PRODUCER: KafkaProducer = KafkaProducer::new();
}
