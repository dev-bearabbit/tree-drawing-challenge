
export function initKakao(key) {
    Kakao.init(key);
}

export function shareKakao(options) {
    const parsedOptions = JSON.parse(options);
    Kakao.Share.sendDefault(parsedOptions);
}
