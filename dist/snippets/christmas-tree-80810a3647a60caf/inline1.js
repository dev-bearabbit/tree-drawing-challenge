
export function loadLottieAnimation() {
    lottie.loadAnimation({
        container: document.getElementById('lottie-snow-effect'),
        renderer: 'svg',
        loop: true,
        autoplay: true,
        path: 'https://lottie.host/3b6e7f49-ff4a-4c93-911e-77da0699be4b/y8geFWfYeX.json'
    });
}
