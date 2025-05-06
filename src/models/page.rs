use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use std::any::Any;

use crate::elements::{UIElement, UIElementExt};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub title: String,
    pub file_name: String,
    pub elements: Vec<Box<dyn UIElement>>,
    pub meta_tags: HashMap<String, String>,
    pub styles: HashMap<String, String>,
    pub scripts: Vec<String>,
}

impl Page {
    pub fn new(name: String, title: String, file_name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            title,
            file_name,
            elements: Vec::new(),
            meta_tags: HashMap::new(),
            styles: HashMap::new(),
            scripts: Vec::new(),
        }
    }
    
    // Добавление элемента на страницу
    pub fn add_element(&mut self, element: Box<dyn UIElement>) {
        self.elements.push(element);
    }
    
    // Удаление элемента со страницы
    pub fn remove_element(&mut self, element_id: &str) {
        self.elements.retain(|e| e.get_id() != element_id);
    }
    
    // Поиск элемента по ID
    pub fn find_element(&self, element_id: &str) -> Option<&Box<dyn UIElement>> {
        self.elements.iter().find(|e| e.get_id() == element_id)
    }
    
    // Поиск элемента для редактирования по ID
    pub fn find_element_mut(&mut self, element_id: &str) -> Option<&mut Box<dyn UIElement>> {
        self.elements.iter_mut().find(|e| e.get_id() == element_id)
    }
    
    // Поиск элемента в точке (x, y)
    pub fn find_element_at_point(&self, point: (f32, f32)) -> Option<&Box<dyn UIElement>> {
        self.elements.iter().rev().find(|e| e.contains_point(point))
    }
}

// Расширение трейта UIElement для приведения типов
pub trait UIElementExt: UIElement {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: UIElement + Any> UIElementExt for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}