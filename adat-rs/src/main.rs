mod app;
mod commands;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("ADAT-RS — Active Directory Attack Tool"),
        ..Default::default()
    };
    eframe::run_native(
        "ADAT-RS",
        options,
        Box::new(|cc| Ok(Box::new(app::AdatApp::new(cc)))),
    )
}
