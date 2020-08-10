use super::CLIENT_POOL;
use super::Pool;
use doggo_core::user::User;
use doggo_core::collection_abstractions::UserRepository;
use domain_patterns::models::Entity;

pub struct VitessUserRepository {
    pool: Pool,
}

impl VitessUserRepository {
    /// Associative function to create a new user repository from a connection.
    pub fn new() -> VitessUserRepository {
        VitessUserRepository {
            // "Clone" the pool (it's an Arc, so just increase count) and then get a connection for use
            // in this handler.
            pool: CLIENT_POOL.clone(),
        }
    }
}

impl UserRepository for VitessUserRepository {
    type Error = mysql::Error;

    fn get(&mut self, email: &String) -> Result<Option<User>, Self::Error> {
        let user: Option<User> =
            match self.pool.query_first(
                format!(r"SELECT u.id, u.email, u.password
                FROM users AS u
                WHERE u.email = '{}'", email)
            ) {
                Ok(Some(row)) => {
                    let (id, email, password) = row;
                    Some(User::new_raw(id, email, password))
                },
                Ok(None) => None,

                // Underlying MySQL error type unrelated to existence of user in db.
                Err(e) => {
                    return Err(e);
                }
            };

        Ok(user)
    }

    fn insert(&mut self, user: &User) -> Result<Option<String>, Self::Error> {
        match self.pool.query_drop(
            format!(r"INSERT INTO users (id, email, password)
            VALUES ('{}', '{}', '{}')", user.id(), &user.email(), &user.password())
        ) {
            Ok(_) => Ok(Some(user.id())),
            Err(e) => Err(e),
        }
    }
}
