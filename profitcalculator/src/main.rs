slint::include_modules!();

const TAXPERCENTAGE: f64 = 0.3;
const OWNERPERCENTAGE: f64 = 0.55;
const PROFITPERCENTAGE: f64 = 0.05;
const OPEXPERCENTAGE: f64 = 0.1;


fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_show_income(move |string|{
        let ui = ui_handle.unwrap();
        let num:f64 = string.trim().parse::<f64>().unwrap();
        let tax:f64 = num *TAXPERCENTAGE;
        let owner:f64 = num *OWNERPERCENTAGE;
        let profit:f64 = num *PROFITPERCENTAGE;
        let opex:f64 = num *OPEXPERCENTAGE;

        let result = format!("TAXES: {:.2}\nOWNER: {:.2}\nPROFIT: {:.2}\nOPEZ: {:.2}\n",tax,owner,profit,opex);
        ui.set_results(result.into());
        
    });
    ui.run()
}
