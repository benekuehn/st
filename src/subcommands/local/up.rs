//! `up` subcommand.

use crate::{
    ctx::StContext,
    errors::{StError, StResult},
    git::RepositoryExt,
};
use clap::Args;
use nu_ansi_term::Color;

/// CLI arguments for the `up` subcommand.
#[derive(Debug, Clone, Eq, PartialEq, Args)]
pub struct UpCmd;

impl UpCmd {
    /// Run the `up` subcommand.
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
        if current_index == stack.len() - 1 {
            println!(
                "Already at topmost branch `{}`. Cannot move up further.",
                Color::Green.paint(&current_branch)
            );
            return Ok(());
        }

        let up_branch = &stack[current_index + 1];

        // Check out the branch above
        ctx.repository.checkout_branch(up_branch)?;

        println!("Moved up to: `{}`", Color::Green.paint(up_branch));

        Ok(())
    }
}
