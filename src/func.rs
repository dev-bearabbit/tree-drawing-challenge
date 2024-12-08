
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
