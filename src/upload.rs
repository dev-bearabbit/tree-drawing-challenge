use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, window};

pub async fn render_canvas(score: u32) -> Result<String, String> {

    let document = window()
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
    img.set_src("/image/background.jpg");

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
            if let Err(err) = context_clone.draw_image_with_html_image_element(&img_clone, 0.0, 0.0) {
                web_sys::console::error_1(&format!("Failed to draw image: {:?}", err).into());
                sender
                    .send(Err(format!("Failed to draw image: {:?}", err)))
                    .unwrap();
                return;
            }
    
            web_sys::console::log_1(&"Image drawn on canvas.".into());
    
            // 텍스트 위치 설정
            let base_x = 50.0; // "내 트리는" 시작 위치
            let base_y = 380.0;
    
            // "내 트리는" 텍스트
            context_clone.set_text_align("left");
            context_clone.set_text_baseline("middle");
            context_clone.set_fill_style_str("#FFFFFF"); // 흰색
            context_clone.set_font("bold 74px Pretendard");
            if let Err(err) = context_clone.fill_text("내 트리는", base_x, base_y) {
                web_sys::console::error_1(&format!("Failed to render text: {:?}", err).into());
                sender
                    .send(Err(format!("Failed to render text: {:?}", err)))
                    .unwrap();
                return;
            }
    
            // "내 트리는"의 끝 위치 계산
            let text_metrics = context_clone
                .measure_text("내 트리는")
                .map_err(|_| "Failed to measure text".to_string())
                .unwrap();
            let score_x = base_x + text_metrics.width() + 10.0; // "내 트리는" 끝 + 10px 여백
    
            // "00" 텍스트 (점수)
            context_clone.set_fill_style_str("#72F48F"); // 초록색
            context_clone.set_font("bold 74px Pretendard");
            if let Err(err) = context_clone.fill_text(&format!("{}", score), score_x, base_y) {
                web_sys::console::error_1(&format!("Failed to render score: {:?}", err).into());
                sender
                    .send(Err(format!("Failed to render score: {:?}", err)))
                    .unwrap();
                return;
            }
    
            // 점수 텍스트의 끝 위치 계산
            let score_metrics = context_clone
                .measure_text(&format!("{}", score))
                .map_err(|_| "Failed to measure score text".to_string())
                .unwrap();
            let point_x = score_x + score_metrics.width() + 10.0; // 점수 끝 + 10px 여백
    
            // "점" 텍스트
            context_clone.set_fill_style_str("#FFFFFF"); // 흰색
            context_clone.set_font("bold 74px Pretendard");
            if let Err(err) = context_clone.fill_text("점", point_x, base_y) {
                web_sys::console::error_1(&format!("Failed to render point: {:?}", err).into());
                sender
                    .send(Err(format!("Failed to render point: {:?}", err)))
                    .unwrap();
                return;
            }
    
            // 작은 글씨
            context_clone.set_fill_style_str("#61738A"); // 안흰색
            context_clone.set_font("bold 44px Pretendard");
            if let Err(err) = context_clone.fill_text("어디 한번 덤벼 보시지", base_x, base_y + 85.0) {
                web_sys::console::error_1(&format!("Failed to render small text: {:?}", err).into());
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


pub async fn upload_image(data_url: &str) -> Result<(String, String), String> {
    let api_key = "KEY";
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

        let data = js_sys::Reflect::get(&json, &"data".into()).unwrap();
        let url_viewer = js_sys::Reflect::get(&data, &"url_viewer".into()).unwrap()
        .as_string().ok_or_else(|| "url_viewer is not a string".to_string())?;
        let url = js_sys::Reflect::get(&data, &"url".into()).unwrap()
        .as_string().ok_or_else(|| "url_viewer is not a string".to_string())?;

    Ok((url, url_viewer))
}

