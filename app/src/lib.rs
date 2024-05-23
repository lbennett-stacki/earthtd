use crate::error_template::{AppError, ErrorTemplate};
use crate::game::draw_game;
use engine::{Game, Mob};
use leptos::html::Canvas;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{signal_throttled, use_mouse_in_element};
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

pub const WIDTH: i64 = 1000;
pub const HEIGHT: i64 = 200;
pub const BOX_SIZE: i64 = 10;

fn the_game() -> &'static Mutex<Game> {
    static GAME: OnceLock<Mutex<Game>> = OnceLock::new();

    GAME.get_or_init(|| Mutex::new(Game::new(WIDTH, HEIGHT, BOX_SIZE)))
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

    let target = create_node_ref::<Canvas>();
    let mouse = use_mouse_in_element(target);

    let throttled_x: Signal<f64> = signal_throttled(mouse.element_x, 500.0);
    let throttled_y: Signal<f64> = signal_throttled(mouse.element_y, 500.0);

    create_effect(move |_| {
        draw_game(
            WIDTH,
            HEIGHT,
            BOX_SIZE,
            mobs.get(),
            throttled_x.get(),
            throttled_y.get(),
        );
    });

    view! {
        <h1>"Welcome to EarthTD!"</h1>
        <h2>{round_text}</h2>
        <code>{move || (mouse.element_x.get() / BOX_SIZE as f64).ceil()} x { move || (mouse.element_y.get() / BOX_SIZE as f64).ceil()}</code>
        <div>
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

            <div on>
                // Plus 1 for border
                <canvas node_ref=target width=WIDTH+1 height=HEIGHT+1 id="canvas" />
            </div>
        </div>
    }
}
