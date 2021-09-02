use std::collections::HashMap;

pub struct CountableMetrics {
    pub total_commands: u32,
    pub total_processing_time: f32,
    pub events: u32
}

impl CountableMetrics {
    pub fn new() -> Self {
        Self {
            total_commands: 0,
            total_processing_time: 0f32,
            events: 0
        }
    }

    pub fn add(&mut self, time: f32) {
        self.total_commands += 1;
        self.total_processing_time += time;
    }

    pub fn add_event(&mut self) {
        self.events += 1;
    }

    pub fn avg(&self) -> f32 {
        self.total_processing_time / (self.total_commands as f32)
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
    pub fn new() -> Self {
        Self {
            processing: CountableMetrics::new(),
            bt_messages: BtMessagesMetrics::new(),
        }
    }
}
