use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "
export function loadLottieAnimation() {
    console.log('Lottie animation is starting...');
    lottie.loadAnimation({
        container: document.getElementById('lottie-snow-effect'),
        renderer: 'svg',
        loop: true,
        autoplay: true,
        path: '/tree-drawing-challenge/lottie/snow-effect.json',
        rendererSettings: {
        preserveAspectRatio: 'xMidYMid slice' // 전체 화면에 맞도록 크기 조정
        }
    });
}
")]

extern "C" {
    pub fn loadLottieAnimation();
}

pub fn start_snow_animation() {
    loadLottieAnimation(); // Ensure this runs after wasm initialization
}