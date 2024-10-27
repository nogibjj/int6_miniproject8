/// Function to calculate the area of a triangle
/// Returns `None` if the sides do not form a valid triangle
pub fn calculate_triangle_area(a: f64, b: f64, c: f64) -> Option<f64> {
    if is_valid_triangle(a, b, c) {
        let s = (a + b + c) / 2.0;
        Some((s * (s - a) * (s - b) * (s - c)).sqrt())
    } else {
        None
    }
}

/// Function to calculate the perimeter of a triangle
/// Returns `None` if the sides do not form a valid triangle
pub fn calculate_triangle_perimeter(a: f64, b: f64, c: f64) -> Option<f64> {
    if is_valid_triangle(a, b, c) {
        Some(a + b + c)
    } else {
        None
    }
}

/// Helper function to check if three sides can form a valid triangle
fn is_valid_triangle(a: f64, b: f64, c: f64) -> bool {
    a + b > c && a + c > b && b + c > a
}
