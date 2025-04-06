//! `down` subcommand.

use crate::{
    ctx::StContext,
    errors::{StError, StResult},
    git::RepositoryExt,
};
use clap::Args;
use nu_ansi_term::Color;

/// CLI arguments for the `down` subcommand.
#[derive(Debug, Clone, Eq, PartialEq, Args)]
pub struct DownCmd;

impl DownCmd {
    /// Run the `down` subcommand.
    pub fn run(self, ctx: StContext<'_>) -> StResult<()> {
        // Discover the current stack
        let stack = ctx.discover_stack()?;

        // If the stack only has the trunk branch, there's no place to move up to
        if stack.len() <= 1 {
            return Err(StError::NoStackFound);
        }

        // Get the current branch name
        let current_branch = ctx.repository.current_branch_name()?;

        // Find the current branch in the stack
        let current_index = stack
            .iter()
            .position(|branch| branch == &current_branch)
            .ok_or(StError::BranchNotTracked(current_branch.clone()))?;

        // If we're already at the trunk branch (index 0), can't go up any further
        if current_index == 0 {
            println!(
                "Already at trunk branch `{}`. Cannot move down further.",
                Color::Green.paint(&current_branch)
            );
            return Ok(());
        }

        let down_branch = &stack[current_index - 1];

        // Check out the branch below
        ctx.repository.checkout_branch(down_branch)?;

        println!("Moved down to: `{}`", Color::Green.paint(down_branch));

        Ok(())
    }
}
