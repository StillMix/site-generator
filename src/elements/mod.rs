pub mod button;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use std::any::Any;

// Базовый трейт для всех элементов
pub trait UIElement {
    fn get_id(&self) -> &str;
    fn get_element_type(&self) -> &ElementType;
    fn get_position(&self) -> (f32, f32);
    fn set_position(&mut self, position: (f32, f32));
    fn get_size(&self) -> (f32, f32);
    fn set_size(&mut self, size: (f32, f32));
    fn contains_point(&self, point: (f32, f32)) -> bool;
    fn render(&self, painter: &egui::Painter, selected: bool);
    
    // Методы для приведения типов
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Реализуем методы приведения типов для всех, кто реализует UIElement и Any
impl<T: UIElement + Any> UIElementExt for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Трейт для приведения типов в editor.rs
pub trait UIElementExt: UIElement {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

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

// Базовая структура для элемента
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementBase {
    pub id: String,
    pub element_type: ElementType,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub styles: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
}

impl ElementBase {
    pub fn new(element_type: ElementType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            element_type,
            position: (100.0, 100.0), // Изменено с (0.0, 0.0) для лучшей видимости
            size: (100.0, 50.0),
            styles: HashMap::new(),
            attributes: HashMap::new(),
        }
    }
    
    // Метод для проверки, находится ли точка внутри элемента
    pub fn contains_point(&self, point: (f32, f32)) -> bool {
        point.0 >= self.position.0 
            && point.0 <= self.position.0 + self.size.0
            && point.1 >= self.position.1 
            && point.1 <= self.position.1 + self.size.1
    }
}