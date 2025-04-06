//! `top` subcommand.

use crate::{
    ctx::StContext,
    errors::{StError, StResult},
    git::RepositoryExt,
};
use clap::Args;
use nu_ansi_term::Color;

/// CLI arguments for the `top` subcommand.
#[derive(Debug, Clone, Eq, PartialEq, Args)]
pub struct TopCmd;

impl TopCmd {
    /// Run the `top` subcommand.
    pub fn run(self, ctx: StContext<'_>) -> StResult<()> {
        // Discover the current stack
        let stack = ctx.discover_stack()?;

        // If the stack only has the trunk branch, there's no top branch to move to
        if stack.len() <= 1 {
            return Err(StError::NoStackFound);
        }

        // The top branch is the last branch in the stack
        let top_branch = &stack[stack.len() - 1];

        // Check out the top branch
        ctx.repository.checkout_branch(top_branch)?;

        println!(
            "Moved to top branch of stack: `{}`",
            Color::Green.paint(top_branch)
        );

        Ok(())
    }
}
