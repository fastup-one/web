use dioxus::prelude::*;

fn main() {
    launch(app);
}

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
                }
            }
        }
    }
}