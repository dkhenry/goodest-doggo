use domain_patterns::command::Handles;
use crate::collection_abstractions::UserRepository;
use crate::commands::{CreateUserCommand, LoginCommand};
use crate::Error;
use crate::user::User;
use crate::Error::{DbFailure, NotAuthorized};

pub struct VitessUserCommandHandler<T>
    where T: UserRepository
{
    repo: T,
}

impl<T> VitessUserCommandHandler<T>
    where T: UserRepository
{
    pub fn new(repo: T) -> VitessUserCommandHandler<T> {
        VitessUserCommandHandler {
            repo,
        }
    }
}

impl<T> Handles<CreateUserCommand> for VitessUserCommandHandler<T>
    where T: UserRepository
{
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: CreateUserCommand) -> Self::Result {
        let new_user = User::new(msg.email, msg.password)?;

        let u_id = self.repo.insert(&new_user)
            .map_err(|e| {eprintln!("{}", e); DbFailure { source: Box::new(e) }})?;

        // Safe to unwrap.  If we had a duplicate key error, that's a database error and would
        // be returned above in the map err
        Ok(u_id.unwrap())
    }
}

impl<T> Handles<LoginCommand> for VitessUserCommandHandler<T>
    where T: UserRepository
{
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: LoginCommand) -> Self::Result {
        let user = self.repo.get(&msg.email)
            .map_err(|e| DbFailure { source: Box::new(e)})?
            .ok_or( NotAuthorized )?;


        if !user.valid_password(&msg.password) {
            return Err(NotAuthorized);
        }

        // We successfully logged in so return user id
        Ok(user.id().to_string())
    }
}
