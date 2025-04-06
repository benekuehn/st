//! `bottom` subcommand.

use crate::{
    ctx::StContext,
    errors::{StError, StResult},
    git::RepositoryExt,
};
use clap::Args;
use nu_ansi_term::Color;

/// CLI arguments for the `bottom` subcommand.
#[derive(Debug, Clone, Eq, PartialEq, Args)]
pub struct BottomCmd;

impl BottomCmd {
    /// Run the `bottom` subcommand.
    pub fn run(self, ctx: StContext<'_>) -> StResult<()> {
        // Discover the current stack
        let stack = ctx.discover_stack()?;

        // If the stack only has the trunk branch, there's no bottom branch to move to
        if stack.len() <= 1 {
            return Err(StError::NoStackFound);
        }

        // The bottom branch is the first branch after the trunk
        let bottom_branch = &stack[1];

        // Check out the bottom branch
        ctx.repository.checkout_branch(bottom_branch)?;

        println!(
            "Moved to bottom branch of stack: `{}`",
            Color::Green.paint(bottom_branch)
        );

        Ok(())
    }
}
