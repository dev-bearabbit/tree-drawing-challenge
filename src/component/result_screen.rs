use crate::func::format_time;
use crate::upload;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultScreenProps {
    pub score: u32,
    pub current_path: Vec<(f64, f64)>, // 사용자가 그린 점의 좌표
    pub on_retry: Callback<MouseEvent>,
    pub remaining_time: f64,
}

#[function_component(ResultScreen)]
pub fn result_screen(props: &ResultScreenProps) -> Html {
    // 상태 정의
    let is_share_section_visible = use_state(|| false); // 공유 섹션 표시 상태
    let image_url = use_state(|| None::<String>); // 업로드된 이미지 URL 상태

    let path_points = props
        .current_path
        .iter()
        .map(|(x, y)| format!("{},{}", x, y))
        .collect::<Vec<_>>()
        .join(" ");

    // 공유 버튼 핸들러
    let share_handler = {
        let is_share_section_visible = is_share_section_visible.clone();
        let image_url = image_url.clone();
        let score = props.score;

        Callback::from(move |_: MouseEvent| {
            wasm_bindgen_futures::spawn_local({
                let is_share_section_visible = is_share_section_visible.clone();
                let image_url = image_url.clone();

                async move {
                    web_sys::console::log_1(&"Starting canvas rendering...".into());

                    // 캔버스 렌더링
                    let data_url = match upload::render_canvas(score).await {
                        Ok(data_url) => {
                            web_sys::console::log_1(&"Canvas rendered successfully.".into());
                            data_url
                        }
                        Err(err) => {
                            web_sys::console::error_1(&format!("Canvas error: {}", err).into());
                            return; // 작업 중단
                        }
                    };

                    web_sys::console::log_1(&"Starting image upload...".into());

                    // 이미지 업로드
                    let uploaded_image_url = match upload::upload_image(&data_url).await {
                        Ok(image_url) => {
                            web_sys::console::log_1(&"Image uploaded successfully.".into());
                            image_url
                        }
                        Err(err) => {
                            web_sys::console::error_1(&format!("Upload error: {}", err).into());
                            return; // 작업 중단
                        }
                    };

                    // 상태 업데이트
                    is_share_section_visible.set(true);
                    image_url.set(Some(uploaded_image_url));
                }
            });
        })
    };

    let share_to_platform = {
        let image_url = image_url.clone(); // 상태 복사
        Callback::from(move |platform: String| {
            if let Some(url) = &*image_url {
                match platform.as_str() {
                    "facebook" => {
                        upload::share_to_facebook(url);
                    }
                    "twitter" => {
                        upload::share_to_twitter(url);
                    }
                    "web" => {
                        upload::share_to_web(url);
                    }
                    "download" => {
                        upload::share_to_download(url);
                    }
                    _ => {}
                }
            } else {
                web_sys::console::error_1(&"Image URL not available.".into());
            }
        })
    };
    
    
    html! {
        <div class="screen">
            <div class="result-sentence">
                <h3>{ "친구에게 도전장을 보내보세요!" }</h3>
            </div>

            <div class="score">
                <svg class="score-background" viewBox="0 0 125 67" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path fill-rule="evenodd" clip-rule="evenodd"
                        d="M8 0C3.58172 0 0 3.58172 0 8V48C0 52.4183 3.58172 56 8 56H53.3397L58.5359 65C60.0755 67.6667 63.9245 67.6667 65.4641 65L70.6603 56H117C121.418 56 125 52.4183 125 48V8C125 3.58172 121.418 0 117 0H8Z"
                        fill="#D9D9D9" />
                    <path fill-rule="evenodd" clip-rule="evenodd"
                        d="M8 0C3.58172 0 0 3.58172 0 8V48C0 52.4183 3.58172 56 8 56H53.3397L58.5359 65C60.0755 67.6667 63.9245 67.6667 65.4641 65L70.6603 56H117C121.418 56 125 52.4183 125 48V8C125 3.58172 121.418 0 117 0H8Z"
                        fill="url(#paint0_linear_5_8)" />
                    <defs>
                    <@{"linearGradient"} id="paint0_linear_5_8" x1="31.0157" y1="-48.9452" x2="161.433" y2="65.9978" gradientUnits="userSpaceOnUse">
                        <stop stop-color="#FFF983" />
                        <stop offset="0.585" stop-color="#83FFF1" />
                        <stop offset="0.955" stop-color="#8389FF" />
                    </@>
                    </defs>
                </svg>
                <div class="score-text">{ format!("{}점", props.score) }</div>
            </div>

            <div class="tree-container">

                { if props.score >= 70 {
                    yellow_star()
                } else {
                    dim_star()
                }}

                <svg class="tree-pattern"
                    viewBox="0 0 256 291"
                    preserveAspectRatio="xMidYMin"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    >
                    <path d="M121.15 8.50157L117.724 6.43576V6.43576L121.15 8.50157ZM134.851 8.50158L131.425 10.5674V10.5674L134.851 8.50158ZM35.8845 149.877L39.3097 151.942L35.8845 149.877ZM68.6952 162.008L72.1781 163.975C72.8776 162.737 72.8668 161.22 72.1498 159.992C71.4329 158.763 70.1175 158.008 68.6952 158.008V162.008ZM5.04591 274.704L1.56301 272.737H1.56301L5.04591 274.704ZM250.954 274.704L254.437 272.737L250.954 274.704ZM187.305 162.008V158.008C185.882 158.008 184.567 158.763 183.85 159.992C183.133 161.22 183.122 162.737 183.822 163.975L187.305 162.008ZM220.116 149.877L216.69 151.942V151.942L220.116 149.877ZM124.575 10.5674C126.13 7.98848 129.87 7.98849 131.425 10.5674L138.276 6.43576C133.61 -1.30098 122.39 -1.30097 117.724 6.43576L124.575 10.5674ZM39.3097 151.942L124.575 10.5674L117.724 6.43576L32.4592 147.811L39.3097 151.942ZM42.735 158.008C39.6217 158.008 37.7019 154.608 39.3097 151.942L32.4592 147.811C27.6356 155.809 33.3952 166.008 42.735 166.008V158.008ZM68.6952 158.008H42.735V166.008H68.6952V158.008ZM8.5288 276.671L72.1781 163.975L65.2123 160.041L1.56301 272.737L8.5288 276.671ZM12.0117 282.638C8.94932 282.638 7.0228 279.338 8.5288 276.671L1.56301 272.737C-2.95499 280.737 2.82455 290.638 12.0117 290.638V282.638ZM243.988 282.638H12.0117V290.638H243.988V282.638ZM247.471 276.671C248.977 279.338 247.051 282.638 243.988 282.638V290.638C253.175 290.638 258.955 280.737 254.437 272.737L247.471 276.671ZM183.822 163.975L247.471 276.671L254.437 272.737L190.788 160.041L183.822 163.975ZM213.265 158.008H187.305V166.008H213.265V158.008ZM216.69 151.942C218.298 154.608 216.378 158.008 213.265 158.008V166.008C222.605 166.008 228.364 155.809 223.541 147.811L216.69 151.942ZM131.425 10.5674L216.69 151.942L223.541 147.811L138.276 6.43576L131.425 10.5674Z" fill="white" fill-opacity="0.4"/>

                    <polyline
                        points={path_points}
                        stroke="#72F48F"
                        stroke-width="8"
                        fill="none"
                    />

                </svg>

                <button onclick={props.on_retry.clone()} class="retry-button">{ "↻ 다시 도전하기" }</button>

                <div class="timer">
                    { format_time(props.remaining_time) }
                </div>
            </div>
            <button class="start-button" onclick={share_handler}>{ "도전장 보내기" }</button>

            <div id="share-section" class={if *is_share_section_visible { "share-section show" } else { "share-section hidden" }}>
                <div class="share-container">
                    <div class="share-text">{ "🌲 친구에게 도전장 보내기 🌲" }</div>
                    <div class="icons">
                        <button class="icon-button" onclick={share_to_platform.reform(|_| "facebook".to_string())}><img src="image/facebook-icon.png" alt="Facebook"/></button>
                        <button class="icon-button" onclick={share_to_platform.reform(|_| "twitter".to_string())}><img src="image/x-icon.png" alt="Twitter" /></button>
                        <button class="icon-button" onclick={share_to_platform.reform(|_| "web".to_string())}><img src="image/link-icon.png" alt="Copy" /></button>
                        <button class="icon-button" onclick={share_to_platform.reform(|_| "download".to_string())}><img src="image/download-icon.png" alt="Download" /></button>
                    </div>
                </div>
            </div>
        </div>
    }
}

