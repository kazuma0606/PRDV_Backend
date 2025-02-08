use validator::ValidationError;

pub fn validate_id(id: i32) -> Result<(), ValidationError> {
    if id < 1 {
        return Err(ValidationError::new("id must be greater than 0"));
    }
    Ok(())
}
