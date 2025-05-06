use crate::elements::{ElementType};
use crate::elements::button::Button;
use crate::models::page::Page;
use egui::{Ui, Context, Color32};


// Состояние редактора
#[derive(Default)]
pub struct Editor {
    // Выбранный элемент для редактирования
    selected_element_id: Option<String>,
    // Выбранный тип элемента для добавления
    selected_element_type: Option<ElementType>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_element_id: None,
            selected_element_type: Some(ElementType::Button), // По умолчанию выбран тип "Кнопка"
        }
    }
    
    // Основной метод отображения редактора
    pub fn show(&mut self, ctx: &Context, ui: &mut Ui, page: &mut Page) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.show_toolbar(ui, page);
        });
        
        egui::SidePanel::right("properties").resizable(true).min_width(300.0).show(ctx, |ui| {
            self.show_properties(ui, page);
        });
        
        egui::SidePanel::left("elements").resizable(true).min_width(200.0).show(ctx, |ui| {
            self.show_elements_panel(ui, page);
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_editor_area(ui, page);
        });
    }
    
    // Показать панель инструментов
    fn show_toolbar(&mut self, ui: &mut Ui, page: &mut Page) {
        ui.horizontal(|ui| {
            ui.heading("Генератор сайтов");
            
            ui.separator();
            
            if ui.button("Сохранить").clicked() {
                // Логика сохранения
                println!("Сохранение проекта");
            }
            
            if ui.button("Экспорт").clicked() {
                // Логика экспорта
                println!("Экспорт проекта");
            }
            
            ui.separator();
            
            ui.label("Страница:");
            ui.text_edit_singleline(&mut page.title);
        });
    }
    
    // Показать панель элементов
    fn show_elements_panel(&mut self, ui: &mut Ui, page: &mut Page) {
        ui.heading("Элементы");
        
        ui.separator();
        
        ui.vertical(|ui| {
            if ui.selectable_label(
                self.selected_element_type == Some(ElementType::Button), 
                "Кнопка"
            ).clicked() {
                self.selected_element_type = Some(ElementType::Button);
            }
        });
        
        ui.separator();
        
        // Добавляем кнопку "Добавить элемент"
        if ui.button("Добавить элемент").clicked() {
            match self.selected_element_type {
                Some(ElementType::Button) => {
                    // Создаем новую кнопку
                    let button = Button::new();
                    // Добавляем кнопку на страницу
                    page.add_element(Box::new(button));
                },
                _ => {
                    // Здесь будет логика для других типов элементов
                }
            }
        }
    }
    
    // Показать редактор свойств
    fn show_properties(&mut self, ui: &mut Ui, page: &mut Page) {
        ui.heading("Свойства");
        
        if let Some(element_id) = &self.selected_element_id {
            if let Some(element) = page.find_element_mut(element_id) {
                ui.separator();
                
                match element.get_element_type() {
                    ElementType::Button => {
                        if let Some(button) = element.as_any().downcast_ref::<Button>() {
                            // Редактирование текста кнопки
                            ui.label("Текст кнопки:");
                            let mut content = button.content.clone();
                            if ui.text_edit_singleline(&mut content).changed() {
                                if let Some(button_mut) = element.as_any_mut().downcast_mut::<Button>() {
                                    button_mut.content = content;
                                }
                            }
                            
                            // Редактирование цветов и других свойств
                            // ...
                        }
                    },
                    _ => {
                        ui.label("Редактирование этого типа элемента пока не поддерживается");
                    }
                }
            } else {
                ui.label("Элемент не найден");
                self.selected_element_id = None;
            }
        } else {
            ui.label("Выберите элемент для редактирования");
        }
    }
    
    // Показать область редактирования
    fn show_editor_area(&mut self, ui: &mut Ui, page: &mut Page) {
        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::click_and_drag()
        );
        
        let rect = response.rect;
        
        // Отрисовываем фон рабочей области
        painter.rect_filled(rect, 0.0, Color32::WHITE);
        
        // Отрисовываем элементы страницы
        for element in &page.elements {
            let selected = Some(element.get_id().to_string()) == self.selected_element_id;
            element.render(&painter, selected);
        }
        
        // Обработка событий мыши
        if response.clicked() {
            // Клик мыши
            let pos = response.interact_pointer_pos.unwrap();
            let click_pos = (pos.x, pos.y);
            
            if let Some(element) = page.find_element_at_point(click_pos) {
                // Выбираем элемент
                self.selected_element_id = Some(element.get_id().to_string());
            } else {
                // Сбрасываем выбор
                self.selected_element_id = None;
            }
        } else if response.dragged() {
            // Перетаскивание
            if let Some(pos) = response.interact_pointer_pos {
                if let Some(element_id) = &self.selected_element_id {
                    if let Some(element) = page.find_element_mut(element_id) {
                        // Перемещаем выбранный элемент к текущей позиции мыши
                        let size = element.get_size();
                        element.set_position((pos.x - size.0 / 2.0, pos.y - size.1 / 2.0));
                    }
                }
            }
        }
    }
}