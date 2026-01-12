pub mod client;
pub mod client_info_card;
pub mod client_overview;

pub mod w2_jobs;
pub use w2_jobs::W2Jobs;

pub mod general_income;
pub use general_income::GeneralIncome;

pub use client::*;
pub use client_info_card::*;
pub use client_overview::*;