use crate::component::drawing_screen::DrawingScreen;
use crate::component::result_screen::ResultScreen;
use crate::component::start_screen::StartScreen;
use crate::func::*;
use crate::lottie::start_snow_animation;
use gloo::timers::callback::{Interval, Timeout};
use wasm_bindgen::JsValue;
use web_sys::{js_sys, window};
use yew::prelude::*;

pub struct TreeDrawingChallenge {
    last_position: Option<(f64, f64)>, // 마지막 위치 저장하여 원을 연결
    pattern: Vec<(f64, f64)>,          // 트리 외곽 라인 패턴을 하나의 연속된 좌표로 저장
    score: Option<u32>,
    timer: Option<Timeout>,
    countdown: Option<Interval>,
    remaining_time: f64,
    svg_ref: NodeRef,
    is_drawing: bool,
    game_state: GameState, // 화면 상태 추가
    is_mobile: Option<bool>,
    result_path: Vec<(f64, f64)>,     // 사용자가 그린 경로
}
pub enum GameState {
    StartScreen,
    DrawingScreen,
    ResultScreen,
    UnsupportedDevice,
}

pub enum Msg {
    StartGame,
    StartDraw,
    StopDraw,
    CalculateScore,
    UpdateTime(f64),
    DetectDevice,
    SetResultPath(Vec<(f64, f64)>),
}

impl TreeDrawingChallenge {
    /// 타이머 시작
    fn start_timer(&mut self, ctx: &Context<Self>, duration: f64) {
        let start_time = Self::get_now();

        self.remaining_time = duration;

        let link = ctx.link().clone();
        self.countdown = Some(Interval::new(100, move || {
            let now = Self::get_now();
            let elapsed = now - start_time;
            let remaining = (duration - elapsed).max(0.0);

            link.send_message(Msg::UpdateTime(remaining));

            if remaining <= 0.0 {
                link.send_message(Msg::StopDraw);
            }
        }));
    }

    /// 브라우저의 현재 시간을 밀리초 단위로 반환
    fn get_now() -> f64 {
        window()
            .expect("window should be available")
            .performance()
            .expect("performance API should be available")
            .now()
    }

    fn stop_timer(&mut self) {
        self.timer = None;
        self.countdown = None;
    }
}

