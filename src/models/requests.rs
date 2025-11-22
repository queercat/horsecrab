#[derive(FromForm)]
pub struct RegistrationRequest<'r>  {
    pub username: &'r str,
    pub password: &'r str 
}