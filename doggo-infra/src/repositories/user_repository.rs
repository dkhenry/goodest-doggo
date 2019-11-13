use super::CLIENT_POOL;
use super::Conn;
use doggo_core::user::User;
use doggo_core::collection_abstractions::UserRepository;
use domain_patterns::models::Entity;
use ulid::Ulid;

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
    type Error = mysql::Error;

    fn get(&mut self, email: &String) -> Result<Option<User>, Self::Error> {
        let user: Option<User> =
            match self.conn.query(
                format!(r"SELECT u.id, u.email, u.password
                FROM users AS u
                WHERE u.email = '{}'", email)
            ) {
                Ok(mut qr) => {
                    if let Some(row_result) = qr.next() {
                        let row = row_result?;
                        let (id_bytes, email, password) = mysql::from_row::<(Vec<u8>, String, String)>(row);
                        let mut arr = [0; 16];
                        arr.copy_from_slice(&id_bytes);
                        let id = u128::from_ne_bytes(arr);
                        Some(User::new_raw(id, email, password))
                    } else {
                        None
                    }
                },

                // Underlying MySQL error type unrelated to existence of user in db.
                Err(e) => {
                    return Err(e);
                }
            };

        Ok(user)
    }

    fn insert(&mut self, user: &User) -> Result<Option<Ulid>, Self::Error> {
        match self.conn.query(
            format!(r"INSERT INTO users (id, email, password)
            VALUES (unhex('{:032x}'), '{}', '{}')", user.bin_id(), &user.email(), &user.password())
        ) {
            Ok(_) => Ok(Some(user.raw_id())),
            Err(e) => Err(e),
        }
    }
}
