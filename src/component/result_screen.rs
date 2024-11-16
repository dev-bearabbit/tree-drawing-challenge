use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultScreenProps {
    pub score: u32,
    pub pattern: Vec<(f64, f64)>, // 트리 밑그림 좌표
    pub circles: Vec<(f64, f64)>, // 사용자가 그린 점의 좌표
    pub on_retry: Callback<MouseEvent>,
}

#[function_component(ResultScreen)]
pub fn result_screen(props: &ResultScreenProps) -> Html {

    html! {
        <div class="result-screen">

            <div class="sentence">
                <h3>{ "친구에게 도전장을 보내보세요!" }</h3>
                <h1>{ format!("내 트리는 {}점", props.score) }</h1>
            </div>

            <div class="tree-container">
                <svg width="100%" height="250" class="result-drawing">

                    { if props.score >= 90 {
                        yellow_star()
                    } else {
                        dim_star()
                    }}

                    <polyline
                        points={props.pattern.iter().map(|(x, y)| format!("{},{}", x, y)).collect::<Vec<_>>().join(" ")}
                        stroke="#FFFFFF66"
                        stroke-width="5"
                        fill="none"
                    />
                    
                    <text x="100" y="2000" font-size="24" text-anchor="middle" fill="white" dominant-baseline="middle">
                    { "result" }
                </text>

                    { for props.circles.iter().map(|(x, y)| html! {
                        <circle cx={format!("{}", x)} cy={format!("{}", y)} r="5" fill="#72F48F" />
                    })}
                </svg>
            </div>
            <button onclick={props.on_retry.clone()} class="retry-button">{ "다시 도전하기" }</button>
            <button class="start-button">{ "도전장 보내기" }</button>
        </div>
    }
}


// 노란색 별 SVG
fn yellow_star() -> Html {
    html! {
    <svg x="85" y="15" width="30" height="30" viewBox="0 0 52 49" fill="none" xmlns="http://www.w3.org/2000/svg">
                <g filter="url(#filter0_d_9_248)">
                    <path d="M56.3204 33.598C57.1084 32.3791 58.8916 32.3791 59.6796 33.598L67.1592 45.1676C67.429 45.585 67.8447 45.887 68.3251 46.0146L81.6397 49.553C83.0425 49.9258 83.5935 51.6216 82.6778 52.7477L73.9858 63.4364C73.6722 63.8221 73.5134 64.3107 73.5404 64.807L74.2897 78.5634C74.3687 80.0127 72.9261 81.0608 71.5721 80.5379L58.7206 75.5743C58.2569 75.3952 57.7431 75.3952 57.2794 75.5743L44.4279 80.5379C43.0739 81.0608 41.6313 80.0127 41.7103 78.5634L42.4596 64.807C42.4866 64.3107 42.3278 63.8221 42.0142 63.4364L33.3222 52.7477C32.4065 51.6216 32.9575 49.9258 34.3603 49.553L47.6749 46.0146C48.1553 45.887 48.571 45.585 48.8408 45.1676L56.3204 33.598Z" fill="#FFF983"/>
                </g>
                <defs>
                    <filter id="filter0_d_9_248" x="0.872559" y="0.683823" width="114.255" height="111.991" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                        <feflood flood-opacity="0" result="BackgroundImageFix"/>
                        <fecolormatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0" result="hardAlpha"/>
                        <feoffset/>
                        <fegaussianblur stdDeviation="16"/>
                        <fecomposite in2="hardAlpha" operator="out"/>
                        <fecolormatrix type="matrix" values="0 0 0 0 1 0 0 0 0 0.94902 0 0 0 0 0 0 0 0 1 0"/>
                        <feblend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow_9_248"/>
                        <feblend mode="normal" in="SourceGraphic" in2="effect1_dropShadow_9_248" result="shape"/>
                    </filter>
                </defs>
            </svg>
        }
}


// 흐릿한 별 SVG
fn dim_star() -> Html {
    html! {
        <svg x="85" y="15" width="30" height="30" viewBox="0 0 52 49" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M24.3204 1.598C25.1084 0.379102 26.8916 0.379101 27.6796 1.598L35.1592 13.1676C35.429 13.585 35.8447 13.887 36.3251 14.0146L49.6397 17.553C51.0425 17.9258 51.5935 19.6216 50.6778 20.7477L41.9858 31.4364C41.6722 31.8221 41.5134 32.3107 41.5404 32.807L42.2897 46.5634C42.3687 48.0127 40.9261 49.0608 39.5721 48.5379L26.7206 43.5743C26.2569 43.3952 25.7431 43.3952 25.2794 43.5743L12.4279 48.5379C11.0739 49.0608 9.63133 48.0127 9.71027 46.5634L10.4596 32.807C10.4866 32.3107 10.3278 31.8221 10.0142 31.4364L1.32224 20.7477C0.40651 19.6216 0.957531 17.9258 2.36028 17.553L15.6749 14.0146C16.1553 13.887 16.571 13.585 16.8408 13.1676L24.3204 1.598Z" fill="#FFFFFF29"/>
        </svg>
    }
}
