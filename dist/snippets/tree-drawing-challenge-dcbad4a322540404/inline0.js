
export function loadLottieAnimationAsync() {
    setTimeout(() => {
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
    }, 0); // 비동기로 실행
}
