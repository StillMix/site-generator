use crate::elements::ElementType;
use crate::elements::UIElement;
use crate::elements::buttons::Button;
use crate::models::page::Page;
use egui::{Ui, Context, Color32};

// Импортируем ButtonProperty
use crate::elements::buttons::buttonProperty::ButtonProperty;

// Структура редактора
pub struct Editor {
    // Выбранный элемент для редактирования
    selected_element_id: Option<String>,
    // Выбранный тип элемента для добавления
    selected_element_type: Option<ElementType>,
    // Перетаскивание элемента
    dragging_new_element: bool,
    // Позиция мыши
    mouse_pos: Option<(f32, f32)>,
    // Свойства кнопки
    button_property: ButtonProperty,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            selected_element_id: None,
            selected_element_type: None,
            dragging_new_element: false,
            mouse_pos: None,
            button_property: ButtonProperty::new(),
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_element_id: None,
            selected_element_type: Some(ElementType::Button), // По умолчанию выбран тип "Кнопка"
            dragging_new_element: false,
            mouse_pos: None,
            button_property: ButtonProperty::new(),
        }
    }
    
    // Основной метод отображения редактора
    pub fn show(&mut self, ctx: &Context, _ui: &mut Ui, page: &mut Page) {
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
    
    fn show_elements_panel(&mut self, ui: &mut Ui, _page: &mut Page) {
        ui.heading("Элементы");
        
        ui.separator();
        
        ui.vertical(|ui| {
            // Настраиваем сенсор для кнопки, чтобы явно разрешить определение перетаскивания
            let response = ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add(egui::Button::new("Кнопка").sense(egui::Sense::click_and_drag()))
            }).inner;
            
            // Отслеживаем наведение курсора на элемент (сделаем вывод только при изменении)
            static mut WAS_HOVERED: bool = false;
            if response.hovered() {
                if unsafe { !WAS_HOVERED } {
                    println!("Пользователь навелся на элемент в списке: Кнопка");
                    unsafe { WAS_HOVERED = true; }
                }
            } else {
                unsafe { WAS_HOVERED = false; }
            }
            
            // Обрабатываем клик на кнопке
            if response.clicked() {
                self.selected_element_type = Some(ElementType::Button);
                println!("Пользователь кликнул на элемент в списке: Кнопка");
            }
            
            // Проверяем состояние мыши напрямую
            let is_button_down = ui.input(|i| i.pointer.primary_down());
            let is_dragging = ui.input(|i| i.pointer.any_down() && i.pointer.is_moving());
            
            // Обнаружение начала перетаскивания
            if response.hovered() && is_button_down && !self.dragging_new_element {
                self.dragging_new_element = true;
                self.selected_element_type = Some(ElementType::Button);
                println!("Пользователь зажал ЛКМ на элементе в списке: Кнопка");
            }
            
            // Проверяем перетаскивание
            if self.dragging_new_element && is_dragging {
                if let Some(pos) = ui.ctx().pointer_interact_pos() {
                    self.mouse_pos = Some((pos.x, pos.y));
                    println!("Перетаскивание элемента Кнопка на позицию: ({:.1}, {:.1})", pos.x, pos.y);
                }
            }
        });
    }
    
    // Показать редактор свойств
    fn show_properties(&mut self, ui: &mut Ui, page: &mut Page) {
        ui.heading("Свойства");
        
        if let Some(element_id) = &self.selected_element_id {
            if let Some(element) = page.find_element_mut(element_id) {
                ui.separator();
                
                match element.get_element_type() {
                    ElementType::Button => {
                        // Используем ButtonProperty для отображения свойств кнопки
                        self.button_property.show(ui, element);
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
        // Отладочный вывод состояния перетаскивания
        if self.dragging_new_element {
            println!("Перетаскивание активно: {:?}", self.mouse_pos);
        }
        
        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::click_and_drag()
        );
        
        let rect = response.rect;
        
        // Отрисовываем фон рабочей области
        painter.rect_filled(rect, 0.0, Color32::WHITE);
        
        // Получаем текущую позицию мыши
        if let Some(pos) = ui.ctx().pointer_interact_pos() {
            self.mouse_pos = Some((pos.x, pos.y));
        }
        
        // Отрисовываем элементы страницы
        for element in &page.elements {
            let selected = Some(element.get_id().to_string()) == self.selected_element_id;
            element.render(&painter, selected);
        }
        
        // Если перетаскиваем новый элемент, отображаем его предпросмотр
        if self.dragging_new_element && !ui.input(|i| i.pointer.primary_down()) {
            println!("Пользователь завершил перетаскивание элемента");
            if let Some(pos) = self.mouse_pos {
                // Проверяем, что позиция находится внутри области редактирования
                if rect.contains(egui::Pos2::new(pos.0, pos.1)) {
                    match self.selected_element_type {
                        Some(ElementType::Button) => {
                            // Создаем новую кнопку
                            let mut button = crate::elements::button::Button::new();
                            // Устанавливаем позицию, учитывая центр кнопки
                            button.set_position((pos.0 - 50.0, pos.1 - 25.0));
                            // Добавляем кнопку на страницу
                            page.add_element(Box::new(button));
                            println!("Добавлена новая кнопка в позиции ({:.1}, {:.1})", pos.0, pos.1);
                        },
                        _ => {
                            // Здесь будет логика для других типов элементов
                        }
                    }
                } else {
                    println!("Перетаскивание завершено вне области редактирования");
                }
            }
            // Завершаем перетаскивание
            self.dragging_new_element = false;
        }
        
        // Обработка событий мыши
        if response.clicked() {
            // Клик мыши
            let pos = response.interact_pointer_pos.unwrap();
            let click_pos = (pos.x, pos.y);
            
            if let Some(element) = page.find_element_at_point(click_pos) {
                // Выбираем элемент
                self.selected_element_id = Some(element.get_id().to_string());
                self.dragging_new_element = false;
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
        } else if response.drag_released() {
            // Отпускание кнопки мыши после перетаскивания
            if self.dragging_new_element {
                println!("Пользователь завершил перетаскивание элемента");
                if let Some(pos) = ui.ctx().pointer_interact_pos() {
                    // Проверяем, что позиция находится внутри области редактирования
                    if rect.contains(pos) {
                        match self.selected_element_type {
                            Some(ElementType::Button) => {
                                // Создаем новую кнопку
                                let mut button = crate::elements::button::Button::new();
                                // Устанавливаем позицию, учитывая центр кнопки
                                button.set_position((pos.x - 50.0, pos.y - 25.0));
                                // Добавляем кнопку на страницу
                                page.add_element(Box::new(button));
                                println!("Добавлена новая кнопка в позиции ({}, {})", pos.x, pos.y);
                            },
                            _ => {
                                // Здесь будет логика для других типов элементов
                            }
                        }
                    } else {
                        println!("Перетаскивание завершено вне области редактирования");
                    }
                }
                // Завершаем перетаскивание
                self.dragging_new_element = false;
            }
        }
        
        // Отменяем перетаскивание, если кнопка мыши отпущена вне редактора
        if !ui.input(|i| i.pointer.any_down()) {
            if self.dragging_new_element {
                println!("Перетаскивание отменено");
                self.dragging_new_element = false;
            }
        }
    }
}