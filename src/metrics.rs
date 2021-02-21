pub struct ProcessingMetrics {
    pub total_commands: u32,
    pub total_processing_time: f32
}

impl ProcessingMetrics {
    pub fn new() -> Self {
        Self {
            total_commands: 0,
            total_processing_time: 0f32
        }
    }

    pub fn add(&mut self, time: f32) {
        self.total_commands += 1;
        self.total_processing_time += time;
    }

    pub fn avg(&self) -> f32 {
        self.total_processing_time / (self.total_commands as f32)
    }
}

pub struct GlobalMetrics {
    pub processing: ProcessingMetrics
}

impl GlobalMetrics {
    pub fn new() -> Self {
        Self {
            processing: ProcessingMetrics::new()
        }
    }
}