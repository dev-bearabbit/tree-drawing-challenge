
mod func;
mod app;
mod component;
mod lottie;

use app::TreeDrawingChallenge;

fn main() {
    yew::Renderer::<TreeDrawingChallenge>::new().render();
}
