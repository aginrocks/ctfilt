use color_eyre::eyre::{ContextCompat, Result};
use names::Generator;

pub fn generate_hostname() -> Result<String> {
    let mut generator = Generator::default();
    generator.next().wrap_err("Failed to generate hostname")
}
