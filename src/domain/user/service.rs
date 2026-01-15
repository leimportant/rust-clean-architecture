use super::entity::User;

pub struct UserDomainService;

impl UserDomainService {
    pub fn can_login(user: &User) -> bool {
        user.is_active.as_str() == "Y"
    }
}
