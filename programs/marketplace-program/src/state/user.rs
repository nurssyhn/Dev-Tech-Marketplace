use anchor_lang::prelude::*;

// Define a struct for user profile with additional fields
#[account]
pub struct UserProfile {
    pub name: String,                // 4 + 20
    pub email: String,               // 4 + 25
    pub profile_image: String,       // 4 + 50
    pub user_type: UserType,         // 1
    pub skills: Option<Vec<String>>, // 4 + 10*10            Only applicable for employees
}

impl UserProfile {
    pub const MAX_SIZE: usize = (4 + 20) + (4 + 25) + (4 + 50) + 1 + (4 + 10 * 10);

    pub fn init_employee_profile(
        &mut self,
        name: String,
        email: String,
        profile_image: String,
        skills: Option<Vec<String>>,
    ) -> Result<()> {
        self.name = name;
        self.email = email;
        self.profile_image = profile_image;
        self.user_type = UserType::Employee;
        self.skills = skills;
        Ok(())
    }

    pub fn init_employer_profile(
        &mut self,
        name: String,
        email: String,
        profile_image: String,
    ) -> Result<()> {
        self.name = name;
        self.email = email;
        self.profile_image = profile_image;
        self.user_type = UserType::Employer;
        self.skills = None;
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone)]
pub enum UserType {
    Employer,
    Employee,
}
