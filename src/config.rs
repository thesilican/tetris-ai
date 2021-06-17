use common::api::ai::TetrisAI;
use rusty_ai::ai::RustyAI;

pub fn create_ai() -> impl TetrisAI + Unpin + 'static {
    // DummyAI::new()
    let weights = "PeUG/L2JDZ0+b33UPomlAz4u5ba+Sd2LvmZgJ75ueZS+H/K5vjFDQ74lc0m+bHEhvnh4ZL5NB7u+UN5bvfujZ75vVw6+K899vnGXUL5QWlK+O3Bivl8v/L5i4Xe+hoe2vYMBHzrsdGU9S92bPYdPQD0qjsI88ZJvPHhyu7zEjOa8miU2u5CSig==".parse().unwrap();
    RustyAI::new(&weights, 0)
}
