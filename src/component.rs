use yew::prelude::*;
use web_sys::TouchEvent;
use gloo::timers::callback::{Timeout, Interval};
use crate::logic::{get_touch_position, calculate_score};

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
}

pub enum Msg {
    StartGame,
    StartDraw(TouchEvent),
    UpdateDrawPosition(TouchEvent),
    EndDraw,
    StopDraw,
    CalculateScore,
    UpdateTime,
}

impl Component for TreeDrawingChallenge {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            circles: vec![],
            last_position: None,
            pattern: vec![
                (100.0, 20.0),  // 트리 꼭대기
                (60.0, 80.0),   // 상단 왼쪽
                (80.0, 80.0),  // 상단 오른쪽
                (40.0, 140.0),  // 중간 왼쪽
                (60.0, 140.0), // 중간 
                (20.0, 200.0),  // 하단 왼쪽
                (180.0, 200.0), // 하단 밑선
                (140.0, 140.0),  // 중간 오른쪽
                (160.0, 140.0),  // 중간 오른쪽
                (120.0, 80.0),  // 상단 오른쪽
                (140.0, 80.0),  // 상단 오른쪽
                (100.0, 20.0),  // 트리 꼭대기
            ],
            score: None,
            timer: None,
            countdown: None,
            draw_interval: None,
            remaining_time: 5000.0,
            svg_ref: NodeRef::default(),
            is_drawing: false,
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
        let svg_ref_clone = self.svg_ref.clone();
        let start_game = ctx.link().callback(|_| Msg::StartGame);
        let start_draw = ctx.link().callback(move |event: TouchEvent| Msg::StartDraw(event));
        let update_draw_position = ctx.link().callback(move |event: TouchEvent| Msg::UpdateDrawPosition(event));
        let end_draw = ctx.link().callback(|_| Msg::EndDraw);

        html! {
            <div class="game-container">
                <div class="header">
                    <h1>{ "Tree Drawing Challenge" }</h1>
                    <button onclick={start_game} disabled={self.is_drawing} class="start-button">{ "Start" }</button>
                </div>
                <svg
                    ref={svg_ref_clone}
                    width="100%"
                    height="300"
                    ontouchstart={start_draw}
                    ontouchmove={update_draw_position}
                    ontouchend={end_draw}
                    class="drawing-area"
                >
                    <polyline
                    points={self.pattern.iter().map(|(x, y)| format!("{},{}", x, y)).collect::<Vec<_>>().join(" ")}
                    stroke="green"
                    stroke-width="5"
                    fill="none"
                />
                    
                    { for self.circles.iter().map(|(x, y)| html! {
                        <circle cx={format!("{}", x)} cy={format!("{}", y)} r="5" fill="blue" />
                    })}
                </svg>
                <div class="info-panel">
                    <p>{ format!("Time remaining: {:.2} seconds", self.remaining_time / 1000.0) }</p>
                    { if let Some(score) = self.score {
                        html! { <p class="score">{ format!("Your Score: {}", score) }</p> }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        }
    }
}
