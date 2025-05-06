mod models;
pub mod elements;
mod editor;

use eframe::{App, CreationContext};
use models::page::Page;
use editor::Editor;

// Основной класс приложения
struct SiteGeneratorApp {
    editor: Editor,
    current_page: Page,
}

impl SiteGeneratorApp {
    fn new(_cc: &CreationContext) -> Self {
        // Создаем тестовую страницу
        let page = Page::new(
            "home".to_string(),
            "Главная страница".to_string(),
            "index.html".to_string()
        );
        
        Self {
            editor: Editor::new(),
            current_page: page,
        }
    }
}

impl App for SiteGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Показываем редактор
            self.editor.show(ctx, ui, &mut self.current_page);
        });
    }
}

fn main() {
    // Настройки окна приложения
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    
    // Запуск приложения
    eframe::run_native(
        "Генератор сайтов",
        options,
        Box::new(|cc| Box::new(SiteGeneratorApp::new(cc)))
    ).expect("Не удалось запустить приложение");
}