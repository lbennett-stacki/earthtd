use crate::error_template::{AppError, ErrorTemplate};
use crate::game::draw_game;
use engine::{Game, Mob};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::sync::{Mutex, OnceLock};

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

fn the_game() -> &'static Mutex<Game> {
    static GAME: OnceLock<Mutex<Game>> = OnceLock::new();

    GAME.get_or_init(|| {
        let width: i64 = 1000;
        let height: i64 = 1000;
        let box_size: i64 = 10;

        Mutex::new(Game::new(width, height, box_size))
    })
}

#[server(StartGame, "/api")]
pub async fn start_game() -> Result<Vec<Mob>, ServerFnError> {
    let mut game = the_game().lock().unwrap();
    game.start();
    Ok(game.mobs())
}

#[server(NextRound, "/api")]
pub async fn next_round() -> Result<Vec<Mob>, ServerFnError> {
    let mut game = the_game().lock().unwrap();
    game.next_round();
    Ok(game.mobs())
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let width: i64 = 1000;
    let height: i64 = 1000;
    let box_size: i64 = 10;

    let (mobs, set_mobs) = create_signal(vec![]);

    let (round, set_count) = create_signal(0);

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
        draw_game(width, height, box_size, mobs.get());
    });

    view! {
        <h1>"Welcome to EarthTD!"</h1>
        <h2>{round_text}</h2>
        <button on:click=move |_| {
            spawn_local(async move {
                let result = next_round().await;

                match result {
                    Ok(mobs) => {
                        log::info!("Next round!");

                        set_mobs(mobs);
                    }
                    Err(err) => {
                        log::error!("Error: {:#?}", err);
                    }
                }

            });
        }>{next_round_text}</button>

        <canvas width="1000" height="1000" id="canvas" />
    }
}
