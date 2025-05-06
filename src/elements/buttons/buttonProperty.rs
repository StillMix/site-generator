use egui::{Ui, Color32};
use crate::elements::buttons::Button;
use std::any::Any;
use crate::elements::UIElement;

// Структура ColorPicker из editor.rs (перенесем ее сюда)
pub struct ColorPicker {
    pub is_open: bool,
    pub current_hsv: (f32, f32, f32), // (hue, saturation, value)
    pub current_hex: String,
}

impl ColorPicker {
    pub fn new() -> Self {
        Self {
            is_open: false,
            current_hsv: (0.33, 0.57, 0.69), // Зеленый по умолчанию (соответствует #4CAF50)
            current_hex: "#4CAF50".to_string(),
        }
    }
    
    // Метод для преобразования HSV в RGB
    pub fn hsv_to_rgb(&self) -> (u8, u8, u8) {
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
    pub fn rgb_to_hex(&self) -> String {
        let (r, g, b) = self.hsv_to_rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
    
    // Метод для преобразования шестнадцатеричного формата в RGB
    pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
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
    pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
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
    pub fn update_from_hex(&mut self, hex: &str) {
        if let Some((r, g, b)) = Self::hex_to_rgb(hex) {
            self.current_hsv = Self::rgb_to_hsv(r, g, b);
            self.current_hex = hex.to_uppercase();
        }
    }
    
    // Обновляет шестнадцатеричное значение на основе HSV
    pub fn update_from_hsv(&mut self) {
        self.current_hex = self.rgb_to_hex();
    }
    
    // Показать в интерфейсе палитру выбора цвета
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
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

// Структура для работы с свойствами кнопки
pub struct ButtonProperty {
    pub bg_color_picker: ColorPicker,
    pub text_color_picker: ColorPicker,
    pub border_color_picker: ColorPicker,
}

impl ButtonProperty {
    pub fn new() -> Self {
        Self {
            bg_color_picker: ColorPicker::new(),
            text_color_picker: ColorPicker::new(),
            border_color_picker: ColorPicker::new(),
        }
    }
    
    // Метод для отображения и редактирования свойств кнопки
    pub fn show(&mut self, ui: &mut Ui, element: &mut Box<dyn UIElement>) -> bool {
        // Проверяем, что это кнопка
        if let Some(button) = element.as_any_mut().downcast_mut::<Button>() {
            // Редактирование текста кнопки
            ui.label("Текст кнопки:");
            let mut content = button.content.clone();
            let content_changed = ui.text_edit_singleline(&mut content).changed();
            if content_changed {
                button.content = content;
            }
            
            ui.separator();
            ui.heading("Стили");
            
            // Цвет фона
            ui.label("Цвет фона:");
            let current_bg_color = button.base.styles.get("background-color")
                .cloned().unwrap_or_else(|| "#4CAF50".to_string());
            self.bg_color_picker.update_from_hex(&current_bg_color);
            
            let bg_color_changed = if let Some(new_color) = self.bg_color_picker.show(ui) {
                button.base.styles.insert("background-color".to_string(), new_color);
                true
            } else {
                false
            };
            
            // Цвет текста
            ui.label("Цвет текста:");
            let current_text_color = button.base.styles.get("color")
                .cloned().unwrap_or_else(|| "#FFFFFF".to_string());
            self.text_color_picker.update_from_hex(&current_text_color);
            
            let text_color_changed = if let Some(new_color) = self.text_color_picker.show(ui) {
                button.base.styles.insert("color".to_string(), new_color);
                true
            } else {
                false
            };
            
            // Обводка
            ui.label("Обводка:");
            let border_width = button.base.styles.get("border-width")
                .cloned().unwrap_or_else(|| "1px".to_string());
            
            let mut border_enabled = !border_width.starts_with("0");
            let border_enabled_changed = ui.checkbox(&mut border_enabled, "Включить обводку").changed();
            
            let mut border_changed = false;
            if border_enabled_changed {
                if border_enabled {
                    button.base.styles.insert("border-width".to_string(), "1px".to_string());
                    button.base.styles.insert("border-style".to_string(), "solid".to_string());
                    button.base.styles.insert("border-color".to_string(), "#000000".to_string());
                } else {
                    button.base.styles.insert("border-width".to_string(), "0px".to_string());
                }
                border_changed = true;
            }
            
            let mut border_width_changed = false;
            if border_enabled {
                // Толщина обводки
                ui.label("Толщина обводки (px):");
                let mut border_width_val = border_width.replace("px", "")
                    .parse::<f32>().unwrap_or(1.0);
                
                border_width_changed = ui.add(egui::Slider::new(&mut border_width_val, 1.0..=10.0).step_by(1.0)).changed();
                
                if border_width_changed {
                    button.base.styles.insert("border-width".to_string(), format!("{}px", border_width_val));
                }
                
                ui.label("Цвет обводки:");
                let current_border_color = button.base.styles.get("border-color")
                    .cloned().unwrap_or_else(|| "#000000".to_string());
                self.border_color_picker.update_from_hex(&current_border_color);
                
                if let Some(new_color) = self.border_color_picker.show(ui) {
                    button.base.styles.insert("border-color".to_string(), new_color);
                    border_changed = true;
                }
            }
            
            // Скругление углов
            ui.label("Скругление углов:");
            let border_radius = button.base.styles.get("border-radius")
                .cloned().unwrap_or_else(|| "4px".to_string());
            
            let border_radius_val = border_radius.replace("px", "")
                .parse::<f32>().unwrap_or(4.0);
            let mut border_radius_copy = border_radius_val;
            
            let border_radius_changed = ui.add(egui::Slider::new(&mut border_radius_copy, 0.0..=50.0).step_by(1.0)).changed();
            
            if border_radius_changed {
                button.base.styles.insert("border-radius".to_string(), format!("{}px", border_radius_copy));
            }
            
            // Индивидуальное скругление углов
            let has_custom_radius = button.base.styles.contains_key("border-top-left-radius");
            let mut custom_radius = has_custom_radius;
            
            let custom_radius_changed = ui.checkbox(&mut custom_radius, "Настроить углы по отдельности").changed();
            
            let mut custom_corners_changed = false;
            if custom_radius_changed {
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
                custom_corners_changed = true;
            }
            
            if custom_radius {
                for (name, label) in [
                    ("border-top-left-radius", "Левый верхний:"),
                    ("border-top-right-radius", "Правый верхний:"),
                    ("border-bottom-left-radius", "Левый нижний:"),
                    ("border-bottom-right-radius", "Правый нижний:")
                ] {
                    ui.label(label);
                    let radius = button.base.styles.get(name)
                        .cloned().unwrap_or_else(|| format!("{}px", border_radius_copy));
                    
                    let radius_val = radius.replace("px", "")
                        .parse::<f32>().unwrap_or(border_radius_copy);
                    let mut radius_copy = radius_val;
                    
                    if ui.add(egui::Slider::new(&mut radius_copy, 0.0..=50.0).step_by(1.0)).changed() {
                        button.base.styles.insert(name.to_string(), format!("{}px", radius_copy));
                        custom_corners_changed = true;
                    }
                }
            }
            
            // Возвращаем true, если было изменено хотя бы одно свойство
            return content_changed || 
                   bg_color_changed || 
                   text_color_changed || 
                   border_enabled_changed || 
                   border_width_changed || 
                   border_changed ||
                   border_radius_changed || 
                   custom_radius_changed || 
                   custom_corners_changed;
        }
        
        false
    }
}