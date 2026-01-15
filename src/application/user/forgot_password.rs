use tracing::info;

pub struct ForgotPassword;

impl ForgotPassword {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, email: &str) -> Result<(), String> {
        info!("Password reset requested for email: {}", email);
        // In a real application, this would trigger an email to be sent
        // with a password reset link.
        Ok(())
    }
}
