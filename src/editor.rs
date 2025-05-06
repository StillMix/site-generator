use crate::elements::ElementType;
use crate::elements::UIElement; // Добавьте эту строку
use crate::elements::button::Button;
use crate::models::page::Page;
use egui::{Ui, Context, Color32};


// Состояние редактора
#[derive(Default)]
// Добавить в начало файла src/editor.rs (после существующих импортов)
struct ColorPicker {
    is_open: bool,
    current_hsv: (f32, f32, f32), // (hue, saturation, value)
    current_hex: String,
}

impl ColorPicker {
    fn new() -> Self {
        Self {
            is_open: false,
            current_hsv: (0.33, 0.57, 0.69), // Зеленый по умолчанию (соответствует #4CAF50)
            current_hex: "#4CAF50".to_string(),
        }
    }
    
    // Метод для преобразования HSV в RGB
    fn hsv_to_rgb(&self) -> (u8, u8, u8) {
        let (h, s, v) = self.current_hsv;
        let h = h % 1.0 * 6.0;
        let i = h.floor();
        let f = h - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        
        let (r, g, b) = match i as u8 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };
        
        ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
    
    // Метод для преобразования RGB в шестнадцатеричный формат
    fn rgb_to_hex(&self) -> String {
        let (r, g, b) = self.hsv_to_rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
    
    // Метод для преобразования шестнадцатеричного формата в RGB
    fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
        if !hex.starts_with('#') || hex.len() != 7 {
            return None;
        }
        
        match (
            u8::from_str_radix(&hex[1..3], 16),
            u8::from_str_radix(&hex[3..5], 16),
            u8::from_str_radix(&hex[5..7], 16),
        ) {
            (Ok(r), Ok(g), Ok(b)) => Some((r, g, b)),
            _ => None,
        }
    }
    
    // Метод для преобразования RGB в HSV
    fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;
        
        // Hue
        let h = if delta < 0.00001 {
            0.0
        } else if max == r {
            ((g - b) / delta).rem_euclid(6.0) / 6.0
        } else if max == g {
            ((b - r) / delta + 2.0) / 6.0
        } else {
            ((r - g) / delta + 4.0) / 6.0
        };
        
        // Saturation
        let s = if max < 0.00001 { 0.0 } else { delta / max };
        
        // Value
        let v = max;
        
