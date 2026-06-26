pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;


pub use constants::*;
pub use instructions::*;

declare_id!("F8dWQZHCnRQB7TjGebxzQTTkZbsjSmBtysLsvbgFgdhx");

#[program]
pub mod hackathon {
    enum DragonMood {
        Angry,
        Sad,
        Happy
    }

    enum Status {
        Available,
        Assigned,
        InFlight,
        Resting,
        Injured,
        Retired,
    }

    enum MissionStatus {
        Pending,
        InProgress,
        Completed,
        Failed
    }

    pub struct Dragon {
        name: String,
        fire_level: u16,
        mood: DragonMood,
        status: Status,
        damage_recorded: u64,
        created_at: i64,
    }

    pub struct Mission {
        dragon: Dragon,
        target: Village,
        risk_level: u16,
        status: MissionStatus,
    }

    pub struct Village {
        name: String,
        damage: u64,
        incidents: Vec<String>,
    }

   
}
