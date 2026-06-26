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
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
