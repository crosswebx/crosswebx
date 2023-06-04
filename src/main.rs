#![windows_subsystem = "windows"]
use crosswebx::run;

fn main() {
    pollster::block_on(run());
}
