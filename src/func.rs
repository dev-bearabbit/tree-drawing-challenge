use web_sys::{SvgsvgElement, TouchEvent};
use yew::NodeRef;

/// 터치 이벤트로부터 SVG 내부 좌표 계산
pub fn get_touch_position(event: &TouchEvent, svg_ref: &NodeRef) -> Option<(f64, f64)> {
    if let Some(touch) = event.touches().item(0) {
        let client_x = touch.client_x() as f64;
        let client_y = touch.client_y() as f64;

        if let Some(svg) = svg_ref.cast::<SvgsvgElement>() {
            let rect = svg.get_bounding_client_rect();

            // SVG의 크기와 ViewBox 크기 가져오기
            let svg_width = rect.width();
            let svg_height = rect.height();

            let view_box = svg
                .get_attribute("viewBox")
                .unwrap_or_default()
                .split_whitespace()
                .map(|v| v.parse::<f64>().unwrap_or(0.0))
                .collect::<Vec<_>>();

            if view_box.len() == 4 {
                let view_box_x = view_box[0];
                let view_box_y = view_box[1];
                let view_box_width = view_box[2];
                let view_box_height = view_box[3];

                // 터치 좌표를 SVG 내부 좌표로 변환
                let x = ((client_x - rect.left()) / svg_width) * view_box_width + view_box_x;
                let y =
                    ((client_y - rect.top()) / svg_height) * view_box_height + view_box_y;

                Some((x, y))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// 주요 패턴 점 통과 여부 기반 점수 계산
pub fn calculate_score(user_path: &[(f64, f64)], pattern: &[(f64, f64)], threshold: f64) -> u32 {
    let mut passed_points = 0;

    for pattern_point in pattern {
        if user_path.iter().any(|user_point| {
            let dx = user_point.0 - pattern_point.0;
            let dy = user_point.1 - pattern_point.1;
            (dx.powi(2) + dy.powi(2)).sqrt() <= threshold
        }) {
            passed_points += 1;
        }
    }

    // 점수 계산
    let percentage = passed_points as f64 / pattern.len() as f64;
    (percentage * 100.0).round() as u32 // 0 ~ 100 사이 점수
}

pub fn format_time(milliseconds: f64) -> String {
    let total_seconds = (milliseconds / 1000.0).floor() as u32; // 밀리초를 초로 변환
    let seconds = total_seconds % 60; // 초 계산
    let millis = ((milliseconds % 1000.0) / 10.0).round() as u32; // 밀리초를 두 자리로 변환

    // 두 자리로 포맷팅: "04 : 35" 형식
    format!("{:02} : {:02}", seconds, millis)
}
