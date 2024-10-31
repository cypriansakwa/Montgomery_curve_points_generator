const A: i64 = 6;
const B: i64 = 7;
const P: i64 = 13; // Prime for field F_p

/// Checks if a given (x, y) point satisfies the Montgomery curve equation.
fn is_point_on_curve(x: i64, y: i64) -> bool {
    let lhs = (B * y * y) % P;
    let rhs = (x * x * x + A * x * x + x) % P;
    lhs == rhs
}

/// Generates all points on the Montgomery curve over F_p.
fn generate_montgomery_curve_points() -> Vec<(i64, i64)> {
    let mut points = vec![];

    for x in 0..P {
        for y in 0..P {
            if is_point_on_curve(x, y) {
                points.push((x, y));
            }
        }
    }

    points
}

fn main() {
    let points = generate_montgomery_curve_points();
    println!("Points on the Montgomery curve:");
    for point in points {
        println!("{:?}", point);
    }
}
