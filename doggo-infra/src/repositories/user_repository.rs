use super::CLIENT_POOL;
use super::Conn;
use crate::errors::Error;
use doggo_core::user::User;
use doggo_core::collection_abstractions::UserRepository;
use domain_patterns::models::Entity;

pub struct VitessUserRepository {
    conn: Conn,
}

impl VitessUserRepository {
    /// Associative function to create a new user repository from a connection.
    pub fn new() -> VitessUserRepository {
        VitessUserRepository {
            // "Clone" the pool (it's an Arc, so just increase count) and then get a connection for use
            // in this handler.
            conn: CLIENT_POOL.clone().get_conn().unwrap(),
        }
    }
}

impl UserRepository for VitessUserRepository {
    type Error = Error;

    fn get(&mut self, id: String) -> Result<Option<User>, Self::Error> {
        let user: Option<User> =
            match self.conn.query(
                format!(r"SELECT u.email, u.password
                FROM users AS u
                WHERE u.id = '{}'", id)
            ) {
                Ok(mut qr) => {
                    if let Some(row_result) = qr.next() {
                        let row = row_result?;
                        let (email, password) = mysql::from_row(row);
                        Some(User::new_raw(id.clone(), email, password))
                    } else {
                        None
                    }
                },

                // Underlying MySQL error type unrelated to existence of user in db.
                Err(e) => {
                    return Err(e.into());
                }
            };

        Ok(user)
    }

    fn insert(&mut self, user: &User) -> Result<Option<String>, Self::Error> {
        match self.conn.query(
            format!(r"INSERT INTO users (id, email, password)
            VALUES ('{}','{}', '{}')", &user.id(), &user.email(), &user.password())
        ) {
            Ok(_) => Ok(Some(user.id())),
            Err(e) => Err(Error::from(e))
        }
    }
}
