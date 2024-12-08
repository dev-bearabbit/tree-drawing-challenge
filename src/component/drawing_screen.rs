use crate::func::format_time;
use web_sys::{TouchEvent, SvgElement};
use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct DrawingScreenProps {
    pub remaining_time: f64,
    pub svg_ref: NodeRef,
    pub result_path: Callback<Vec<(f64, f64)>>,
    pub on_start_draw: Callback<TouchEvent>,
    pub on_touch_end: Callback<()>,
}

#[function_component(DrawingScreen)]
pub fn drawing_screen(props: &DrawingScreenProps) -> Html {
    // Path 데이터를 상태 대신 참조로 관리하여 DOM 업데이트 최소화
    let current_path = use_mut_ref(|| Vec::new());

    // 터치 이벤트 처리 (스로틀링 적용)
    let on_update_draw = {
        let current_path = current_path.clone();
        let svg_ref = props.svg_ref.clone();
    
        Callback::from(move |event: TouchEvent| {
            if let Some(touch) = event.touches().get(0) {
                if let Some(svg) = svg_ref.cast::<SvgElement>() {
                    let bounding_box = svg.get_bounding_client_rect();
                    let x = touch.client_x() as f64 - bounding_box.x();
                    let y = touch.client_y() as f64 - bounding_box.y();
                    current_path.borrow_mut().push((x, y));
    
                    // 렌더링을 브라우저 애니메이션 프레임과 동기화
                    let path_points = current_path
                        .borrow()
                        .iter()
                        .map(|(x, y)| format!("{},{}", x, y))
                        .collect::<Vec<_>>()
                        .join(" ");
    
                    let closure = Closure::wrap(Box::new(move || {
                        if let Some(polyline) = svg.query_selector("polyline").ok().flatten() {
                            polyline
                                .set_attribute("points", &path_points)
                                .expect("Failed to set points attribute");
                        }
                    }) as Box<dyn FnMut()>);
    
                    web_sys::window()
                        .unwrap()
                        .request_animation_frame(closure.as_ref().unchecked_ref())
                        .expect("requestAnimationFrame failed");
                    closure.forget();
                }
            }
        })
    };

    // 터치 종료 이벤트 처리
    let on_touch_end = {
        let current_path = current_path.clone();
        let result_path = props.result_path.clone();
        let on_touch_end = props.on_touch_end.clone();

        Callback::from(move |_| {
            result_path.emit(current_path.borrow().clone()); // 최종 경로를 부모에 전달
            on_touch_end.emit(())
        })
    };

    html! {
        <div class="screen">
            <div class="sentence">
                <h3>
                    { "트리 모양 선을 따라" }
                    <br />
                    { "빠르게 그려주세요" }
                </h3>
            </div>
            <div class="tree-container">

                <svg class="star" viewBox="0 0 52 49" preserveAspectRatio="xMidYMin" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M24.3204 1.598C25.1084 0.379102 26.8916 0.379101 27.6796 1.598L35.1592 13.1676C35.429 13.585 35.8447 13.887 36.3251 14.0146L49.6397 17.553C51.0425 17.9258 51.5935 19.6216 50.6778 20.7477L41.9858 31.4364C41.6722 31.8221 41.5134 32.3107 41.5404 32.807L42.2897 46.5634C42.3687 48.0127 40.9261 49.0608 39.5721 48.5379L26.7206 43.5743C26.2569 43.3952 25.7431 43.3952 25.2794 43.5743L12.4279 48.5379C11.0739 49.0608 9.63133 48.0127 9.71027 46.5634L10.4596 32.807C10.4866 32.3107 10.3278 31.8221 10.0142 31.4364L1.32224 20.7477C0.40651 19.6216 0.957531 17.9258 2.36028 17.553L15.6749 14.0146C16.1553 13.887 16.571 13.585 16.8408 13.1676L24.3204 1.598Z" fill="#FFFFFF29"/>
                </svg>

                <svg class="tree-pattern"
                     viewBox="0 0 256 291"
                     preserveAspectRatio="xMidYMin"
                     fill="none"
                     xmlns="http://www.w3.org/2000/svg"
                     ref={props.svg_ref.clone()}
                     ontouchstart={props.on_start_draw.clone()}
                     ontouchmove={on_update_draw}
                     ontouchend={on_touch_end}
                     >
                    <path d="M121.15 8.50157L117.724 6.43576V6.43576L121.15 8.50157ZM134.851 8.50158L131.425 10.5674V10.5674L134.851 8.50158ZM35.8845 149.877L39.3097 151.942L35.8845 149.877ZM68.6952 162.008L72.1781 163.975C72.8776 162.737 72.8668 161.22 72.1498 159.992C71.4329 158.763 70.1175 158.008 68.6952 158.008V162.008ZM5.04591 274.704L1.56301 272.737H1.56301L5.04591 274.704ZM250.954 274.704L254.437 272.737L250.954 274.704ZM187.305 162.008V158.008C185.882 158.008 184.567 158.763 183.85 159.992C183.133 161.22 183.122 162.737 183.822 163.975L187.305 162.008ZM220.116 149.877L216.69 151.942V151.942L220.116 149.877ZM124.575 10.5674C126.13 7.98848 129.87 7.98849 131.425 10.5674L138.276 6.43576C133.61 -1.30098 122.39 -1.30097 117.724 6.43576L124.575 10.5674ZM39.3097 151.942L124.575 10.5674L117.724 6.43576L32.4592 147.811L39.3097 151.942ZM42.735 158.008C39.6217 158.008 37.7019 154.608 39.3097 151.942L32.4592 147.811C27.6356 155.809 33.3952 166.008 42.735 166.008V158.008ZM68.6952 158.008H42.735V166.008H68.6952V158.008ZM8.5288 276.671L72.1781 163.975L65.2123 160.041L1.56301 272.737L8.5288 276.671ZM12.0117 282.638C8.94932 282.638 7.0228 279.338 8.5288 276.671L1.56301 272.737C-2.95499 280.737 2.82455 290.638 12.0117 290.638V282.638ZM243.988 282.638H12.0117V290.638H243.988V282.638ZM247.471 276.671C248.977 279.338 247.051 282.638 243.988 282.638V290.638C253.175 290.638 258.955 280.737 254.437 272.737L247.471 276.671ZM183.822 163.975L247.471 276.671L254.437 272.737L190.788 160.041L183.822 163.975ZM213.265 158.008H187.305V166.008H213.265V158.008ZM216.69 151.942C218.298 154.608 216.378 158.008 213.265 158.008V166.008C222.605 166.008 228.364 155.809 223.541 147.811L216.69 151.942ZM131.425 10.5674L216.69 151.942L223.541 147.811L138.276 6.43576L131.425 10.5674Z" fill="white" fill-opacity="0.4"/>

                    <polyline
                    points=""
                    stroke="#72F48F"
                    stroke-width="8"
                    fill="none"
                    />

                </svg>

                <div class="timer">
                    { format_time(props.remaining_time) }
                </div>
            </div>
        </div>
    }
}
