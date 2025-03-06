use anchor_lang::prelude::*;

declare_id!("7AzUsuwMKP9XFpQaVt8Nt2XyAw8UHLWMYLnenxysV9Ce");

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn adding_task(ctx: Context<AddingTask>, text: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        if text.chars().count() > 400 {
            return Err(ErrorCode::TextTooLong.into());
        }

        task.id = task.key(); 
        task.author = *author.key;
        task.is_done = false;
        task.created_at = clock.unix_timestamp;
        task.updated_at = clock.unix_timestamp;
        task.text = text.clone();

        msg!("✅ Task Created by: {}", author.key);
        msg!("✅ Task Text: {}", text);
        msg!("✅ Task Created At: {}", clock.unix_timestamp);
        Ok(())
    }

    pub fn updating_task(ctx: Context<UpdatingTask>, is_done: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let clock = Clock::get().unwrap();

        task.is_done = is_done;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let clock = Clock::get().unwrap();

        task.is_done = true;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn toggle_completion_status(ctx: Context<ToggleTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let clock = Clock::get().unwrap();

        require!(
            task.author == *ctx.accounts.author.key,
            ErrorCode::Unauthorized
        );

        task.is_done = !task.is_done;
        task.updated_at = clock.unix_timestamp;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeletingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

#[account]
pub struct Task {
    pub id: Pubkey,      // Unique identifier for the task
    pub author: Pubkey,  // The account that owns the task
    pub is_done: bool,   // Whether the task is done or not
    pub text: String,    // The text of the task
    pub created_at: i64, // The timestamp when the task was created
    pub updated_at: i64, // The timestamp when the task was last updated
}

#[derive(Accounts)]
pub struct ToggleTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}


impl Task {
    pub const LEN: usize = 8  
        + 32  
        + 1  
        + (4 + 400 * 4)  
        + 8  
        + 8  
        + 32; 
}

#[error_code]
pub enum ErrorCode {
    #[msg("The text is too long")]
    TextTooLong,
    #[msg("You are not authorized for this action")]
    Unauthorized,
}
