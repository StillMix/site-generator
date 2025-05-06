use serde::{Serialize, Deserialize};
use egui::{Pos2, Rect, Vec2, Color32, Stroke};
use crate::elements::{ElementBase, ElementType, UIElement};
use std::any::Any;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Button {
    pub base: ElementBase,
    pub content: String,
    pub onclick: Option<String>,
}

impl Button {
    pub fn new() -> Self {
        let mut base = ElementBase::new(ElementType::Button);
        
        // Устанавливаем стили по умолчанию
        base.styles.insert("background-color".to_string(), "#4CAF50".to_string());
        base.styles.insert("color".to_string(), "#FFFFFF".to_string());
        base.styles.insert("border-radius".to_string(), "4px".to_string());
        
        Self {
            base,
            content: "Кнопка".to_string(),
            onclick: None,
        }
    }
    
    // Возвращает HTML-представление кнопки
    pub fn to_html(&self) -> String {
        let mut style = String::new();
        for (key, value) in &self.base.styles {
            style.push_str(&format!("{}:{};", key, value));
        }
        
        let mut attributes = String::new();
        for (key, value) in &self.base.attributes {
            if key != "style" {
                attributes.push_str(&format!(" {}=\"{}\"", key, value));
            }
        }
        
        // Добавляем обработчик события onclick
        let onclick_attr = if let Some(handler) = &self.onclick {
            format!(" onclick=\"{}\"", handler)
        } else {
            String::new()
        };
        
        // Формируем HTML-код кнопки
        format!(
            "<button id=\"{}\" style=\"{}\" {}{}>{}</button>",
            self.base.id, style, attributes, onclick_attr, self.content
        )
    }
}

impl UIElement for Button {
    fn get_id(&self) -> &str {
        &self.base.id
    }
    
    fn get_element_type(&self) -> &ElementType {
        &self.base.element_type
    }
    
    fn get_position(&self) -> (f32, f32) {
        self.base.position
    }
    
    fn set_position(&mut self, position: (f32, f32)) {
        self.base.position = position;
    }
    
    fn get_size(&self) -> (f32, f32) {
        self.base.size
    }
    
