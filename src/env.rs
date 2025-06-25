use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Display, Clone, Copy)]
pub enum Environment {
    #[strum(serialize = "production")]
    Prod,
    #[strum(serialize = "development")]
    Dev,
}
