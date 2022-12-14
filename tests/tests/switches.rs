use typescript_jit::Runtime;
use typescript_tests::{TestLogger, check};

#[test]
fn run_switches() -> Result<(), String> {
    log::set_boxed_logger(TestLogger::new("results/switches.log")).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let rt = Runtime::new();

    let module = rt.load_file(
        "tests/switches.ts",
        Some("results/switches.ir".into())
    ).map_err(|e| e.to_string())?;

    check(module, "choice", 2.0)?;

    Ok(())
}