use yew::prelude::*;
use web_sys::TouchEvent;
use gloo::timers::callback::{Timeout, Interval};
use crate::func::{get_touch_position, calculate_score};
use crate::component::start_screen::StartScreen;
use crate::component::drawing_screen::DrawingScreen;
use crate::component::result_screen::ResultScreen;
use crate::lottie::start_snow_animation;

pub struct TreeDrawingChallenge {
    circles: Vec<(f64, f64)>, // 사용자가 그린 경로를 원으로 연결
    last_position: Option<(f64, f64)>, // 마지막 위치 저장하여 원을 연결
    pattern: Vec<(f64, f64)>, // 트리 외곽 라인 패턴을 하나의 연속된 좌표로 저장
    score: Option<u32>,
    timer: Option<Timeout>,
    countdown: Option<Interval>,
    draw_interval: Option<Interval>,
    remaining_time: f64,
    svg_ref: NodeRef,
    is_drawing: bool,
    game_state: GameState, // 화면 상태 추가
}
pub enum GameState {
    StartScreen,
    DrawingScreen,
    ResultScreen,
}

pub enum Msg {
    StartGame,
    StartDraw(TouchEvent),
    UpdateDrawPosition(TouchEvent),
    EndDraw,
    StopDraw,
    CalculateScore,
    UpdateTime
}

impl Component for TreeDrawingChallenge {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        start_snow_animation(); // 렌더링 후 애니메이션 시작
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            circles: vec![],
            last_position: None,
            pattern: vec![
                (100.0, 60.0),  // 트리 꼭대기
                (40.0, 160.0),   // 오른쪽
                (60.0, 160.0),  // 오른쪽
                (20.0, 240.0),  // 밑바닥
                (180.0, 240.0), // 밑바닥
                (140.0, 160.0),  // 왼쪽
                (160.0, 160.0),  // 왼쪽
                (100.0, 60.0),  // 트리 꼭대기
            ],
            score: None,
            timer: None,
            countdown: None,
            draw_interval: None,
            remaining_time: 5000.0,
            svg_ref: NodeRef::default(),
            is_drawing: false,
            game_state: GameState::StartScreen
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartGame => {
                self.circles.clear();
                self.last_position = None;
                self.score = None;
                self.remaining_time = 5000.0;
                self.is_drawing = true;
                self.game_state = GameState::DrawingScreen;

                let link = ctx.link().clone();
                self.timer = Some(Timeout::new(5000, move || link.send_message(Msg::StopDraw)));

                let link = ctx.link().clone();
                self.countdown = Some(Interval::new(50, move || link.send_message(Msg::UpdateTime)));
                
                true
            }
            Msg::StartDraw(event) => {
                if self.is_drawing {
                    if let Some((x, y)) = get_touch_position(&event, &self.svg_ref) {
                        self.last_position = Some((x, y));
                        self.circles.push((x, y));

                        let link = ctx.link().clone();
                        self.draw_interval = Some(Interval::new(10, move || {
                            link.send_message(Msg::UpdateDrawPosition(event.clone()))
                        }));
                    }
                }
                true
            }
            Msg::UpdateDrawPosition(event) => {
                if self.is_drawing {
                    if let Some((x, y)) = get_touch_position(&event, &self.svg_ref) {
                        if let Some(last_pos) = self.last_position {
                            if ((x - last_pos.0).powi(2) + (y - last_pos.1).powi(2)).sqrt() > 2.0 {
                                self.circles.push((x, y));
                                self.last_position = Some((x, y));
                            }
                        }
                    }
                }
                true
            }
            Msg::EndDraw => {
                self.draw_interval = None;
                self.last_position = None;
                true
            }
            Msg::StopDraw => {
                self.remaining_time = 0.0;
                self.is_drawing = false; // 그리기 비활성화
                self.timer = None;
                self.countdown = None;
                self.draw_interval = None;
                self.game_state = GameState::ResultScreen;
                ctx.link().send_message(Msg::CalculateScore);
                true
            }
            Msg::CalculateScore => {
                let user_points: Vec<_> = self.circles.clone();
                self.score = Some(calculate_score(&user_points, &self.pattern));
                true
            }
            Msg::UpdateTime => {
                if self.remaining_time > 0.0 {
                    self.remaining_time -= 50.0;
                    if self.remaining_time < 0.0 {
                        self.remaining_time = 0.0;
                    }
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
                            html! { <StartScreen on_start={start_game} pattern={self.pattern.clone()} /> }
                        }
                        GameState::DrawingScreen => {
                            let start_draw = ctx.link().callback(move |event: TouchEvent| Msg::StartDraw(event));
                            let update_draw_position = ctx.link().callback(move |event: TouchEvent| Msg::UpdateDrawPosition(event));
                            let end_draw = ctx.link().callback(|_| Msg::EndDraw);

                            html! {
                                <DrawingScreen 
                                    remaining_time={self.remaining_time} 
                                    svg_ref={self.svg_ref.clone()}
                                    pattern={self.pattern.clone()}
                                    circles={self.circles.clone()}
                                    on_start_draw={start_draw}
                                    on_update_draw={update_draw_position}
                                    on_end_draw={end_draw}
                                />
                            }
                        }
                        GameState::ResultScreen => {
                            let retry = ctx.link().callback(|_| Msg::StartGame);

                            html! {
                                <ResultScreen 
                                    score={self.score.unwrap_or(0)} 
                                    pattern={self.pattern.clone()} 
                                    circles={self.circles.clone()} 
                                    on_retry={retry} 
                                />
                            }
                        }
                    }
                }
            </>
        }
    }
}
