site-generator/
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── src/
│ ├── main.rs # Основная точка входа
│ ├── app/ # Модуль для логики приложения
│ │ ├── mod.rs
│ │ ├── editor.rs # Редактор сайта
│ │ ├── project.rs # Управление проектом
│ │ └── export.rs # Экспорт сайта
│ ├── ui/ # Модуль для пользовательского интерфейса
│ │ ├── mod.rs
│ │ ├── components/ # Компоненты UI
│ │ │ ├── mod.rs
│ │ │ ├── button.rs
│ │ │ ├── text.rs
│ │ │ └── ...
│ │ └── windows/ # Окна приложения
│ │ ├── mod.rs
│ │ ├── main_window.rs
│ │ ├── editor_window.rs
│ │ └── ...
│ ├── models/ # Модели данных
│ │ ├── mod.rs
│ │ ├── page.rs # Модель страницы
│ │ ├── element.rs # Модель элемента
│ │ └── site.rs # Модель сайта
│ └── utils/ # Вспомогательные функции
│ ├── mod.rs
│ ├── file_io.rs
│ └── html_generator.rs
├── templates/ # Шаблоны для генерации
│ ├── page_template.html
│ └── ...
├── assets/ # Статичные ресурсы
│ ├── css/
│ └── js/
└── examples/ # Примеры сайтов
