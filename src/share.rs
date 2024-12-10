use urlencoding::encode;
use serde_json::json;
use web_sys::{window, console};
use wasm_bindgen::prelude::*;

pub fn share_to_twitter(image_url: &str, myscore: &str) {

    // 트윗 텍스트와 이미지 URL 인코딩
    let image_url_encoded = encode(image_url);
    let score_message = format!("내 점수는 {}점! 너도 도전해볼래?", myscore);
    let tweet_text = format!(
        "{}%0A{}%0A{}",
        encode("🎄트리 그리기 챌린지🎄"),
        encode("https://drawtree.netlify.app"),
        encode(&score_message)
    );

    // 트위터 intent URL 생성
    let twitter_url = format!(
        "https://twitter.com/intent/tweet?original_referer={}&url={}&text={}",
       image_url_encoded, image_url_encoded, tweet_text
    );

    // 현재 페이지를 트위터 링크로 리디렉션
    if let Some(window) = window() {
        if let Err(err) = window.location().set_href(&twitter_url) {
            web_sys::console::error_1(&format!("Failed to redirect: {:?}", err).into());
        }
    } else {
        web_sys::console::error_1(&"Window object not available.".into());
    }
}

pub fn share_to_facebook(image_url: &str, myscore: &str) {

    // URL 인코딩
    let image_url_encoded = encode(image_url);
    let score_message = format!("내 점수는 {}점! 너도 도전해볼래?", myscore);
    let text = format!(
        "{}%0A{}%0A{}",
        encode("🎄트리 그리기 챌린지🎄"),
        encode("https://drawtree.netlify.app"),
        encode(&score_message),
    );
    // facebook intent URL 생성
    let facebook_url = format!(
        "https://www.facebook.com/share.php?u={}&quote={}",
       image_url_encoded, text
    );

    // 현재 페이지를 페이스북 링크로 리디렉션
    if let Some(window) = window() {
        if let Err(err) = window.location().set_href(&facebook_url) {
            web_sys::console::error_1(&format!("Failed to redirect: {:?}", err).into());
        }
    } else {
        web_sys::console::error_1(&"Window object not available.".into());
    }
}

// JavaScript 초기화 함수 호출
#[wasm_bindgen(inline_js = "
export function initKakao(key) {
    Kakao.init(key);
}

export function shareKakao(options) {
    const parsedOptions = JSON.parse(options);
    Kakao.Share.sendDefault(parsedOptions);
}
")]
extern "C" {
    pub fn initKakao(key: &str);
    pub fn shareKakao(options: &str);
}

pub fn share_to_kakao(image_url: &str, myscore: &str) {

    let app_key = "KEY";
    initKakao(app_key);
    console::log_1(&"Kakao SDK Initialized!".into());

    let options = json!({
        "objectType": "feed",
        "content": {
            "title": "🎄트리 그리기 챌린지🎄",
            "description": format!("내 점수는 {}점! 너도 도전해볼래?", myscore),
            "imageUrl": image_url,
            "link": {
                "mobileWebUrl": "https://drawtree.netlify.app",
                "webUrl": "https://drawtree.netlify.app"
            }
        },
        "buttons": [
            {
                "title": "도전하러 가기",
                "link": {
                    "mobileWebUrl": "https://drawtree.netlify.app",
                    "webUrl": "https://drawtree.netlify.app"
                }
            }
        ]
    })
    .to_string();

    shareKakao(&options);
}


#[wasm_bindgen(inline_js = "
export function copyToClipboard(text) {
    navigator.clipboard.writeText(text)
        .then(() => {
            console.log('copy complete:', text);
        })
        .catch(err => {
            console.error('copy failed:', err);
        });
}
")]
extern "C" {
    pub fn copyToClipboard(text: &str);
}


