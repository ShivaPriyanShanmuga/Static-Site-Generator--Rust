# 🦀 Static Site Generator (SSG) in Rust  

A minimal and extensible static site generator built with Rust. Converts Markdown files into full HTML pages using pulldown-cmark for Markdown parsing and Tera for templating.  
  
---
  
## ✨ Features  
  
- Parses Markdown with front matter (YAML)  
- Supports custom HTML templates via Tera  
- Maintains folder structure in output  
- Fast and lightweight  
- Command-line input/output configuration  
  
---

## Demo  

cargo run -- --input content --output dist  

--input: Path to your folder of Markdown files  

--output: Path to where HTML files will be saved  
  
This will generate:  

dist/  
├── index.html  
└── blog/  
    └── my-post.html  
    
## 🏗 Folder Structure  
.  
├── content/           # Markdown source files (input)  
│   └── index.md  
├── templates/         # HTML templates for Tera  
│   └── base.html  
├── dist/              # Generated HTML files (output)  
├── src/               # Rust source code  
├── Cargo.toml         # Dependencies  
└── README.md  
  
## 🧪 Example Markdown File
\-\-\-  
title: "Welcome"  
\-\-\-  

\# Hello World!  
  
This is a **Markdown** page.  
  
## 🔧 Usage  
  
cargo run -- --input content --output dist  
  
--input: Path to your folder of Markdown files  
  
--output: Path to where HTML files will be saved  


## 📦 Dependencies :   
  
- pulldown-cmark  
  
- tera  
  
- serde_yaml  
  
- walkdir  
  
- clap  
  
## 🔨 Build & Run  
  
cargo build  
cargo run -- --input content --output dist  
