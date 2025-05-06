use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ElementType {
    Text,
    Button,
    Image,
    Container,
    Form,
    Link,
    Custom(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Element {
    pub id: String,
    pub element_type: ElementType,
    pub content: String,
    pub styles: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub children: Vec<Element>,
    pub events: HashMap<String, String>,
}

impl Element {
    pub fn new(element_type: ElementType) -> Self {
        let id = Uuid::new_v4().to_string();
        
        let mut element = Self {
            id,
            element_type,
            content: String::new(),
            styles: HashMap::new(),
            attributes: HashMap::new(),
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            children: Vec::new(),
            events: HashMap::new(),
        };
        
        // Устанавливаем содержимое и стили по умолчанию в зависимости от типа
        match element.element_type {
            ElementType::Text => {
                element.content = "Текст".to_string();
                element.styles.insert("color".to_string(), "#000000".to_string());
                element.styles.insert("font-size".to_string(), "16px".to_string());
            },
            ElementType::Button => {
                element.content = "Кнопка".to_string();
                element.styles.insert("background-color".to_string(), "#4CAF50".to_string());
                element.styles.insert("color".to_string(), "#FFFFFF".to_string());
                element.styles.insert("border-radius".to_string(), "4px".to_string());
            },
            ElementType::Image => {
                element.attributes.insert("src".to_string(), "img/placeholder.png".to_string());
                element.attributes.insert("alt".to_string(), "Изображение".to_string());
            },
            ElementType::Link => {
                element.content = "Ссылка".to_string();
                element.attributes.insert("href".to_string(), "#".to_string());
                element.styles.insert("color".to_string(), "#0000EE".to_string());
                element.styles.insert("text-decoration".to_string(), "underline".to_string());
            },
            _ => {}
        }
        
        element
    }
    
    // Проверка, находится ли точка внутри элемента
    pub fn contains_point(&self, point: (f32, f32)) -> bool {
        point.0 >= self.position.0 
            && point.0 <= self.position.0 + self.size.0
            && point.1 >= self.position.1 
            && point.1 <= self.position.1 + self.size.1
    }
    
    // Перемещение элемента
    pub fn move_to(&mut self, position: (f32, f32)) {
        self.position = position;
    }
    
    // Изменение размера элемента
    pub fn resize(&mut self, size: (f32, f32)) {
        self.size = size;
    }
}