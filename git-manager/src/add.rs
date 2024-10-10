use crate::user::{User, UserOperator};
use crate::util::paint_yellow;
use clap::Args;

#[derive(Args, Debug)]
pub struct AddArgs {
    pub alias: String,
    pub name: String,
    pub email: String,
}

pub async fn add (    
    args: &AddArgs,
    user_operator: &mut UserOperator,
) -> Result<(), Box<dyn std::error::Error>> {
    user_operator.set_user(User {
        alias: args.alias.clone(),
        name: args.name.clone(),
        email: args.email.clone(),
    })
    .await?;

    println!();
    println!("{} added successfully", paint_yellow(args.alias.as_str()),);
    Ok(())
}