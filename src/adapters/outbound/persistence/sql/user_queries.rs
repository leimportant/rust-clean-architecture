pub const CREATE_USER_SQL: &str = r#"
    INSERT INTO users (id, email, username, name, password, is_active)
    VALUES (?, ?, ?, ?, ?, ?)
"#;

pub const FIND_USER_BY_EMAIL_SQL: &str = r#"
    SELECT id, email, username, name, password
    FROM users
    WHERE email = ?
"#;

pub const FIND_USER_BY_ID_SQL: &str = r#"
    SELECT id, email, username, name, password, is_active
    FROM users
    WHERE id = ?
"#;