// 노란색 별 SVG
fn yellow_star() -> Html {
    html! {
    <svg class="star-yellow" viewBox="0 0 52 49" preserveAspectRatio="xMidYMin" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M24.3204 1.598C25.1084 0.379102 26.8916 0.379101 27.6796 1.598L35.1592 13.1676C35.429 13.585 35.8447 13.887 36.3251 14.0146L49.6397 17.553C51.0425 17.9258 51.5935 19.6216 50.6778 20.7477L41.9858 31.4364C41.6722 31.8221 41.5134 32.3107 41.5404 32.807L42.2897 46.5634C42.3687 48.0127 40.9261 49.0608 39.5721 48.5379L26.7206 43.5743C26.2569 43.3952 25.7431 43.3952 25.2794 43.5743L12.4279 48.5379C11.0739 49.0608 9.63133 48.0127 9.71027 46.5634L10.4596 32.807C10.4866 32.3107 10.3278 31.8221 10.0142 31.4364L1.32224 20.7477C0.40651 19.6216 0.957531 17.9258 2.36028 17.553L15.6749 14.0146C16.1553 13.887 16.571 13.585 16.8408 13.1676L24.3204 1.598Z" fill="#FFF983"/>
    </svg>
    }
}

// 흐릿한 별 SVG
fn dim_star() -> Html {
    html! {
        <svg class="star" viewBox="0 0 52 49" preserveAspectRatio="xMidYMin" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M24.3204 1.598C25.1084 0.379102 26.8916 0.379101 27.6796 1.598L35.1592 13.1676C35.429 13.585 35.8447 13.887 36.3251 14.0146L49.6397 17.553C51.0425 17.9258 51.5935 19.6216 50.6778 20.7477L41.9858 31.4364C41.6722 31.8221 41.5134 32.3107 41.5404 32.807L42.2897 46.5634C42.3687 48.0127 40.9261 49.0608 39.5721 48.5379L26.7206 43.5743C26.2569 43.3952 25.7431 43.3952 25.2794 43.5743L12.4279 48.5379C11.0739 49.0608 9.63133 48.0127 9.71027 46.5634L10.4596 32.807C10.4866 32.3107 10.3278 31.8221 10.0142 31.4364L1.32224 20.7477C0.40651 19.6216 0.957531 17.9258 2.36028 17.553L15.6749 14.0146C16.1553 13.887 16.571 13.585 16.8408 13.1676L24.3204 1.598Z" fill="#FFFFFF29"/>
        </svg>
    }
}
