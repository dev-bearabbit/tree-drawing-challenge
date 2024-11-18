use web_sys::{TouchEvent, SvgsvgElement};
use yew::NodeRef;


/// 터치 이벤트로부터 SVG 내부 좌표 계산
pub fn get_touch_position(event: &TouchEvent, svg_ref: &NodeRef) -> Option<(f64, f64)> {
    if let Some(touch) = event.touches().item(0) {
        let client_x = touch.client_x() as f64;
        let client_y = touch.client_y() as f64;
        let svg = svg_ref.cast::<SvgsvgElement>()?;
        let rect = svg.get_bounding_client_rect();
        let svg_x = rect.left() as f64;
        let svg_y = rect.top() as f64;
        let x = client_x - svg_x;
        let y = client_y - svg_y;
        Some((x, y))
    } else {
        None
    }
}

pub fn calculate_score(user_path: &[(f64, f64)], pattern: &[(f64, f64)]) -> u32 {
    let mut total_distance = 0.0;

    for (user_point, pattern_point) in user_path.iter().zip(pattern.iter()) {
        let dx = user_point.0 - pattern_point.0;
        let dy = user_point.1 - pattern_point.1;
        total_distance += (dx.powi(2) + dy.powi(2)).sqrt();
    }

    // 점수는 거리가 적을수록 높음
    let average_distance = total_distance / user_path.len().max(1) as f64;
    // 점수 계산 조정 (거리가 클수록 점수가 더 빠르게 줄어들도록 설정)
    let normalized_score = 100.0 / (1.0 + average_distance / 5.0); // 거리 스케일 조정
    let score = normalized_score.max(0.0).min(100.0).round() as u32; // 0 ~ 100 사이로 제한
    score
}

pub fn format_time(milliseconds: f64) -> String {
    let total_seconds = (milliseconds / 1000.0).floor() as u32; // 밀리초를 초로 변환
    let seconds = total_seconds % 60; // 초 계산
    let millis = (milliseconds % 1000.0).round() as u32; // 남은 밀리초 계산

    // 두 자리로 포맷팅: "04 : 35" 형식
    format!("{:02} : {:02}", seconds, millis % 100)
}