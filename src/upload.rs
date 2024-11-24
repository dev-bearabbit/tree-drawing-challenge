use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

pub async fn render_canvas(score: u32) -> Result<String, String> {
    web_sys::console::log_1(&"Initializing canvas...".into());

    let document = web_sys::window()
        .ok_or("Failed to get window")?
        .document()
        .ok_or("Failed to get document")?;

    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .map_err(|_| "Failed to create canvas element")?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| "Failed to cast to HtmlCanvasElement")?;

    web_sys::console::log_1(&"Canvas created.".into());

    canvas.set_width(800);
    canvas.set_height(800);

    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .map_err(|_| "Failed to get canvas context")?
        .ok_or("Failed to get 2D context")?
        .dyn_into()
        .map_err(|_| "Failed to cast to CanvasRenderingContext2d")?;

    web_sys::console::log_1(&"Canvas context initialized.".into());

    // 이미지 로드
    let img = HtmlImageElement::new().map_err(|_| "Failed to create HtmlImageElement")?;
    img.set_src("/tree-drawing-challenge/image/background.jpg");

    web_sys::console::log_1(&"Image element created. Waiting for onload...".into());

    // onerror 이벤트 추가
    let error_closure = Closure::wrap(Box::new(|| {
        web_sys::console::error_1(
            &"Failed to load image. Please check the path or network.".into(),
        );
    }) as Box<dyn Fn()>);
    img.set_onerror(Some(error_closure.as_ref().unchecked_ref()));
    error_closure.forget();

    let (sender, receiver) = futures::channel::oneshot::channel();
    let sender = Rc::new(RefCell::new(Some(sender)));

    let sender_clone = Rc::clone(&sender);
    let context_clone = context.clone();
    let img_clone = img.clone();

    let closure = Closure::wrap(Box::new(move || {
        web_sys::console::log_1(&"Image loaded.".into());

        if let Some(sender) = sender_clone.borrow_mut().take() {
            if let Err(err) = context_clone.draw_image_with_html_image_element(&img_clone, 0.0, 0.0)
            {
                web_sys::console::error_1(&format!("Failed to draw image: {:?}", err).into());
                sender
                    .send(Err(format!("Failed to draw image: {:?}", err)))
                    .unwrap();
                return;
            }

            web_sys::console::log_1(&"Image drawn on canvas.".into());

            context_clone.set_text_align("right");
            context_clone.set_text_baseline("middle");
            // 점수 (다른 색상 적용)
            context_clone.set_fill_style_str("#72F48F"); // 점수 색상
            context_clone.set_font("bold 60px Pretendard"); // 점수 폰트
            if let Err(err) = context_clone.fill_text(&format!("{}", score), 375.0, 380.0) {
                web_sys::console::error_1(
                    &format!("Failed to render score text: {:?}", err).into(),
                );
                sender
                    .send(Err(format!("Failed to render score text: {:?}", err)))
                    .unwrap();
                return;
            }

            context_clone.set_text_align("left"); // 텍스트 정렬: 왼쪽
            context_clone.set_text_baseline("middle"); // 텍스트 기준선: 중간
                                                       // 큰 글씨
            context_clone.set_fill_style_str("#FFFFFF"); // 큰 글씨 색상
            context_clone.set_font("bold 60px Pretendard"); // 큰 글씨 폰트
            if let Err(err) = context_clone.fill_text("내 트리는", 50.0, 380.0) {
                web_sys::console::error_1(
                    &format!("Failed to render large text: {:?}", err).into(),
                );
                sender
                    .send(Err(format!("Failed to render large text: {:?}", err)))
                    .unwrap();
                return;
            }

            // 큰 글씨
            context_clone.set_fill_style_str("#FFFFFF"); // 큰 글씨 색상
            context_clone.set_font("bold 60px Pretendard"); // 큰 글씨 폰트
            if let Err(err) = context_clone.fill_text("점", 380.0, 380.0) {
                web_sys::console::error_1(
                    &format!("Failed to render large text: {:?}", err).into(),
                );
                sender
                    .send(Err(format!("Failed to render large text: {:?}", err)))
                    .unwrap();
                return;
            }

            // 작은 글씨
            context_clone.set_fill_style_str("#FFFFFF"); // 작은 글씨 색상
            context_clone.set_font("bold 35px Pretendard"); // 작은 글씨 폰트
            if let Err(err) = context_clone.fill_text("어디 한번 덤벼 보시지", 50.0, 450.0)
            {
                web_sys::console::error_1(
                    &format!("Failed to render small text: {:?}", err).into(),
                );
                sender
                    .send(Err(format!("Failed to render small text: {:?}", err)))
                    .unwrap();
                return;
            }

            web_sys::console::log_1(&"Text rendered on canvas.".into());
            sender.send(Ok(())).unwrap();
        }
    }) as Box<dyn Fn()>);

    img.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    // 이미지 로드 완료 대기
    receiver
        .await
        .map_err(|_| "Image loading failed".to_string())??;

    web_sys::console::log_1(&"Canvas rendering complete.".into());

    // 캔버스를 데이터 URL로 변환
    canvas
        .to_data_url()
        .map_err(|_| "Failed to convert canvas to data URL".to_string())
}

pub async fn upload_image(data_url: &str) -> Result<String, String> {
    let api_key = "2fc4f7a32019bd384305c71135034668";
    let base64_image = data_url.split(',').nth(1).ok_or("Invalid data URL")?;

    let form_data = web_sys::FormData::new().map_err(|_| "Failed to create FormData")?;
    form_data
        .append_with_str("key", api_key)
        .map_err(|_| "Failed to append API key to FormData")?;
    form_data
        .append_with_str("image", base64_image)
        .map_err(|_| "Failed to append image to FormData")?;

    let window = web_sys::window().ok_or("No window available")?;
    let ans = web_sys::RequestInit::new();
    ans.set_method("POST");
    ans.set_body(&form_data);
    let fetch = window.fetch_with_str_and_init("https://api.imgbb.com/1/upload", &ans);

    let response = wasm_bindgen_futures::JsFuture::from(fetch)
        .await
        .map_err(|_| "Failed to fetch from ImgBB")?
        .dyn_into::<web_sys::Response>()
        .map_err(|_| "Failed to convert fetch response")?;

    if !response.ok() {
        return Err("ImgBB API request failed".to_string());
    }

    let json = wasm_bindgen_futures::JsFuture::from(response.json().unwrap())
        .await
        .map_err(|_| "Failed to parse ImgBB response")?;

    let url = js_sys::Reflect::get(&json, &"data".into())
        .and_then(|data| js_sys::Reflect::get(&data, &"url".into()))
        .map_err(|_| "Failed to extract URL from ImgBB response")?;

    url.as_string()
        .ok_or("Failed to convert URL to String".to_string())
}

pub fn share_to_twitter(image_url: &str) {
    let twitter_url = format!(
        "https://twitter.com/intent/tweet?text={}&url={}",
        "🎄트리 그리기 챌린지🎄 친구에게 도전해 보세요", image_url
    );

    if let Some(window) = web_sys::window() {
        // `open_with_url`이 실패하면 에러 출력
        if let Err(err) = window.open_with_url(&twitter_url) {
            web_sys::console::error_1(&format!("Failed to open Twitter: {:?}", err).into());
        }
    } else {
        web_sys::console::error_1(&"No window object available.".into());
    }
}
