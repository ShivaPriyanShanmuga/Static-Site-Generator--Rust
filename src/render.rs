use crate::page::Page;
use tera::{Context, Tera};

pub struct Renderer {
    tera: Tera,
}

impl Renderer {
    pub fn new(template_dir: &str) -> Self {
        let template_pattern = format!("{}/**/*", template_dir);
        let tera: Tera = Tera::new(&template_pattern).expect("Failed to load templates");
        Renderer { tera }
    }

    pub fn render_page(&self, page: &Page) -> Result<String, tera::Error> {
        let mut context = Context::new();

        context.insert("title", &page.metadata.title);
        context.insert("content", &page.html);
        self.tera.render("base.html", &context)
    }
}