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

pub fn calculate_score(circles: &[(f64, f64)], pattern: &[(f64, f64)]) -> u32 {
    let max_score = 100.0;
    let tolerance = 10.0; // 허용 오차 범위

    let mut score = 0.0;

    for (px, py) in pattern {
        let mut min_distance = f64::MAX;

        for (cx, cy) in circles {
            let distance = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
            if distance < min_distance {
                min_distance = distance;
            }
        }

        if min_distance <= tolerance {
            score += max_score / pattern.len() as f64;
        } else {
            score += (max_score / pattern.len() as f64) * (1.0 - (min_distance / tolerance).min(1.0));
        }
    }

    score.round() as u32
}

