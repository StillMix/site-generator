use crate::models::element::{Element, ElementType};
use crate::models::page::Page;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub struct HtmlGenerator;

impl HtmlGenerator {
    // Генерирует HTML для страницы
    pub fn generate_html(page: &Page, output_dir: &str) -> Result<String, std::io::Error> {
        // Создаем директорию для экспорта, если она не существует
        fs::create_dir_all(output_dir)?;
        
        // Путь к файлу страницы
        let file_path = format!("{}/{}", output_dir, page.file_name);
        
        // Читаем шаблон страницы
        let template = fs::read_to_string("templates/page_template.html")
            .unwrap_or_else(|_| "<!DOCTYPE html><html><head><title>{title}</title>{head}</head><body>{body}</body></html>".to_string());
        
        // Генерируем метатеги
        let mut meta_tags = String::new();
        for (name, content) in &page.meta_tags {
            meta_tags.push_str(&format!("<meta name=\"{}\" content=\"{}\">\n", name, content));
        }
        
        // Генерируем стили
        let mut styles = String::new();
        styles.push_str("<style>\n");
        for (selector, style) in &page.styles {
            styles.push_str(&format!("{} {{ {} }}\n", selector, style));
        }
        styles.push_str("</style>\n");
        
        // Генерируем скрипты
        let mut scripts = String::new();
        for script in &page.scripts {
            scripts.push_str(&format!("<script>{}</script>\n", script));
        }
        
        // Собираем head
        let head = format!("{}\n{}\n{}", meta_tags, styles, scripts);
        
        // Генерируем HTML для элементов
        let body = Self::generate_elements_html(&page.elements);
        
        // Подставляем в шаблон
        let html = template
            .replace("{title}", &page.title)
            .replace("{head}", &head)
            .replace("{body}", &body);
        
        // Записываем в файл
        let mut file = File::create(&file_path)?;
        file.write_all(html.as_bytes())?;
        
        Ok(file_path)
    }
    
    // Генерирует HTML для элементов
    fn generate_elements_html(elements: &[Element]) -> String {
        let mut html = String::new();
        
        for element in elements {
            html.push_str(&Self::generate_element_html(element));
        }
        
        html
    }
    
    // Генерирует HTML для одного элемента
    fn generate_element_html(element: &Element) -> String {
        let mut style = String::new();
        for (key, value) in &element.styles {
            style.push_str(&format!("{}:{};", key, value));
        }
        
        let mut attributes = String::new();
        for (key, value) in &element.attributes {
            if key != "style" {
                attributes.push_str(&format!(" {}=\"{}\"", key, value));
            }
        }
        
        // Позиционирование через CSS
        style.push_str(&format!("position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;",
            element.position.0, element.position.1, element.size.0, element.size.1));
        
        let style_attr = format!(" style=\"{}\"", style);
        
        // Генерируем HTML в зависимости от типа элемента
        match element.element_type {
            ElementType::Text => {
                format!("<div id=\"{}\"{}{}>{}</div>\n", 
                    element.id, style_attr, attributes, element.content)
            },
            ElementType::Button => {
                format!("<button id=\"{}\"{}{}>{}</button>\n", 
                    element.id, style_attr, attributes, element.content)
            },
            ElementType::Image => {
                let src = element.attributes.get("src").cloned().unwrap_or_default();
                let alt = element.attributes.get("alt").cloned().unwrap_or_default();
                format!("<img id=\"{}\" src=\"{}\" alt=\"{}\"{}>\n", 
                    element.id, src, alt, style_attr)
            },
            ElementType::Container => {
                let mut container_html = format!("<div id=\"{}\"{}{}>\n", 
                    element.id, style_attr, attributes);
                    
                // Добавляем дочерние элементы
                for child in &element.children {
                    container_html.push_str(&Self::generate_element_html(child));
                }
                
                container_html.push_str("</div>\n");
                container_html
            },
            ElementType::Form => {
                let mut form_html = format!("<form id=\"{}\"{}{}>\n", 
                    element.id, style_attr, attributes);
                    
                // Добавляем дочерние элементы
                for child in &element.children {
                    form_html.push_str(&Self::generate_element_html(child));
                }
                
                form_html.push_str("</form>\n");
                form_html
            },
            ElementType::Link => {
                let href = element.attributes.get("href").cloned().unwrap_or_default();
                format!("<a id=\"{}\" href=\"{}\"{}{}>{}</a>\n", 
                    element.id, href, style_attr, attributes, element.content)
            },
            ElementType::Custom(ref name) => {
                format!("<div id=\"{}\" data-type=\"{}\"{}{}>{}</div>\n", 
                    element.id, name, style_attr, attributes, element.content)
            }
        }
    }
    
    // Экспортирует весь проект сайта
    pub fn export_project(pages: &[Page], output_dir: &str) -> Result<String, std::io::Error> {
        // Создаем директорию для экспорта, если она не существует
        fs::create_dir_all(output_dir)?;
        
        // Создаем директории для ресурсов
        fs::create_dir_all(format!("{}/css", output_dir))?;
        fs::create_dir_all(format!("{}/js", output_dir))?;
        fs::create_dir_all(format!("{}/img", output_dir))?;
        
        // Генерируем HTML для каждой страницы
        for page in pages {
            Self::generate_html(page, output_dir)?;
        }
        
        Ok(output_dir.to_string())
    }
}