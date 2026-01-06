use serde::{Deserialize, Serialize};
use validator::Validate; 
use chrono::{DateTime, Utc}; 
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Term{
    Fifteen,
    Twenty,
    TwentyFive,
    Thirty,

}

// This struct represents a DSCRCalculator in the database and in the app
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DSCRCalculator {
    pub id: Uuid,
    pub purchase_price: f64,
    pub down_payment: f64,
    pub rate: f64,
    pub term: Term,
    pub annual_property_tax: f64,
    pub annual_insurance: f64,
    pub monthly_hoa: f64,
    pub e_x_rent: f64,
    pub created_at: DateTime<Utc>, 
    pub updated_at: DateTime<Utc>,
}
impl Default for DSCRCalculator { 
    fn default() -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            purchase_price: 0.0,
            down_payment: 0.0,
            rate: 0.0,
            term: Term::Thirty,
            annual_property_tax: 0.0,
            annual_insurance: 0.0,
            monthly_hoa: 0.0,
            e_x_rent: 0.0,
            created_at: now,
            updated_at: now,
        }
    }
}




#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDSCRCalculator {
    #[validate(range(min = 0.0, message = "Purchase price must be a positive number"))]
    pub purchase_price: f64,
    #[validate(range(min = 0.0, message = "Down payment must be a positive number"))]
    pub down_payment: f64,
    #[validate(range(min = 0.0, message = "Rate must be a positive number"))]
    pub rate: f64,
    pub term: Term,
    #[validate(range(min = 0.0, message = "Annual property tax must be a positive number"))]
    pub annual_property_tax: f64,
    #[validate(range(min = 0.0, message = "Annual insurance must be a positive number"))]
    pub annual_insurance: f64,
    #[validate(range(min = 0.0, message = "Monthly HOA must be a positive number"))]
    pub monthly_hoa: f64,
    #[validate(range(min = 0.0, message = "Expected rent must be a positive number"))]
    pub e_x_rent: f64,


}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDSCRCalculator {
    #[validate(range(min = 0.0, message = "Purchase price must be a positive number"))]
    pub purchase_price: Option<f64>,
    #[validate(range(min = 0.0, message = "Down payment must be a positive number"))]
    pub down_payment: Option<f64>,
    #[validate(range(min = 0.0, message = "Rate must be a positive number"))]
    pub rate: Option<f64>,
    pub term: Option<Term>,
    #[validate(range(min = 0.0, message = "Annual property tax must be a positive number"))]
    pub annual_property_tax: Option<f64>,
    #[validate(range(min = 0.0, message = "Annual insurance must be a positive number"))]
    pub annual_insurance: Option<f64>,
    #[validate(range(min = 0.0, message = "Monthly HOA must be a positive number"))]
    pub monthly_hoa: Option<f64>,
    #[validate(range(min = 0.0, message = "Expected rent must be a positive number"))]
    pub e_x_rent: Option<f64>,
}



// Query parameters for GET /dscr_calculators endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSCRCalculatorQueryParams {
    pub page: Option<u64>,       // For pagination
    pub per_page: Option<u64>,
    pub search: Option<String>,   // For search functionality
}