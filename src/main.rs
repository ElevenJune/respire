use app::App;
use color_eyre::Result;

mod app;
mod ui;
mod breath_cycle;


fn main() -> Result<()> {
    //cli_log::init_cli_log!();
    let app = App::new();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}