        (h, s, v)
    }
    
    // Обновляет HSV на основе шестнадцатеричного значения
    fn update_from_hex(&mut self, hex: &str) {
        if let Some((r, g, b)) = Self::hex_to_rgb(hex) {
            self.current_hsv = Self::rgb_to_hsv(r, g, b);
            self.current_hex = hex.to_uppercase();
        }
    }
    
    // Обновляет шестнадцатеричное значение на основе HSV
    fn update_from_hsv(&mut self) {
        self.current_hex = self.rgb_to_hex();
    }
    
    // Показать в интерфейсе палитру выбора цвета
    fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut color_changed = false;
        
        // Показываем компактную кнопку с предпросмотром цвета
        ui.horizontal(|ui| {
            let (r, g, b) = self.hsv_to_rgb();
            let preview_color = egui::Color32::from_rgb(r, g, b);
            
            if ui.add(egui::Button::new("").fill(preview_color).min_size(egui::vec2(25.0, 25.0))).clicked() {
                self.is_open = !self.is_open;
            }
            
            ui.text_edit_singleline(&mut self.current_hex);
            
            if ui.button("OK").clicked() {
                let current_hex_copy = self.current_hex.clone();
self.update_from_hex(&current_hex_copy);
                color_changed = true;
            }
        });
        
        // Если палитра открыта, показываем ее
        if self.is_open {
            egui::Window::new("Выбор цвета")
                .fixed_size([280.0, 320.0])
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    // Основной прямоугольник для выбора S/V
                    let (mut h, mut s, mut v) = self.current_hsv;
                    let rect_response = ui.allocate_response(
                        egui::vec2(200.0, 200.0), 
                        egui::Sense::click_and_drag()
                    );
                    
                    if let Some(mouse_pos) = ui.ctx().pointer_interact_pos() {
                        if rect_response.dragged() || rect_response.clicked() {
                            let rect = rect_response.rect;
                            s = ((mouse_pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
                            v = 1.0 - ((mouse_pos.y - rect.min.y) / rect.height()).clamp(0.0, 1.0);
                            self.current_hsv = (h, s, v);
                            self.update_from_hsv();
                            color_changed = true;
                        }
                    }
                    
                    // Рисуем цветовой градиент S/V
let painter = ui.painter();
let rect = rect_response.rect;

// Создаем градиент яркости/насыщенности
for y in 0..200 {
    for x in 0..200 {
        let sat = x as f32 / 200.0;
        let val = 1.0 - (y as f32 / 200.0);
        
        // Вычисляем RGB для текущего пикселя
        let (hue_r, hue_g, hue_b) = {
            let h = h * 6.0;
            let i = h.floor();
            let f = h - i;
            let p = 0.0;
            let q = 1.0 - f;
            let t = f;
            
            match i as u8 {
                0 => (1.0, t, p),
                1 => (q, 1.0, p),
                2 => (p, 1.0, t),
                3 => (p, q, 1.0),
                4 => (t, p, 1.0),
                _ => (1.0, p, q),
            }
        };
        
        let r = (((1.0 - sat) + sat * hue_r) * val * 255.0) as u8;
        let g = (((1.0 - sat) + sat * hue_g) * val * 255.0) as u8;
        let b = (((1.0 - sat) + sat * hue_b) * val * 255.0) as u8;
        
        let pixel_color = egui::Color32::from_rgb(r, g, b);
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(rect.min.x + x as f32, rect.min.y + y as f32),
                egui::vec2(1.0, 1.0)
            ),
            0.0,
            pixel_color
        );
    }
}

// Указатель текущего выбора на градиенте S/V
let pos_x = rect.min.x + s * rect.width();
let pos_y = rect.min.y + (1.0 - v) * rect.height();
painter.circle_stroke(
    egui::pos2(pos_x, pos_y),
    5.0,
    egui::Stroke::new(1.0, egui::Color32::WHITE)
);
                    
                    ui.spacing_mut().item_spacing = egui::vec2(10.0, 0.0);
                    
                    // Боковая полоса выбора оттенка
                    ui.horizontal(|ui| {
                        // Цветовая полоса для выбора оттенка
                        let hue_bar_response = ui.allocate_response(
                            egui::vec2(20.0, 200.0), 
                            egui::Sense::click_and_drag()
                        );
                        
                        if let Some(mouse_pos) = ui.ctx().pointer_interact_pos() {
                            if hue_bar_response.dragged() || hue_bar_response.clicked() {
                                let bar_rect = hue_bar_response.rect;
                                h = ((mouse_pos.y - bar_rect.min.y) / bar_rect.height()).clamp(0.0, 1.0);
                                self.current_hsv = (h, s, v);
                                self.update_from_hsv();
                                color_changed = true;
                            }
                        }
                        
                        // Рисуем градиент оттенков
let painter = ui.painter();
let bar_rect = hue_bar_response.rect;

// Градиент оттенков
for y in 0..200 {
    let hue = y as f32 / 200.0;
    let (r, g, b) = {
        let h = hue * 6.0;
        let i = h.floor();
        let f = h - i;
        let p = 0.0;
        let q = 1.0 - f;
        let t = f;
        
        match i as u8 {
            0 => (1.0, t, p),
            1 => (q, 1.0, p),
            2 => (p, 1.0, t),
            3 => (p, q, 1.0),
            4 => (t, p, 1.0),
            _ => (1.0, p, q),
        }
    };
    
    let hue_color = egui::Color32::from_rgb(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    );
    
    painter.rect_filled(
        egui::Rect::from_min_size(
            egui::pos2(bar_rect.min.x, bar_rect.min.y + y as f32),
            egui::vec2(20.0, 1.0)
        ),
        0.0,
        hue_color
    );
}

// Маркер выбранного оттенка
let marker_y = bar_rect.min.y + h * bar_rect.height();
painter.rect_stroke(
    egui::Rect::from_min_max(
        egui::pos2(bar_rect.min.x, marker_y - 2.0),
        egui::pos2(bar_rect.max.x, marker_y + 2.0)
    ),
    0.0,
    egui::Stroke::new(1.0, egui::Color32::WHITE)
);
                        
                        // Предпросмотр выбранного цвета
                        ui.vertical(|ui| {
                            let preview_size = egui::vec2(30.0, 30.0);
                            let (r, g, b) = self.hsv_to_rgb();
                            let preview_color = egui::Color32::from_rgb(r, g, b);
                            
                            ui.add(
                                egui::widgets::Button::new("")
                                    .fill(preview_color)
                                    .min_size(preview_size)
                            );
                            
                            ui.spacing();
                            
                            // Поле ввода HEX-кода
                            ui.label("HEX:");
                            let mut hex_value = self.current_hex.clone();
                            if ui.text_edit_singleline(&mut hex_value).changed() {
                                if hex_value.starts_with('#') && hex_value.len() <= 7 {
                                    self.current_hex = hex_value;
                                    if self.current_hex.len() == 7 {
                                        let current_hex_copy = self.current_hex.clone();
self.update_from_hex(&current_hex_copy);
                                    }
                                } else if !hex_value.starts_with('#') {
                                    self.current_hex = format!("#{}", hex_value);
                                }
                                color_changed = true;
                            }
                            
                            // Кнопки для подтверждения и отмены
                            ui.horizontal(|ui| {
                                if ui.button("OK").clicked() {
                                    self.is_open = false;
                                    color_changed = true;
                                }
                                
                                if ui.button("Отмена").clicked() {
                                    self.is_open = false;
                                }
                            });
                        });
                    });
                });
        }
        
        if color_changed {
            Some(self.current_hex.clone())
        } else {
            None
        }
    }
}
#[derive(Default)]
pub struct Editor {
    // Выбранный элемент для редактирования
    selected_element_id: Option<String>,
    // Выбранный тип элемента для добавления
    selected_element_type: Option<ElementType>,
    // Перетаскивание элемента
    dragging_new_element: bool,
    // Позиция мыши
    mouse_pos: Option<(f32, f32)>,
    // Добавляем палитры цветов
    bg_color_picker: ColorPicker,
    text_color_picker: ColorPicker,
    border_color_picker: ColorPicker,
}


