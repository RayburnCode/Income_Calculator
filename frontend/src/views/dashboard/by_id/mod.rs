pub mod income_worksheet;
pub use income_worksheet::Worksheet;

pub mod options_template;
pub use options_template::OptionsTemplate;

pub mod client;
pub use client::ClientDetails;

pub mod outreach;
pub use outreach::{Timeline, OutreachTemplates};

pub mod notes;
pub use notes::ClientNotes;

pub mod document;
pub use document::{ClientDocuments, UploadDocuments};

pub mod conditions;
pub use conditions::ClientConditions; 