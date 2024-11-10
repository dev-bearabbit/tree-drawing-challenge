
mod component;
mod logic;

use component::TreeDrawingChallenge;

fn main() {
    yew::Renderer::<TreeDrawingChallenge>::new().render();
}
