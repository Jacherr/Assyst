use std::{collections::HashMap, sync::RwLock};

use prometheus::{
    register_counter, register_int_counter, register_int_gauge, register_int_gauge_vec, Counter,
    IntCounter, IntGauge, IntGaugeVec,
};

pub struct CountableMetrics {
    pub total_commands: IntCounter,
    pub total_processing_time: Counter,
    pub events: IntCounter,
    pub guilds: IntGauge,
    pub current_commands: IntGauge,
    pub latency: IntGaugeVec,
    pub cdn_files: IntGauge,
    pub cdn_size: IntGauge,
}

impl CountableMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            total_commands: register_int_counter!("commands", "Total number of commands executed")?,
            total_processing_time: register_counter!("processing_time", "Total processing time")?,
            events: register_int_counter!("events", "Total number of events")?,
            guilds: register_int_gauge!("guilds", "Total guilds")?,
            current_commands: register_int_gauge!(
                "current_commands",
                "Count of currently executing commands"
            )?,
            latency: register_int_gauge_vec!("latency", "Gateway latency", &["shard"])?,
            cdn_files: register_int_gauge!("cdn_files", "Total files stored in the CDN")?,
            cdn_size: register_int_gauge!("cdn_size", "Size in bytes of the CDN")?,
        })
    }
}

pub struct BtMessagesMetrics(RwLock<HashMap<u64, u32>>);

impl BtMessagesMetrics {
    pub fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    pub fn sum(&self) -> u32 {
        self.0.read().unwrap().iter().fold(0, |a, b| a + b.1)
    }

    pub fn inc(&self, guild_id: u64) {
        let mut map = self.0.write().unwrap();
        *map.entry(guild_id).or_insert(0) += 1;
    }
}

pub struct GlobalMetrics {
    /// Processing metrics
    processing: CountableMetrics,
    /// BadTranslator metrics
    ///
    /// Maps Guild ID to messages count
    bt_messages: BtMessagesMetrics,
}

impl GlobalMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            processing: CountableMetrics::new()?,
            bt_messages: BtMessagesMetrics::new(),
        })
    }

    #[inline]
    pub fn add_processing_time(&self, time: f64) {
        self.processing.total_commands.inc();
        self.processing.total_processing_time.inc_by(time)
    }

    #[inline]
    pub fn add_command(&self) {
        self.processing.current_commands.inc();
    }

    #[inline]
    pub fn delete_command(&self) {
        self.processing.current_commands.dec();
    }

    #[inline]
    pub fn add_event(&self) {
        self.processing.events.inc()
    }

    #[inline]
    pub fn get_events(&self) -> u64 {
        self.processing.events.get()
    }

    #[inline]
    pub fn add_guild(&self) {
        self.processing.guilds.inc();
    }

    #[inline]
    pub fn add_guilds(&self, amount: i64) {
        self.processing.guilds.add(amount)
    }

    #[inline]
    pub fn delete_guild(&self) {
        self.processing.guilds.dec();
    }

    #[inline]
    pub fn get_guild_count(&self) -> i64 {
        self.processing.guilds.get()
    }

    #[inline]
    pub fn avg_processing_time(&self) -> f32 {
        let processing_time = self.processing.total_processing_time.get();
        let commands = self.processing.total_commands.get();
        processing_time as f32 / commands as f32
    }

    #[inline]
    pub fn set_shard_latency(&self, shard: u64, latency: i64) {
        let stringed = shard.to_string();
        let counter = self.processing.latency.with_label_values(&[&stringed]);
        counter.set(latency);
    }

    #[inline]
    pub fn set_cdn_files(&self, files: i64) {
        self.processing.cdn_files.set(files);
    }

    #[inline]
    pub fn set_cdn_size(&self, size: i64) {
        self.processing.cdn_size.set(size);
    }
}
