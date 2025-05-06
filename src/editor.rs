use crate::models::element::{Element, ElementType};
use crate::models::page::Page;
use egui::{Ui, Context, Rect, Pos2, Vec2, Color32, Stroke};
use std::collections::HashMap;

// Состояние редактора
#[derive(Default)]
pub struct Editor {
    // Выбранный элемент для редактирования
    selected_element_id: Option<String>,
    // Выбранный тип элемента для добавления
    selected_element_type: Option<ElementType>,
    // Режим перетаскивания
    dragging: bool,
    // Начальная позиция перетаскивания
    drag_start: Option<Pos2>,
    // Смещение перетаскивания
    drag_offset: Vec2,
    // Элемент, который перетаскивается
    dragged_element_id: Option<String>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_element_id: None,
            selected_element_type: None,
            dragging: false,
            drag_start: None,
            drag_offset: Vec2::ZERO,
            dragged_element_id: None,
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
            self.show_elements_panel(ui);
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
    fn show_elements_panel(&mut self, ui: &mut Ui) {
        ui.heading("Элементы");
        
        ui.separator();
        
        ui.vertical(|ui| {
            if ui.selectable_label(
                self.selected_element_type == Some(ElementType::Text), 
                "Текст"
            ).clicked() {
                self.selected_element_type = Some(ElementType::Text);
            }
            
            if ui.selectable_label(
                self.selected_element_type == Some(ElementType::Button), 
                "Кнопка"
            ).clicked() {
                self.selected_element_type = Some(ElementType::Button);
            }
            
            if ui.selectable_label(
                self.selected_element_type == Some(ElementType::Image), 
                "Изображение"
            ).clicked() {
                self.selected_element_type = Some(ElementType::Image);
            }
            
            if ui.selectable_label(
                self.selected_element_type == Some(ElementType::Container), 
                "Контейнер"
            ).clicked() {
                self.selected_element_type = Some(ElementType::Container);
            }
            
            if ui.selectable_label(
                self.selected_element_type == Some(ElementType::Form), 
                "Форма"
            ).clicked() {
                self.selected_element_type = Some(ElementType::Form);
            }
            
            if ui.selectable_label(
                self.selected_element_type == Some(ElementType::Link), 
                "Ссылка"
            ).clicked() {
                self.selected_element_type = Some(ElementType::Link);
            }
        });
        
        ui.separator();
        
        if let Some(element_type) = &self.selected_element_type {
            if ui.button("Добавить элемент").clicked() {
                let element = Element::new(element_type.clone());
                page.add_element(element);
            }
        }
    }
    
