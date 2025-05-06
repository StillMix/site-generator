use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use egui::{Pos2, Rect, Vec2, Color32, Stroke};
use crate::elements::{ElementBase, ElementType, UIElement};

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
    
    fn contains_point(&self, point: (f32, f32)) -> bool {
        self.base.contains_point(point)
    }
    
    fn render(&self, painter: &egui::Painter, selected: bool) {
        let element_rect = Rect::from_min_size(
            Pos2::new(self.base.position.0, self.base.position.1),
            Vec2::new(self.base.size.0, self.base.size.1)
        );
        
        // Определяем цвет фона
        let fill_color = if selected {
            Color32::from_rgba_premultiplied(100, 150, 255, 100)
        } else {
            // Пытаемся использовать цвет из стилей или стандартный цвет
            let bg_color = self.base.styles.get("background-color")
                .unwrap_or(&"#4CAF50".to_string());
            
            match bg_color.as_str() {
                "#4CAF50" => Color32::from_rgb(76, 175, 80),
                _ => Color32::LIGHT_GRAY // По умолчанию светло-серый
            }
        };
        
        // Рисуем фон кнопки
        painter.rect_filled(element_rect, 4.0, fill_color);
        
        // Рисуем рамку кнопки
        let stroke = if selected {
            Stroke::new(2.0, Color32::BLUE)
        } else {
            Stroke::new(1.0, Color32::DARK_GRAY)
        };
        
        painter.rect_stroke(element_rect, 4.0, stroke);
        
        // Рисуем текст кнопки
        let text_color = self.base.styles.get("color")
            .map(|c| match c.as_str() {
                "#FFFFFF" => Color32::WHITE,
                _ => Color32::BLACK
            })
            .unwrap_or(Color32::BLACK);
        
        painter.text(
            element_rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.content,
            egui::FontId::default(),
            text_color
        );
    }
}