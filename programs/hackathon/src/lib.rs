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
    
    pub fn close(_ctx: Context<CloseCounter>) -> Result<()> {
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count.checked_sub(1).unwrap();
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count.checked_add(1).unwrap();
        Ok(())
    }

    pub fn initialize(_ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
        ctx.accounts.counter.count = value.clone();
        Ok(())
    }
    
    #[derive(Accounts)]
    pub struct InitializeCounter<'info> {
        #[account(mut)]
        pub payer: Signer<'info>,
    
        #[account(
      init,
      space = 8 + Counter::INIT_SPACE,
      payer = payer
        )]
        pub counter: Account<'info, Counter>,
        pub system_program: Program<'info, System>,
    }
    #[derive(Accounts)]
    pub struct CloseCounter<'info> {
        #[account(mut)]
        pub payer: Signer<'info>,
    
        #[account(
      mut,
      close = payer, // close account and return lamports to payer
        )]
        pub counter: Account<'info, Counter>,
    }
    
    #[derive(Accounts)]
    pub struct Update<'info> {
        #[account(mut)]
        pub counter: Account<'info, Counter>,
    }
    
    #[account]
    #[derive(InitSpace)]
    pub struct Counter {
        count: u8,
    }
}

   