impl Component for TreeDrawingChallenge {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        start_snow_animation(); // 렌더링 후 애니메이션 시작
    }

    fn create(_ctx: &Context<Self>) -> Self {
        let points = vec![
            (130.0, 0.0),   // 트리 꼭대기
            (120.0, 16.0),
            (110.0, 32.0),
            (100.0, 48.0),
            (90.0, 64.0),
            (80.0, 80.0),
            (70.0, 96.0),
            (60.0, 112.0),
            (50.0, 128.0),
            (40.0, 144.0),
            (30.0, 160.0),  // 오른쪽
            (40.0, 160.0),
            (50.0, 160.0),
            (60.0, 160.0),
            (70.0, 160.0),
            (70.0, 160.0),
            (63.0, 173.0),
            (56.0, 186.0),
            (49.0, 199.0),
            (42.0, 212.0),
            (35.0, 225.0),
            (28.0, 238.0),
            (21.0, 251.0),
            (14.0, 264.0),
            (7.0, 277.0),
            (0.0, 290.0),  // 밑바닥
            (25.0, 290.0),
            (50.0, 290.0),
            (75.0, 290.0),
            (100.0, 290.0),
            (125.0, 290.0),
            (150.0, 290.0),
            (175.0, 290.0),
            (200.0, 290.0),
            (225.0, 290.0),
            (250.0, 290.0),
            (243.0, 277.0),
            (236.0, 264.0),
            (229.0, 251.0),
            (222.0, 238.0),
            (215.0, 225.0),
            (208.0, 212.0),
            (201.0, 199.0),
            (194.0, 186.0),
            (187.0, 173.0),
            (180.0, 160.0), // 왼쪽
            (190.0, 160.0),
            (200.0, 160.0),
            (210.0, 160.0),
            (220.0, 160.0), // 트리 꼭대기
            (211.0, 144.0), 
            (202.0, 128.0), 
            (193.0, 112.0), 
            (184.0, 96.0), 
            (175.0, 80.0), 
            (166.0, 64.0), 
            (157.0, 48.0), 
            (148.0, 32.0), 
            (139.0, 16.0), 
            (130.0, 0.0), 
        ];
        let link = _ctx.link().clone();
        link.send_message(Msg::DetectDevice);

        Self {
            last_position: None,
            pattern: points,
            score: None,
            timer: None,
            countdown: None,
            remaining_time: 5000.0,
            svg_ref: NodeRef::default(),
            is_drawing: false,
            game_state: GameState::StartScreen,
            is_mobile: None,
            result_path: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DetectDevice => {
                if let Some(window) = window() {
                    let navigator = window.navigator();
                    let user_agent = navigator.user_agent().unwrap_or_default();
                    web_sys::console::log_1(&format!("User-Agent: {}", user_agent).into());

                    // 터치 디바이스 확인
                    let has_touch_event =
                        js_sys::Reflect::has(&window, &JsValue::from_str("ontouchstart"))
                            .unwrap_or(false);
                    let is_touch_device = has_touch_event || navigator.max_touch_points() > 0;

                    // 플랫폼 확인
                    let platform = navigator.platform().unwrap_or_default();
                    let is_ipad = platform.contains("iPad") // iPad 플랫폼 명시적 감지
                        || (platform == "MacIntel" && is_touch_device); // 터치 가능한 Mac은 iPadOS일 가능성

                    // 모바일 키워드 확인
                    let is_mobile = user_agent.contains("iPhone")
                        || user_agent.contains("Android")
                        || user_agent.contains("Mobile")
                        || is_ipad;

                    self.is_mobile = Some(is_mobile && is_touch_device);

                    // 모바일이 아니면 UnsupportedDevice 상태로 변경
                    if !self.is_mobile.unwrap_or(false) {
                        self.game_state = GameState::UnsupportedDevice;
                    }
                }
                true
            }
            Msg::StartGame => {
                self.result_path.clear();
                self.last_position = None;
                self.score = None;
                self.remaining_time = 5000.0;
                self.is_drawing = true;
                self.game_state = GameState::DrawingScreen;
                self.start_timer(ctx, 5000.0);
                true
            }
            Msg::StartDraw => {
                true
            }
            Msg::StopDraw => {
                self.is_drawing = false;
                self.stop_timer();
                self.game_state = GameState::ResultScreen;
                ctx.link().send_message(Msg::CalculateScore);
                true
            }
            Msg::SetResultPath(path) => {
                self.result_path= path;
                true
            }
            Msg::CalculateScore => {
                self.score = Some(calculate_score(&self.result_path, &self.pattern, 20.0));
                true
            }
            Msg::UpdateTime(remaining) => {
                self.remaining_time = remaining; // 남은 시간 직접 설정

                if self.remaining_time <= 0.0 {
                    ctx.link().send_message(Msg::StopDraw);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                // 항상 표시되는 눈 효과 요소
                <div id="lottie-snow-effect" style="
                position: fixed;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                overflow: hidden; /* 넘치는 부분 잘라내기 */
                pointer-events: none;
                z-index: 9999;
            "></div>

                // 게임 상태에 따른 화면 전환 로직
                {
                    match self.game_state {
                        GameState::StartScreen => {
                            let start_game = ctx.link().callback(|_| Msg::StartGame);
                            html! { <StartScreen on_start={start_game} /> }
                        }
                        GameState::DrawingScreen => {
                            let start_draw = ctx.link().callback(|_| Msg::StartDraw);
                            let stop_draw = ctx.link().callback(|_| Msg::StopDraw);

                            html! {
                                <DrawingScreen
                                    remaining_time={self.remaining_time}
                                    svg_ref={self.svg_ref.clone()}
                                    on_start_draw={start_draw}
                                    on_touch_end={stop_draw.clone()}
                                    result_path={ctx.link().callback(|path: Vec<(f64, f64)>| Msg::SetResultPath(path))}
                                />
                            }
                        }
                        GameState::ResultScreen => {
                            let retry = ctx.link().callback(|_| Msg::StartGame);

                            html! {
                                <ResultScreen
                                    score={self.score.unwrap_or(0)}
                                    result_path={self.result_path.clone()}
                                    on_retry={retry}
                                    remaining_time={self.remaining_time}
                                />
                            }
                        }
                        GameState::UnsupportedDevice => {
                            html! {
                                <div class="unsupported-device">
                                    <div>
                                        <p class="alert-title">{ "알림" }</p>
                                        <p>{ "이 챌린지는 터치 기반 모바일 디바이스에서만 실행 가능합니다!" }</p>
                                        <p>{ "모바일 디바이스로 접속해주세요 :)" }</p>
                                    </div>
                                </div>
                            }
                        }
                    }
                }
            </>
        }
    }
}
