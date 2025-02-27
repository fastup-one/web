use dioxus::prelude::*;
<<<<<<< HEAD
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
=======

>>>>>>> refactor-001
fn main() {
    launch(app);
}
<<<<<<< HEAD
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script {
            dangerous_inner_html: r#"
            document.addEventListener('DOMContentLoaded', function() {
                const hero = document.getElementById('hero');
                if (hero) {
                    // Create floating particles
                    for (let i = 0; i < 40; i++) {
                        const particle = document.createElement('div');
                        particle.classList.add('particle');
                        particle.style.width = Math.random() * 3 + 1 + 'px';
                        particle.style.height = particle.style.width;
                        particle.style.left = Math.random() * 100 + '%';
                        particle.style.top = Math.random() * 100 + '%';
                        particle.style.opacity = Math.random() * 0.5 + 0.1;
                        particle.style.backgroundColor = `hsl(${Math.random() * 60 + 210}, 100%, 70%)`;
                        hero.appendChild(particle);
                    }
                    
                    // Create a grid background
                    const grid = document.createElement('div');
                    grid.classList.add('bg-grid');
                    grid.style.position = 'absolute';
                    grid.style.inset = '0';
                    grid.style.opacity = '0.2';
                    grid.style.zIndex = '0';
                    grid.style.backgroundImage = `
                        linear-gradient(rgba(59, 130, 246, 0.1) 1px, transparent 1px),
                        linear-gradient(90deg, rgba(59, 130, 246, 0.1) 1px, transparent 1px)
                    `;
                    grid.style.backgroundSize = '40px 40px';
                    hero.insertBefore(grid, hero.firstChild);
                }
            });
            "#
        }
        Hero {}
    }
}
#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "min-h-screen bg-gradient-to-b from-indigo-950 via-slate-900 to-black relative overflow-hidden p-8",
            
            div {
                class: "relative z-10 flex flex-col items-center justify-center",
                
                // Header and title
                img { src: HEADER_SVG, id: "header", class: "w-48 mb-8 animate-float" }
                
                h1 { 
                    class: "text-4xl md:text-6xl font-bold mb-4 text-center text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-500",
                    "Dioxus" 
                }
                
                h2 {
                    class: "text-xl md:text-2xl text-blue-100 mb-10 text-center",
                    "User Interfaces that run anywhere"
                }
                
                // Status badge
                div {
                    class: "inline-flex items-center gap-2 px-3 py-1 bg-blue-900/30 border border-blue-700/30 rounded-full text-sm text-blue-300 mb-10",
                    span { class: "w-2 h-2 rounded-full bg-blue-400 animate-pulse" }
                    "Work in Progress"
                }
                
                // Progress bar
                div {
                    class: "w-full max-w-md mb-12",
                    div {
                        class: "flex justify-between text-xs text-blue-300/70 mb-2",
                        span { "Development Progress" }
                        span { "65%" }
                    }
                    div {
                        class: "h-1.5 bg-blue-900/50 rounded-full overflow-hidden",
                        div {
                            class: "h-full bg-gradient-to-r from-blue-400 to-indigo-500 rounded-full",
                            style: "width: 65%"
                        }
                    }
                }
            }
            
            // Links - keeping original structure but enhancing appearance
            div { 
                id: "links",
                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 max-w-5xl mx-auto relative z-10",
                
                a { 
                    href: "https://dioxuslabs.com/learn/0.6/", 
                    class: "flex items-center p-4 bg-slate-800/50 border border-slate-700/50 rounded-xl hover:bg-slate-700/50 hover:-translate-y-1 transition-all",
                    span { class: "text-2xl mr-3", "📚" }
                    div {
                        span { class: "font-medium block", "Learn Dioxus" }
                        span { class: "text-xs text-blue-300/70", "Documentation & tutorials" }
                    }
                }
                
                a { 
                    href: "https://dioxuslabs.com/awesome", 
                    class: "flex items-center p-4 bg-slate-800/50 border border-slate-700/50 rounded-xl hover:bg-slate-700/50 hover:-translate-y-1 transition-all",
                    span { class: "text-2xl mr-3", "🚀" }
                    div {
                        span { class: "font-medium block", "Awesome Dioxus" }
                        span { class: "text-xs text-blue-300/70", "Community resources & projects" }
                    }
                }
                
                a { 
                    href: "https://github.com/dioxus-community/", 
                    class: "flex items-center p-4 bg-slate-800/50 border border-slate-700/50 rounded-xl hover:bg-slate-700/50 hover:-translate-y-1 transition-all",
                    span { class: "text-2xl mr-3", "📡" }
                    div {
                        span { class: "font-medium block", "Community Libraries" }
                        span { class: "text-xs text-blue-300/70", "Extensions & integrations" }
                    }
                }
                
                a { 
                    href: "https://github.com/DioxusLabs/sdk", 
                    class: "flex items-center p-4 bg-slate-800/50 border border-slate-700/50 rounded-xl hover:bg-slate-700/50 hover:-translate-y-1 transition-all",
                    span { class: "text-2xl mr-3", "⚙️" }
                    div {
                        span { class: "font-medium block", "Dioxus Development Kit" }
                        span { class: "text-xs text-blue-300/70", "Tools for building with Dioxus" }
                    }
                }
                
                a { 
                    href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", 
                    class: "flex items-center p-4 bg-slate-800/50 border border-slate-700/50 rounded-xl hover:bg-slate-700/50 hover:-translate-y-1 transition-all",
                    span { class: "text-2xl mr-3", "💫" }
                    div {
                        span { class: "font-medium block", "VSCode Extension" }
                        span { class: "text-xs text-blue-300/70", "Editor integration & tooling" }
                    }
                }
                
                a { 
                    href: "https://discord.gg/XgGxMSkvUM", 
                    class: "flex items-center p-4 bg-slate-800/50 border border-slate-700/50 rounded-xl hover:bg-slate-700/50 hover:-translate-y-1 transition-all",
                    span { class: "text-2xl mr-3", "👋" }
                    div {
                        span { class: "font-medium block", "Community Discord" }
                        span { class: "text-xs text-blue-300/70", "Join the conversation" }
                    }
=======

fn app() -> Element {
    rsx! {
        div { 
            style: "min-height: 100vh; background: #000; color: white; font-family: system-ui, sans-serif;",
            
            div { 
                style: "max-width: 800px; margin: 0 auto; padding: 2rem; min-height: 100vh; display: flex; flex-direction: column; justify-content: center;",
                
                h1 { 
                    style: "font-size: 3rem; text-align: center; margin-bottom: 1rem;",
                    "FastUP"
                }
                
                p { 
                    style: "font-size: 1.5rem; text-align: center; color: #a3a3a3; margin-bottom: 3rem;",
                    "A revolutionary platform launching soon"
                }
                
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-bottom: 3rem;",
                    
                    div {
                        style: "background: #111; padding: 1.5rem; border-radius: 0.5rem;",
                        h3 { "Advanced Analytics" }
                        p { style: "color: #888;", "Real-time insights powered by cutting-edge algorithms" }
                    }
                    
                    div {
                        style: "background: #111; padding: 1.5rem; border-radius: 0.5rem;",
                        h3 { "Seamless Integration" }
                        p { style: "color: #888;", "Connect with your favorite tools" }
                    }
                    
                    div {
                        style: "background: #111; padding: 1.5rem; border-radius: 0.5rem;",
                        h3 { "AI-Powered" }
                        p { style: "color: #888;", "Intelligent automation" }
                    }
                }
                
                div {
                    style: "text-align: center;",
                    button {
                        style: "background: blue; color: white; font-weight: bold; padding: 0.75rem 2rem; border-radius: 0.5rem; border: none; cursor: pointer;",
                        onclick: move |_| {},
                        "Join Waitlist"
                    }
                }
                
                footer {
                    style: "margin-top: 2rem; text-align: center; color: #555; font-size: 0.875rem;",
                    "© 2025 Project Nebula • Built with Dioxus"
>>>>>>> refactor-001
                }
            }
        }
    }
}