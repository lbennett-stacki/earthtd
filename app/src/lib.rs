use crate::error_template::{AppError, ErrorTemplate};
use crate::game::draw_game;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::f64::consts::PI;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

pub mod error_template;
pub mod game;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="EarthTD"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (round, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|round| *round += 1);

    let round_text = move || {
        if round() > 0 {
            format!("Round {}", round())
        } else {
            "Start a game!".to_string()
        }
    };

    let next_round_text = move || {
        if round() > 0 {
            format!("Next round ({})", round() + 1)
        } else {
            "Start game".to_string()
        }
    };

    create_effect(move |_| {
        draw_game();
      });

    view! {
        <h1>"Welcome to EarthTD!"</h1>
        <h2>{round_text}</h2>
        <button on:click=on_click>{next_round_text}</button>

        <canvas width="500" height="500" id="canvas" />
    }
}