    fn set_size(&mut self, size: (f32, f32)) {
        self.base.size = size;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn contains_point(&self, point: (f32, f32)) -> bool {
        self.base.contains_point(point)
    }
    
    fn render(&self, painter: &egui::Painter, selected: bool) {
        let element_rect = Rect::from_min_size(
            Pos2::new(self.base.position.0, self.base.position.1),
            Vec2::new(self.base.size.0, self.base.size.1)
        );
        
        // Определяем цвет фона
        let bg_color_string = self.base.styles.get("background-color")
            .map(|s| s.clone())
            .unwrap_or_else(|| "#4CAF50".to_string());
            
        let base_color = match bg_color_string.as_str() {
            "#4CAF50" => Color32::from_rgb(76, 175, 80),
            _ => {
                // Простой парсер для цветов в формате HEX
                if bg_color_string.starts_with('#') && bg_color_string.len() >= 7 {
                    let r = u8::from_str_radix(&bg_color_string[1..3], 16).unwrap_or(76);
                    let g = u8::from_str_radix(&bg_color_string[3..5], 16).unwrap_or(175);
                    let b = u8::from_str_radix(&bg_color_string[5..7], 16).unwrap_or(80);
                    Color32::from_rgb(r, g, b)
                } else {
                    Color32::from_rgb(76, 175, 80) // По умолчанию зеленый
                }
            }
        };
        
        // Применяем выделение, сохраняя базовый цвет
        let fill_color = if selected {
            // При выделении делаем цвет немного ярче, сохраняя оттенок
            base_color.gamma_multiply(1.2)
        } else {
            base_color
        };
        
        // Проверяем наличие индивидуальных скруглений углов
        let has_custom_radius = self.base.styles.contains_key("border-top-left-radius") ||
            self.base.styles.contains_key("border-top-right-radius") ||
            self.base.styles.contains_key("border-bottom-left-radius") ||
            self.base.styles.contains_key("border-bottom-right-radius");
        
        if has_custom_radius {
            // Получаем значения для каждого угла
            let top_left = self.base.styles.get("border-top-left-radius")
                .map(|s| s.replace("px", "").parse::<f32>().unwrap_or(4.0))
                .unwrap_or(4.0);
            let top_right = self.base.styles.get("border-top-right-radius")
                .map(|s| s.replace("px", "").parse::<f32>().unwrap_or(4.0))
                .unwrap_or(4.0);
            let bottom_left = self.base.styles.get("border-bottom-left-radius")
                .map(|s| s.replace("px", "").parse::<f32>().unwrap_or(4.0))
                .unwrap_or(4.0);
            let bottom_right = self.base.styles.get("border-bottom-right-radius")
                .map(|s| s.replace("px", "").parse::<f32>().unwrap_or(4.0))
                .unwrap_or(4.0);
            
            // Для простоты используем среднее значение радиуса для основной фигуры
            let avg_radius = (top_left + top_right + bottom_left + bottom_right) / 4.0;
            
            // Рисуем основной прямоугольник
            painter.rect_filled(element_rect, avg_radius, fill_color);
            
            // Определяем параметры обводки
            let border_width = self.base.styles.get("border-width")
                .map(|s| s.replace("px", "").parse::<f32>().unwrap_or(1.0))
                .unwrap_or(1.0);
            
            let border_color_string = self.base.styles.get("border-color")
                .map(|s| s.clone())
                .unwrap_or_else(|| "#000000".to_string());
            
            let border_color = if border_color_string.starts_with('#') && border_color_string.len() >= 7 {
                let r = u8::from_str_radix(&border_color_string[1..3], 16).unwrap_or(0);
                let g = u8::from_str_radix(&border_color_string[3..5], 16).unwrap_or(0);
                let b = u8::from_str_radix(&border_color_string[5..7], 16).unwrap_or(0);
                Color32::from_rgb(r, g, b)
            } else {
                Color32::BLACK
            };
            
            // Рисуем рамку с усредненным радиусом
            if border_width > 0.0 {
                let stroke = if selected {
                    // При выделении добавляем дополнительную рамку
                    Stroke::new(border_width + 1.0, border_color) 
                } else {
                    Stroke::new(border_width, border_color)
                };
                
                painter.rect_stroke(element_rect, avg_radius, stroke);
            } else if selected {
                // Если обводка выключена, но кнопка выделена, рисуем рамку выделения
                painter.rect_stroke(element_rect, avg_radius, Stroke::new(1.0, Color32::BLUE));
            }
            
            // Дополнительно рисуем маленькие дуги в углах для наглядного отображения скруглений
            if selected {
                // Рисуем дуги в каждом углу для визуализации фактического скругления
                // Верхний левый угол
                if top_left > 0.0 {
                    painter.circle_stroke(
                        Pos2::new(element_rect.min.x + top_left, element_rect.min.y + top_left),
                        top_left,
                        Stroke::new(1.0, Color32::LIGHT_BLUE),
                    );
                }
                
                // Верхний правый угол
                if top_right > 0.0 {
                    painter.circle_stroke(
                        Pos2::new(element_rect.max.x - top_right, element_rect.min.y + top_right),
                        top_right,
                        Stroke::new(1.0, Color32::LIGHT_BLUE),
                    );
                }
                
                // Нижний левый угол
                if bottom_left > 0.0 {
                    painter.circle_stroke(
                        Pos2::new(element_rect.min.x + bottom_left, element_rect.max.y - bottom_left),
                        bottom_left,
                        Stroke::new(1.0, Color32::LIGHT_BLUE),
                    );
                }
                
                // Нижний правый угол
                if bottom_right > 0.0 {
                    painter.circle_stroke(
                        Pos2::new(element_rect.max.x - bottom_right, element_rect.max.y - bottom_right),
                        bottom_right,
                        Stroke::new(1.0, Color32::LIGHT_BLUE),
                    );
                }
            }
        } else {
            // Получаем значение скругления углов
            let border_radius = self.base.styles.get("border-radius")
                .map(|s| s.replace("px", "").parse::<f32>().unwrap_or(4.0))
                .unwrap_or(4.0);
            
            // Рисуем фон кнопки
            painter.rect_filled(element_rect, border_radius, fill_color);
            
            // Определяем параметры обводки
            let border_width = self.base.styles.get("border-width")
                .map(|s| s.replace("px", "").parse::<f32>().unwrap_or(1.0))
                .unwrap_or(1.0);
            
            let border_color_string = self.base.styles.get("border-color")
                .map(|s| s.clone())
                .unwrap_or_else(|| "#000000".to_string());
            
            let border_color = if border_color_string.starts_with('#') && border_color_string.len() >= 7 {
                let r = u8::from_str_radix(&border_color_string[1..3], 16).unwrap_or(0);
                let g = u8::from_str_radix(&border_color_string[3..5], 16).unwrap_or(0);
                let b = u8::from_str_radix(&border_color_string[5..7], 16).unwrap_or(0);
                Color32::from_rgb(r, g, b)
            } else {
                Color32::BLACK
            };
            
            // Рисуем рамку кнопки, если ширина обводки больше 0
            if border_width > 0.0 {
                let stroke = if selected {
                    // При выделении добавляем дополнительную рамку
                    Stroke::new(border_width + 1.0, border_color) 
                } else {
                    Stroke::new(border_width, border_color)
                };
                
                painter.rect_stroke(element_rect, border_radius, stroke);
            } else if selected {
                // Если обводка выключена, но кнопка выделена, рисуем рамку выделения
                painter.rect_stroke(element_rect, border_radius, Stroke::new(1.0, Color32::BLUE));
            }
        }
        
        // Определяем цвет текста
        let text_color_string = self.base.styles.get("color")
            .map(|s| s.clone())
            .unwrap_or_else(|| "#FFFFFF".to_string());
        
        let text_color = if text_color_string.starts_with('#') && text_color_string.len() >= 7 {
            let r = u8::from_str_radix(&text_color_string[1..3], 16).unwrap_or(255);
            let g = u8::from_str_radix(&text_color_string[3..5], 16).unwrap_or(255);
            let b = u8::from_str_radix(&text_color_string[5..7], 16).unwrap_or(255);
            Color32::from_rgb(r, g, b)
        } else if text_color_string == "#FFFFFF" {
            Color32::WHITE
        } else {
            Color32::BLACK
        };
        
        // Рисуем текст кнопки
        painter.text(
            element_rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.content,
            egui::FontId::default(),
            text_color
        );
    }
}