    // Показать редактор свойств
    fn show_properties(&mut self, ui: &mut Ui, page: &mut Page) {
        ui.heading("Свойства");
        
        if let Some(element_id) = &self.selected_element_id {
            if let Some(element) = page.find_element_mut(element_id) {
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("ID:");
                    ui.text_edit_singleline(&mut element.id.clone());
                });
                
                ui.horizontal(|ui| {
                    ui.label("Тип:");
                    ui.label(match element.element_type {
                        ElementType::Text => "Текст",
                        ElementType::Button => "Кнопка",
                        ElementType::Image => "Изображение",
                        ElementType::Container => "Контейнер",
                        ElementType::Form => "Форма",
                        ElementType::Link => "Ссылка",
                        ElementType::Custom(ref name) => name,
                    });
                });
                
                ui.separator();
                
                // Редактирование содержимого
                match element.element_type {
                    ElementType::Text | ElementType::Button | ElementType::Link => {
                        ui.label("Содержимое:");
                        ui.text_edit_singleline(&mut element.content);
                    },
                    ElementType::Image => {
                        ui.label("Источник изображения:");
                        let mut src = element.attributes.get("src")
                            .cloned().unwrap_or_default();
                        if ui.text_edit_singleline(&mut src).changed() {
                            element.attributes.insert("src".to_string(), src);
                        }
                        
                        ui.label("Альтернативный текст:");
                        let mut alt = element.attributes.get("alt")
                            .cloned().unwrap_or_default();
                        if ui.text_edit_singleline(&mut alt).changed() {
                            element.attributes.insert("alt".to_string(), alt);
                        }
                    },
                    _ => {}
                }
                
                ui.separator();
                
                // Позиция и размер
                ui.collapsing("Позиция и размер", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        let mut x = element.position.0;
                        if ui.add(egui::DragValue::new(&mut x).speed(1.0)).changed() {
                            element.position.0 = x;
                        }
                        
                        ui.label("Y:");
                        let mut y = element.position.1;
                        if ui.add(egui::DragValue::new(&mut y).speed(1.0)).changed() {
                            element.position.1 = y;
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Ширина:");
                        let mut width = element.size.0;
                        if ui.add(egui::DragValue::new(&mut width).speed(1.0)).changed() {
                            element.size.0 = width;
                        }
                        
                        ui.label("Высота:");
                        let mut height = element.size.1;
                        if ui.add(egui::DragValue::new(&mut height).speed(1.0)).changed() {
                            element.size.1 = height;
                        }
                    });
                });
                
                ui.separator();
                
                // Стили
                ui.collapsing("Стили", |ui| {
                    let mut styles_to_remove = Vec::new();
                    let mut styles_to_add = HashMap::new();
                    
                    for (key, value) in &mut element.styles.clone() {
                        ui.horizontal(|ui| {
                            let mut k = key.clone();
                            let mut v = value.clone();
                            
                            if ui.text_edit_singleline(&mut k).changed() {
                                styles_to_remove.push(key.clone());
                                styles_to_add.insert(k, v.clone());
                            }
                            
                            if ui.text_edit_singleline(&mut v).changed() {
                                element.styles.insert(key.clone(), v);
                            }
                            
                            if ui.button("❌").clicked() {
                                styles_to_remove.push(key.clone());
                            }
                        });
                    }
                    
                    // Удаляем стили, которые нужно удалить
                    for key in styles_to_remove {
                        element.styles.remove(&key);
                    }
                    
                    // Добавляем новые стили
                    for (key, value) in styles_to_add {
                        element.styles.insert(key, value);
                    }
                    
                    // Добавление нового стиля
                    ui.horizontal(|ui| {
                        static mut NEW_STYLE_KEY: String = String::new();
                        static mut NEW_STYLE_VALUE: String = String::new();
                        
                        unsafe {
                            ui.text_edit_singleline(&mut NEW_STYLE_KEY);
                            ui.text_edit_singleline(&mut NEW_STYLE_VALUE);
                            
                            if ui.button("Добавить стиль").clicked() && !NEW_STYLE_KEY.is_empty() {
                                element.styles.insert(NEW_STYLE_KEY.clone(), NEW_STYLE_VALUE.clone());
                                NEW_STYLE_KEY = String::new();
                                NEW_STYLE_VALUE = String::new();
                            }
                        }
                    });
                });
                
                ui.separator();
                
                // Удаление элемента
                if ui.button("Удалить элемент").clicked() {
                    page.remove_element(element_id);
                    self.selected_element_id = None;
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
        for element in page.elements.iter() {
            let element_rect = Rect::from_min_size(
                Pos2::new(element.position.0, element.position.1),
                Vec2::new(element.size.0, element.size.1)
            );
            
            // Заливка элемента
            let fill_color = if Some(element.id.clone()) == self.selected_element_id {
                Color32::from_rgba_premultiplied(100, 150, 255, 100)
            } else {
                Color32::from_rgba_premultiplied(200, 200, 200, 100)
            };
            
            painter.rect_filled(element_rect, 2.0, fill_color);
            
            // Рамка элемента
            let stroke = if Some(element.id.clone()) == self.selected_element_id {
                Stroke::new(2.0, Color32::BLUE)
            } else {
                Stroke::new(1.0, Color32::GRAY)
            };
            
            painter.rect_stroke(element_rect, 2.0, stroke);
            
            // Отрисовка текста элемента
            match element.element_type {
                ElementType::Text | ElementType::Button | ElementType::Link => {
                    painter.text(
                        element_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &element.content,
                        egui::FontId::default(),
                        Color32::BLACK
                    );
                },
                ElementType::Image => {
                    painter.text(
                        element_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "[Изображение]",
                        egui::FontId::default(),
                        Color32::BLACK
                    );
                },
                ElementType::Container => {
                    painter.text(
                        element_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "[Контейнер]",
                        egui::FontId::default(),
                        Color32::BLACK
                    );
                },
                ElementType::Form => {
                    painter.text(
                        element_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "[Форма]",
                        egui::FontId::default(),
                        Color32::BLACK
                    );
                },
                ElementType::Custom(ref name) => {
                    painter.text(
                        element_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &format!("[{}]", name),
                        egui::FontId::default(),
                        Color32::BLACK
                    );
                }
            }
        }
        
        // Обработка событий мыши
        if response.clicked() {
            // Клик мыши
            let pos = response.interact_pointer_pos.unwrap();
            let click_pos = (pos.x, pos.y);
            
            if let Some(element) = page.find_element_at_point(click_pos) {
                // Выбираем элемент
                self.selected_element_id = Some(element.id.clone());
            } else {
                // Сбрасываем выбор
                self.selected_element_id = None;
                
                // Если выбран тип элемента, создаем новый элемент
                if let Some(element_type) = &self.selected_element_type {
                    let mut element = Element::new(element_type.clone());
                    element.position = click_pos;
                    page.add_element(element.clone());
                    self.selected_element_id = Some(element.id);
                }
            }
        } else if response.dragged() {
            // Перетаскивание
            if let Some(pos) = response.interact_pointer_pos {
                if !self.dragging {
                    // Начинаем перетаскивание
                    self.dragging = true;
                    self.drag_start = Some(pos);
                    
                    // Проверяем, есть ли элемент в точке начала перетаскивания
                    if self.selected_element_id.is_none() {
                        let click_pos = (pos.x, pos.y);
                        if let Some(element) = page.find_element_at_point(click_pos) {
                            self.selected_element_id = Some(element.id.clone());
                        }
                    }
                    
                    // Запоминаем перетаскиваемый элемент
                    self.dragged_element_id = self.selected_element_id.clone();
                }
                
                // Если есть выбранный элемент, перемещаем его
                if let Some(element_id) = &self.dragged_element_id {
                    if let Some(element) = page.find_element_mut(element_id) {
                        let drag_delta = pos - self.drag_start.unwrap_or(pos);
                        
                        if drag_delta.length() > 0.0 {
                            let new_pos = (
                                element.position.0 + drag_delta.x - self.drag_offset.x,
                                element.position.1 + drag_delta.y - self.drag_offset.y
                            );
                            
                            element.position = new_pos;
                            self.drag_start = Some(pos);
                        }
                    }
                }
            }
        } else if response.drag_released() {
            // Конец перетаскивания
            self.dragging = false;
            self.drag_start = None;
            self.drag_offset = Vec2::ZERO;
            self.dragged_element_id = None;
        }
    }
}