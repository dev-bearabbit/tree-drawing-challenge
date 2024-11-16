use yew::prelude::*;
use web_sys::TouchEvent;

#[derive(Properties, PartialEq)]
pub struct DrawingScreenProps {
    pub remaining_time: f64,
    pub svg_ref: NodeRef,
    pub pattern: Vec<(f64, f64)>,
    pub circles: Vec<(f64, f64)>,
    pub on_start_draw: Callback<TouchEvent>,
    pub on_update_draw: Callback<TouchEvent>,
    pub on_end_draw: Callback<TouchEvent>,
}

#[function_component(DrawingScreen)]
pub fn drawing_screen(props: &DrawingScreenProps) -> Html {
    html! {
        <div class="drawing-screen">
            <div class="sentence">
                <h3>
                    { "트리 모양 선을 따라" }
                    <br />
                    { "빠르게 그려주세요" }
                </h3>
            </div>
            <div class="tree-container">
                <svg
                    ref={props.svg_ref.clone()}
                    width="100%"
                    height="250"
                    ontouchstart={props.on_start_draw.clone()}
                    ontouchmove={props.on_update_draw.clone()}
                    ontouchend={props.on_end_draw.clone()}
                    class="drawing-area"
                >
                <svg x="85" y="15" width="30" height="30" viewBox="0 0 52 49" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M24.3204 1.598C25.1084 0.379102 26.8916 0.379101 27.6796 1.598L35.1592 13.1676C35.429 13.585 35.8447 13.887 36.3251 14.0146L49.6397 17.553C51.0425 17.9258 51.5935 19.6216 50.6778 20.7477L41.9858 31.4364C41.6722 31.8221 41.5134 32.3107 41.5404 32.807L42.2897 46.5634C42.3687 48.0127 40.9261 49.0608 39.5721 48.5379L26.7206 43.5743C26.2569 43.3952 25.7431 43.3952 25.2794 43.5743L12.4279 48.5379C11.0739 49.0608 9.63133 48.0127 9.71027 46.5634L10.4596 32.807C10.4866 32.3107 10.3278 31.8221 10.0142 31.4364L1.32224 20.7477C0.40651 19.6216 0.957531 17.9258 2.36028 17.553L15.6749 14.0146C16.1553 13.887 16.571 13.585 16.8408 13.1676L24.3204 1.598Z" fill="#FFFFFF29"/>
                </svg>

                    <polyline
                        points={props.pattern.iter().map(|(x, y)| format!("{},{}", x, y)).collect::<Vec<_>>().join(" ")}
                        stroke="#FFFFFF66"
                        stroke-width="5"
                        fill="none"
                    />

                <text x="100" y="200" font-size="24" text-anchor="middle" fill="white" dominant-baseline="middle">
                    { format_time(props.remaining_time) }
                </text>

                    { for props.circles.iter().map(|(x, y)| html! {
                        <circle cx={format!("{}", x)} cy={format!("{}", y)} r="5" fill="#72F48F" />
                    })}

                </svg>

            </div>
        </div>
    }
}

fn format_time(milliseconds: f64) -> String {
    let total_seconds = (milliseconds / 1000.0).floor() as u32; // 밀리초를 초로 변환
    let seconds = total_seconds % 60; // 초 계산
    let millis = (milliseconds % 1000.0).round() as u32; // 남은 밀리초 계산

    // 두 자리로 포맷팅: "04:35" 형식
    format!("{:02}:{:02}", seconds, millis % 100)
}
