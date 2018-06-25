macro_rules! dbtry {
    ($expr:expr) => {
        match $expr {
            Ok(o) => o,
            Err(e) => bail!(DatabaseError(format!("{}", e))),
        }
    };
}

#[derive(Deserialize, Serialize)]
pub struct PackageParameter {
    name: String,
    value: String,
}

mod profiles;
mod project;
mod package;
mod delta;
pub mod template;

pub use self::profiles::{GenerationOptions, PublishProfile, Toggle};
pub use self::project::Project;
pub use self::package::{Node, Package, ValidationKind};
pub use self::delta::Delta;
