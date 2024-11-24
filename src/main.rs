
mod func;
mod app;
mod component;
mod lottie;
mod upload;

use app::TreeDrawingChallenge;

fn main() {
    yew::Renderer::<TreeDrawingChallenge>::new().render();
}
