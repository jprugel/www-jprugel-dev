# www-jprugel-dev

Rebuild of my personal blog website using all rust based technologies.

## Why am I building my website again?
I wanted a website that I can add reactivity to and I wanted to do it in a language that I enjoy.

## What tools and technologies am I using?
- Database: Postgresql
- Server: Rocket-rs
- FrontEnd: Sycamore-rs
- Markdown Converter: markdown-rs
- Domain: Porkbun
- Hosting Service: Railway

## To Do List:
- [ ] Allow the server to handle more of the calculations.

## Standards
- The "Blog" is the whole site.
- The "Feed" is within the "Blog" and it contains "Articles"
- The DataBase is called "posts" but this is temporary
- The "articles" database will expand on posts with: 
  - id: int
  - title: str
  - date: date
  - hook: str
  - body: str
