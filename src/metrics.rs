use std::collections::HashMap;

use prometheus::{register_counter, register_int_counter, Counter, IntCounter};

pub struct CountableMetrics {
    pub total_commands: IntCounter,
    pub total_processing_time: Counter,
    pub events: IntCounter,
}

impl CountableMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            total_commands: register_int_counter!("commands", "Total number of commands executed")?,
            total_processing_time: register_counter!("processing_time", "Total processing time")?,
            events: register_int_counter!("events", "Total number of events")?,
        })
    }

    pub fn add(&mut self, time: f64) {
        self.total_commands.inc();
        self.total_processing_time.inc_by(time)
    }

    pub fn add_event(&mut self) {
        self.events.inc()
    }

    pub fn avg(&self) -> f32 {
        let processing_time = self.total_processing_time.get();
        let commands = self.total_commands.get();
        processing_time as f32 / commands as f32
    }
}

pub struct BtMessagesMetrics(pub HashMap<u64, u32>);

impl BtMessagesMetrics {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn sum(&self) -> u32 {
        self.0.iter().fold(0, |a, b| a + b.1)
    }
}

pub struct GlobalMetrics {
    /// Processing metrics
    pub processing: CountableMetrics,
    /// BadTranslator metrics
    ///
    /// Maps Guild ID to messages count
    pub bt_messages: BtMessagesMetrics,
}

impl GlobalMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            processing: CountableMetrics::new()?,
            bt_messages: BtMessagesMetrics::new(),
        })
    }
}
