use anchor_lang::prelude::*;

// Declare the program ID
declare_id!("7AzUsuwMKP9XFpQaVt8Nt2XyAw8UHLWMYLnenxysV9Ce");

#[program]
pub mod todo_list_app {
    use super::*;

    // Function to add a new task
    pub fn adding_task(ctx: Context<AddingTask>, text: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        // Validate task length (Limit: 400 characters)
        if text.chars().count() > 400 {
            return Err(ErrorCode::TextTooLong.into());
        }

        // Assign task data
        task.id = task.key(); // The task ID is set to its own public key
        task.author = *author.key; // Assign the author (creator) of the task
        task.is_done = false; // New tasks are not completed by default
        task.created_at = clock.unix_timestamp; // Store task creation timestamp
        task.updated_at = clock.unix_timestamp; // Store task update timestamp
        task.text = text.clone(); // Store task text

        // Logging for debugging (Visible in Solana logs)
        msg!("Task successfully created");
        msg!("Task ID: {}", task.id);
        msg!("Created by: {}", author.key);
        msg!("Task Text: {}", text);
        msg!("Created At: {}", clock.unix_timestamp);

        Ok(())
    }

    // Function to update a task's completion status
    pub fn updating_task(ctx: Context<UpdatingTask>, is_done: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let clock = Clock::get().unwrap();

        // Update task completion status and timestamp
        task.is_done = is_done;
        task.updated_at = clock.unix_timestamp;

        Ok(())
    }

    // Function to mark a task as deleted
    pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let clock = Clock::get().unwrap();

        // Instead of deleting, mark it as "done" (soft delete)
        task.is_done = true;
        task.updated_at = clock.unix_timestamp;

        msg!("Task marked as deleted");
        msg!("Task ID: {}", task.id);
        msg!("Updated At: {}", clock.unix_timestamp);

        Ok(())
    }

    // Function to toggle completion status
    pub fn toggle_completion_status(ctx: Context<ToggleTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let clock = Clock::get().unwrap();

        // Ensure that only the task owner can update its status
        require!(
            task.author == *ctx.accounts.author.key,
            ErrorCode::Unauthorized
        );

        // Toggle task completion status
        task.is_done = !task.is_done;
        task.updated_at = clock.unix_timestamp;

        msg!("Task completion status toggled");
        msg!("Task ID: {}", task.id);
        msg!("New Completion Status: {}", task.is_done);
        msg!("Updated At: {}", clock.unix_timestamp);

        Ok(())
    }
}

// Accounts structures

// Account structure for adding a task
#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Account structure for updating a task
#[derive(Accounts)]
pub struct UpdatingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

// Account structure for deleting a task
#[derive(Accounts)]
pub struct DeletingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

// Account structure for toggling task completion
#[derive(Accounts)]
pub struct ToggleTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

// Task account structure stored on-chain
#[account]
pub struct Task {
    pub id: Pubkey,      // Unique task ID (public key)
    pub author: Pubkey,  // Task creator
    pub is_done: bool,   // Completion status
    pub text: String,    // Task description (limited to 400 chars)
    pub created_at: i64, // Creation timestamp
    pub updated_at: i64, // Last update timestamp
}

// Define storage size for task account
impl Task {
    pub const LEN: usize = 8  // Anchor's default discriminator
        + 32  // Task ID (public key)
        + 32  // Author's public key
        + 1   // Completion status (bool)
        + (4 + 400 * 4)  // String length (prefix 4 bytes + max 400 chars)
        + 8   // Created timestamp (i64)
        + 8;  // Updated timestamp (i64)
}

// Custom error definitions
#[error_code]
pub enum ErrorCode {
    #[msg("The text is too long")]
    TextTooLong,
    #[msg("You are not authorized for this action")]
    Unauthorized,
}
