use clap::Parser;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
mod page;
use page::{Page, PageMetadata};
use std::fs;
mod render;
use render::Renderer;


#[derive(Parser)]
#[command(author, version, about = "Builds a static website from markdown files.")]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long, default_value = "dist")]
    output: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    println!("The input directory is {}", args.input);
    println!("The output directory is {}", args.output);

    let markdown_files = collect_markdown_files(&args.input);
    println!("Found {} Markdown Files", markdown_files.len());

    for path in &markdown_files {
        println!("- {}", path.display());
    }

    let renderer = Renderer::new("templates");

    for path in markdown_files {
        match parse_markdown_file(&path) {
            Ok(page) => {
                let title = page.metadata.title.clone().unwrap_or_else(|| "untitled".to_string());
                println!("Parsed: {} -> Title: {}", path.display(), title);
                println!("HTML preview:\n{}\n", page.html.lines().take(5).collect::<Vec<_>>().join("\n"));

                match renderer.render_page(&page) {
                    Ok(rendered) => {
                        if let Err(e) = write_html_output(&page, &rendered, &args.input, &args.output) {
                            eprintln!("Failed to write HTML : {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to render to html {} : {}\n", path.display(), e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse {}: {}", path.display(), e);
            }
        }
    }
}

fn collect_markdown_files<P: AsRef<Path>>(input: P) -> Vec<std::path::PathBuf> {
    WalkDir::new(input)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "md")
        })
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

fn parse_markdown_file(path: &std::path::Path) -> Result<Page, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;

    let (metadata, body) = if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() == 3 {
            let yaml_str = parts[1];
            let body = parts[2].trim().to_string();
            let metadata: PageMetadata = serde_yaml::from_str(yaml_str)?;
            (metadata, body)
        }
        else {
            (PageMetadata {title: None, date: None}, content)
        }
    }
    else {
        (PageMetadata {title: None, date: None}, content)
    };

    let html = markdown_to_html(&body);

    Ok(Page {
        metadata, 
        content: body,
        html,
        source_path: path.to_path_buf(),
    })
}

fn markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::{Parser, Options, html};
    
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn write_html_output(page: &Page, rendered_html: &str, input_dir: &str, output_dir: &str) -> std::io::Result<()> {
    let relative_path = page.source_path.strip_prefix(input_dir).unwrap();
    let mut output_path = PathBuf::from(output_dir);
    output_path.push(relative_path);
    output_path.set_extension("html");

    if let Some(parent) = output_path.parent(){
        fs::create_dir_all(parent)?;
    }

    fs::write(&output_path, rendered_html)?;

    println!("saved to : {}", output_path.display());
    Ok(())
}