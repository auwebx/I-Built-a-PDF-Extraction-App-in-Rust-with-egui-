use eframe::egui;
use egui::epaint::tessellator::path;
use std::fs;

fn main () -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Intelligent Document Processor"),
        ..Default::default()
    };

    eframe::run_native(
        "IDP Rust App",
        options,
        Box::new(|_cc| Box::new(IDPApp::default())),
    )
}

#[derive(Default)]
struct IDPApp {
    extracted_text: String,
    selected_file_path: Option<String>,
    status_message: String,
    is_processing: bool,
}

impl eframe::App for IDPApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Intelligent Document Processor");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("📂 Select PDF File").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PDF Documents", &["pdf"])
                        .pick_file()
                    {
                        self.selected_file_path = Some(path.display().to_string());
                        self.extracted_text.clear();
                        self.status_message = "File selected. Ready to extract.".to_string();
                    }
                }

                if let Some(ref path) = self.selected_file_path {
                    ui.label(format!("Selected: {}", path));
                } else {
                    ui.label("No file selected.");
                }
            });

            ui.add_space(10.0);

            if ui.button("⚙️ Extract Text").clicked() {
                if let Some(ref path) = self.selected_file_path {
                    self.is_processing = true;
                    self.status_message = "Extracting...".to_string();
                    ctx.request_repaint(); // Force UI update to show "Extracting..."

                    // Perform extraction
                    match extract_pdf_text(path) {
                        Ok(text) => {
                            self.extracted_text = text;
                            self.status_message = "✅ Extraction complete!".to_string();
                        }
                        Err(e) => {
                            self.extracted_text = format!("Error: {}", e);
                            self.status_message = format!("❌ Failed: {}", e);
                        }
                    }
                    self.is_processing = false;
                } else {
                    self.status_message = "⚠️ Please select a PDF file first.".to_string();
                }
            }

            ui.add_space(5.0);
            ui.label(&self.status_message);
            ui.separator();

             ui.heading("Extracted Content");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.extracted_text)
                        .desired_width(f32::INFINITY)
                        .desired_rows(20)
                        .interactive(true) // Allow user to copy text
                        .hint_text("Extracted text will appear here..."),
                );
            });


        });
    }
}

fn  extract_pdf_text(path: &str) -> Result<String, String>{
    let bytes = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
    pdf_extract::extract_text_from_mem(&bytes).map_err(|e| format!("PDF extraction error: {}", e))
}
