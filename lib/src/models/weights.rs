use errors::ValidationError;

/// An edge weight.
///
/// Edge weights must be between -1.0 and 1.0.
#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct Weight(pub f32);

impl Weight {
    /// Constructs a new edge weight.
    ///
    /// # Arguments
    ///
    /// * `weight` - The weight, between -1.0 and 1.0.
    ///
    /// # Errors
    /// Returns a `ValidationError` if the weight is below -1.0 or above 1.0.
    pub fn new(w: f32) -> Result<Self, ValidationError> {
        if w < -1.0 || w > 1.0 {
            Err(ValidationError::new("Weight out of range".to_string()))
        } else {
            Ok(Weight(w))
        }
    }
}
