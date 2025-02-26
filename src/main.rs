use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Document head elements
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        // Background effect script
        script { 
            dangerous_inner_html: r#"
            document.addEventListener('DOMContentLoaded', () => {
                const container = document.getElementById('particles-container');
                if (!container) return;
                
                for (let i = 0; i < 50; i++) {
                    createParticle(container);
                }
                
                function createParticle(container) {
                    const particle = document.createElement('div');
                    particle.classList.add('particle');
                    
                    // Random size between 2-6px
                    const size = Math.random() * 4 + 2;
                    particle.style.width = `${size}px`;
                    particle.style.height = `${size}px`;
                    
                    // Random position
                    particle.style.left = `${Math.random() * 100}%`;
                    particle.style.top = `${Math.random() * 100}%`;
                    
                    // Random opacity
                    particle.style.opacity = `${Math.random() * 0.5 + 0.1}`;
                    
                    // Random animation duration
                    const duration = Math.random() * 15 + 10;
                    particle.style.animationDuration = `${duration}s`;
                    
                    // Random animation delay
                    particle.style.animationDelay = `${Math.random() * 5}s`;
                    
                    // Random colors in blue/purple spectrum
                    const hue = Math.random() * 60 + 220; // Blue to purple range
                    particle.style.backgroundColor = `hsl(${hue}, 70%, 60%)`;
                    
                    container.appendChild(particle);
                }
            });
            "#
        }
        
        // Main app container with background effects
        div { 
            class: "min-h-screen relative overflow-hidden bg-gradient-to-br from-slate-950 to-indigo-950",
            
            // Particles container
            div { id: "particles-container", class: "absolute inset-0 z-0" }
            
            // Grid background
            div { 
                class: "absolute inset-0 z-0 bg-grid opacity-30" 
            }
            
            // Content container
            div { 
                class: "relative z-10 container mx-auto px-4 py-8",
                
                // Navbar
                nav { 
                    class: "flex justify-between items-center mb-12",
                    
                    // Logo
                    div { 
                        class: "text-2xl font-bold text-gradient-blue-purple",
                        "DioxusLabs"
                    }
                    
                    // Menu button (placeholder for mobile menu)
                    div { 
                        class: "md:hidden",
                        button { 
                            class: "text-white",
                            "☰"
                        }
                    }
                    
                    // Nav links
                    div { 
                        class: "hidden md:flex space-x-6",
                        a { class: "text-white/80 hover:text-white transition-all", href: "#", "Home" }
                        a { class: "text-white/80 hover:text-white transition-all", href: "#", "About" }
                        a { class: "text-white/80 hover:text-white transition-all", href: "#", "Contact" }
                    }
                }
                
                // Hero section
                section { 
                    class: "text-center py-20",
                    
                    h1 { 
                        class: "text-4xl md:text-6xl lg:text-7xl font-bold mb-6 text-gradient-blue-purple animate-fade-in",
                        "Something Amazing"
                        br {}
                        "is Coming Soon"
                    }
                    
                    p { 
                        class: "text-xl text-white/80 mb-10 max-w-2xl mx-auto animate-fade-in animation-delay-200",
                        "We're working hard to build the next generation of web experiences with Rust and Dioxus. Stay tuned for updates!"
                    }
                    
                    div { 
                        class: "flex flex-col sm:flex-row justify-center gap-4 mb-16 animate-fade-in animation-delay-400",
                        
                        a { 
                            class: "btn-primary glow-hover",
                            href: "#subscribe",
                            "Get Notified"
                        }
                        
                        a { 
                            class: "btn-secondary",
                            href: "https://github.com/DioxusLabs",
                            "View on GitHub"
                        }
                    }
                    
                    // Progress indicator
                    div { 
                        class: "max-w-lg mx-auto mb-12",
                        
                        div { 
                            class: "flex justify-between text-sm text-white/70 mb-2",
                            span { "Development Progress" }
                            span { "65%" }
                        }
                        
                        div { 
                            class: "h-2 bg-white/10 rounded-full overflow-hidden",
                            div { 
                                class: "h-full bg-gradient-to-r from-blue-500 to-purple-500 rounded-full relative",
                                style: "width: 65%",
                                
                                // Glow dot at the end of progress bar
                                div { 
                                    class: "absolute right-0 top-1/2 -translate-y-1/2 w-4 h-4 bg-white rounded-full glow-effect" 
                                }
                            }
                        }
                    }
                    
                    // Header image with floating animation
                    div { 
                        class: "relative mb-16",
                        img { 
                            src: HEADER_SVG, 
                            id: "header", 
                            class: "max-w-full h-auto mx-auto animate-float" 
                        }
                    }
                    
                    // Links section
                    div { 
                        id: "links",
                        class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 max-w-4xl mx-auto",
                        
                        LinkCard { href: "https://dioxuslabs.com/learn/0.6/", emoji: "📚", title: "Learn Dioxus" }
                        LinkCard { href: "https://dioxuslabs.com/awesome", emoji: "🚀", title: "Awesome Dioxus" }
                        LinkCard { href: "https://github.com/dioxus-community/", emoji: "📡", title: "Community Libraries" }
                        LinkCard { href: "https://github.com/DioxusLabs/sdk", emoji: "⚙️", title: "Dioxus Development Kit" }
                        LinkCard { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", emoji: "💫", title: "VSCode Extension" }
                        LinkCard { href: "https://discord.gg/XgGxMSkvUM", emoji: "👋", title: "Community Discord" }
                    }
                }
                
                // Newsletter section
                section { 
                    id: "subscribe",
                    class: "py-16 max-w-xl mx-auto",
                    
                    h2 { 
                        class: "text-3xl font-bold mb-6 text-gradient-blue-purple text-center",
                        "Stay Updated"
                    }
                    
                    p { 
                        class: "text-white/80 mb-8 text-center",
                        "Join our newsletter to get notified when we launch. No spam, just important updates."
                    }
                    
                    form { 
                        class: "flex flex-col sm:flex-row gap-2",
                        
                        input { 
                            class: "flex-1 px-4 py-3 rounded-lg bg-white/10 border border-white/20 focus:border-blue-500 outline-none text-white placeholder:text-white/50",
                            placeholder: "Enter your email",
                            r#type: "email"
                        }
                        
                        button { 
                            class: "btn-primary whitespace-nowrap",
                            r#type: "submit",
                            "Notify Me"
                        }
                    }
                }
                
                // Footer
                footer { 
                    class: "mt-24 pt-8 border-t border-white/10 text-white/60",
                    
                    div { 
                        class: "flex flex-col md:flex-row justify-between items-center",
                        
                        div { 
                            class: "mb-4 md:mb-0",
                            "© 2025 DioxusLabs. All rights reserved."
                        }
                        
                        div { 
                            class: "flex space-x-6",
                            a { class: "hover:text-white transition-colors", href: "#", "Twitter" }
                            a { class: "hover:text-white transition-colors", href: "#", "GitHub" }
                            a { class: "hover:text-white transition-colors", href: "#", "Discord" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LinkCard(href: String, emoji: String, title: String) -> Element {
    rsx! {
        a {
            href: "{href}",
            class: "flex items-center p-4 bg-white/5 hover:bg-white/10 border border-white/10 rounded-xl transition-all transform hover:-translate-y-1 hover:shadow-glow",
            
            span { class: "text-2xl mr-3", "{emoji}" }
            span { class: "font-medium", "{title}" }
        }
    }
}
