mod app;
mod component;
mod func;
mod lottie;
mod upload;

use app::TreeDrawingChallenge;

fn main() {
    yew::Renderer::<TreeDrawingChallenge>::new().render();
}
