use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopGuild {
    pub id: u64,
    pub name: String,
    pub count: u32,
}
impl TopGuild {
    pub fn new(id: u64, name: String, count: u32) -> Self {
        TopGuild { id, name, count }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopGuilds(pub Vec<TopGuild>);

impl TopGuilds {
    pub fn new() -> Self {
        TopGuilds(Vec::new())
    }

    pub fn add_guild(&mut self, id: u64, name: String, count: u32) -> () {
        if self.0.len() > 0 && count < self.0.last().unwrap().count {
            return;
        }

        let mut found = false;
        for guild in self.0.iter_mut() {
            if guild.id == id {
                guild.count = count;
                found = true;
            }
        }
        if !found {
            self.0.push(TopGuild::new(id, name, count));
        }
        self.sort();
        if self.0.len() > 25 {
            self.0.pop();
        }
    }

    pub fn sort(&mut self) -> () {
        self.0.sort_by(|a, b| b.count.cmp(&a.count));
    }
}