impl Editor {
    pub fn new() -> Self {
        Self {
            selected_element_id: None,
            selected_element_type: Some(ElementType::Button), // По умолчанию выбран тип "Кнопка"
            dragging_new_element: false,
            mouse_pos: None,
            bg_color_picker: ColorPicker::new(),
            text_color_picker: ColorPicker::new(),
            border_color_picker: ColorPicker::new(),
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
    // Показать редактор свойств
    fn show_properties(&mut self, ui: &mut Ui, page: &mut Page) {
        ui.heading("Свойства");
        
        if let Some(element_id) = &self.selected_element_id {
            if let Some(element) = page.find_element_mut(element_id) {
                ui.separator();
                
                match element.get_element_type() {
                    ElementType::Button => {
                        // Сначала проверяем, что это кнопка
                        if element.as_any().downcast_ref::<Button>().is_some() {
                            // Редактирование текста кнопки
                            ui.label("Текст кнопки:");
                            
                            // Получаем копию текста для редактирования
                            let mut content = {
                                let button = element.as_any().downcast_ref::<Button>().unwrap();
                                button.content.clone()
                            };
                            
                            if ui.text_edit_singleline(&mut content).changed() {
                                // Применяем изменения только если был изменен текст
                                if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
                                    button.content = content;
                                }
                            }
                            
                            ui.separator();
                            ui.heading("Стили");
                            
                            // Цвет фона - работаем с копией для избежания конфликтов заимствования
                            {
                                ui.label("Цвет фона:");
                                // Получаем текущий цвет фона
                                {
                                    let button = element.as_any().downcast_ref::<Button>().unwrap();
                                    let current_bg_color = button.base.styles.get("background-color")
                                        .cloned().unwrap_or_else(|| "#4CAF50".to_string());
                                    self.bg_color_picker.update_from_hex(&current_bg_color);
                                }
                                
                                // Показываем палитру и обрабатываем изменения
                                if let Some(new_color) = self.bg_color_picker.show(ui) {
                                    if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
                                        button.base.styles.insert("background-color".to_string(), new_color);
                                    }
                                }
                            }
                            
                            // Цвет текста
                            {
                                ui.label("Цвет текста:");
                                // Получаем текущий цвет текста
                                {
                                    let button = element.as_any().downcast_ref::<Button>().unwrap();
                                    let current_text_color = button.base.styles.get("color")
                                        .cloned().unwrap_or_else(|| "#FFFFFF".to_string());
                                    self.text_color_picker.update_from_hex(&current_text_color);
                                }
                                
                                // Показываем палитру и обрабатываем изменения
                                if let Some(new_color) = self.text_color_picker.show(ui) {
                                    if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
                                        button.base.styles.insert("color".to_string(), new_color);
                                    }
                                }
                            }
                            
                            // Обводка
                            {
                                ui.label("Обводка:");
                                let border_width = {
                                    let button = element.as_any().downcast_ref::<Button>().unwrap();
                                    button.base.styles.get("border-width")
                                        .cloned().unwrap_or_else(|| "1px".to_string())
                                };
                                
                                let mut border_enabled = !border_width.starts_with("0");
                                
                                if ui.checkbox(&mut border_enabled, "Включить обводку").changed() {
                                    if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
                                        if border_enabled {
                                            button.base.styles.insert("border-width".to_string(), "1px".to_string());
                                            button.base.styles.insert("border-style".to_string(), "solid".to_string());
                                            button.base.styles.insert("border-color".to_string(), "#000000".to_string());
                                        } else {
                                            button.base.styles.insert("border-width".to_string(), "0px".to_string());
                                        }
                                    }
                                }
                                
                                if border_enabled {
                                    // Толщина обводки
                                    ui.label("Толщина обводки (px):");
                                    let mut border_width_val = border_width.replace("px", "")
                                        .parse::<f32>().unwrap_or(1.0);
                                    
                                    if ui.add(egui::Slider::new(&mut border_width_val, 1.0..=10.0).step_by(1.0)).changed() {
                                        if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
                                            button.base.styles.insert("border-width".to_string(), format!("{}px", border_width_val));
                                        }
                                    }
                                    
                                    ui.label("Цвет обводки:");
                                    // Получаем текущий цвет обводки
                                    {
                                        let button = element.as_any().downcast_ref::<Button>().unwrap();
                                        let current_border_color = button.base.styles.get("border-color")
                                            .cloned().unwrap_or_else(|| "#000000".to_string());
                                        self.border_color_picker.update_from_hex(&current_border_color);
                                    }
                                    
                                    // Показываем палитру и обрабатываем изменения
                                    if let Some(new_color) = self.border_color_picker.show(ui) {
                                        if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
                                            button.base.styles.insert("border-color".to_string(), new_color);
                                        }
                                    }
                                }
                            }
                            
                            // Скругление углов
                            {
                                ui.label("Скругление углов:");
                                let border_radius = {
                                    let button = element.as_any().downcast_ref::<Button>().unwrap();
                                    button.base.styles.get("border-radius")
                                        .cloned().unwrap_or_else(|| "4px".to_string())
                                };
                                
                                let border_radius_val = border_radius.replace("px", "")
                                    .parse::<f32>().unwrap_or(4.0);
                                let mut border_radius_copy = border_radius_val;
                                
                                if ui.add(egui::Slider::new(&mut border_radius_copy, 0.0..=50.0).step_by(1.0)).changed() {
                                    if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
                                        button.base.styles.insert("border-radius".to_string(), format!("{}px", border_radius_copy));
                                    }
                                }
                                
                                // Индивидуальное скругление углов
                                let has_custom_radius = {
                                    let button = element.as_any().downcast_ref::<Button>().unwrap();
                                    button.base.styles.contains_key("border-top-left-radius")
                                };
                                let mut custom_radius = has_custom_radius;
                                
                                if ui.checkbox(&mut custom_radius, "Настроить углы по отдельности").changed() {
                                    if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
                                        if custom_radius {
                                            button.base.styles.insert("border-top-left-radius".to_string(), format!("{}px", border_radius_copy));
                                            button.base.styles.insert("border-top-right-radius".to_string(), format!("{}px", border_radius_copy));
                                            button.base.styles.insert("border-bottom-left-radius".to_string(), format!("{}px", border_radius_copy));
                                            button.base.styles.insert("border-bottom-right-radius".to_string(), format!("{}px", border_radius_copy));
                                        } else {
                                            button.base.styles.remove("border-top-left-radius");
                                            button.base.styles.remove("border-top-right-radius");
                                            button.base.styles.remove("border-bottom-left-radius");
                                            button.base.styles.remove("border-bottom-right-radius");
                                        }
                                    }
                                }
                                
                                if custom_radius {
                                    for (name, label) in [
                                        ("border-top-left-radius", "Левый верхний:"),
                                        ("border-top-right-radius", "Правый верхний:"),
                                        ("border-bottom-left-radius", "Левый нижний:"),
                                        ("border-bottom-right-radius", "Правый нижний:")
                                    ] {
                                        ui.label(label);
                                        let radius = {
                                            let button = element.as_any().downcast_ref::<Button>().unwrap();
                                            button.base.styles.get(name)
                                                .cloned().unwrap_or_else(|| format!("{}px", border_radius_copy))
                                        };
                                        
                                        let radius_val = radius.replace("px", "")
                                            .parse::<f32>().unwrap_or(border_radius_copy);
                                        let mut radius_copy = radius_val;
                                        
                                        if ui.add(egui::Slider::new(&mut radius_copy, 0.0..=50.0).step_by(1.0)).changed() {
                                            if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
                                                button.base.styles.insert(name.to_string(), format!("{}px", radius_copy));
                                            }
                                        }
                                    }
                                }
                            }
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
    }else if response.drag_released() {
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