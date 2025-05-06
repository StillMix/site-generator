use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::element::Element;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub title: String,
    pub file_name: String,
    pub elements: Vec<Element>,
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
    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }
    
    // Удаление элемента со страницы
    pub fn remove_element(&mut self, element_id: &str) {
        self.elements.retain(|e| e.id != element_id);
    }
    
    // Поиск элемента по ID
    pub fn find_element(&self, element_id: &str) -> Option<&Element> {
        self.elements.iter().find(|e| e.id == element_id)
    }
    
    // Поиск элемента для редактирования по ID
    pub fn find_element_mut(&mut self, element_id: &str) -> Option<&mut Element> {
        self.elements.iter_mut().find(|e| e.id == element_id)
    }
    
    // Поиск элемента в точке (x, y)
    pub fn find_element_at_point(&self, point: (f32, f32)) -> Option<&Element> {
        self.elements.iter().rev().find(|e| e.contains_point(point))
    }
}