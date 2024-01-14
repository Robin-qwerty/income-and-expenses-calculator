#![windows_subsystem = "windows"]

slint::include_modules!();

// const HEALTHCARE: f64 = 107;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.45;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let owmer: f64 = num * OWNERPER;
        let profit: f64 = num * PROFITPER;

        let result: String = format!("Owner: {:.2}\n Profit: {:.2}", owmer, profit);

        ui.set_results(result.into());
    });

    ui.run()
}
