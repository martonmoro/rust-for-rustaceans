trait Database {
    fn get_user(&self, id: u32) -> Option<String>;
}

// Real implementation
struct RealDatabase;
impl Database for RealDatabase {
    fn get_user(&self, id: u32) -> Option<String> {
        Some(format!("User {}", id))
    }
}

struct MockDatabase {
    return_value: Option<String>,
}
impl Database for MockDatabase {
    fn get_user(&self, _id: u32) -> Option<String> {
        self.return_value.clone()
    }
}
// code to test
fn greet_user<D: Database>(db: &D, id: u32) -> String {
    match db.get_user(id) {
        Some(name) => format!("Hello, {}!", name),
        None => "User not found".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_found() {
        let mock = MockDatabase {
            return_value: Some("Alice".to_string()),
        };
        assert_eq!(greet_user(&mock, 1), "Hello, Alice!");
    }

    #[test]
    fn test_user_not_found() {
        let mock = MockDatabase { return_value: None };
        assert_eq!(greet_user(&mock, 1), "User not found");
    }
}

fn main() {
    let db = RealDatabase;
    println!("{}", greet_user(&db, 42));
}
