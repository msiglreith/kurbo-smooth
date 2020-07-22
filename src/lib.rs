#[derive(Debug, Copy, Clone)]
pub struct Squircle {
    pub p0: kurbo::Point,
    pub p1: kurbo::Point,
    pub p2: kurbo::Point,
    pub radius: f64,
    pub smoothness: f64,
}

impl Squircle {
    pub fn to_curve(&self) -> Vec<kurbo::Point> {
        let Squircle {
            p0,
            p1,
            p2,
            radius,
            smoothness: xi,
        } = *self;

        if radius <= 0.0 {
            return vec![p1];
        }

        let v0 = p0 - p1;
        let v1 = p2 - p1;

        let length0 = v0.hypot();
        let length1 = v1.hypot();

        let n0 = v0 / length0;
        let n1 = v1 / length1;

        let alpha = n0.dot(n1).acos();

        let t = 1.0 / (alpha / 2.0).tan();

        // clamp radius and amount of smoothing
        let smoothing_length = (t + xi) * radius;
        let smoothing = smoothing_length.min(length0).min(length1);
        let (radius, xi) = if smoothing < t * radius {
            (smoothing / t, 0.0)
        } else {
            (radius, (smoothing / radius - t).max(0.0))
        };

        let pc = p1 + n0 * radius * t;
        let ccw = v0.cross(v1) > 0.0;
        let n = if ccw {
            kurbo::Vec2 { x: -n0.y, y: n0.x }
        } else {
            kurbo::Vec2 { x: n0.y, y: -n0.x }
        };

        let center = pc + n * radius;

        let phi = std::f64::consts::PI - alpha;
        let phi0 = 0.5 * phi * xi;
        let phi1 = phi - 2.0 * phi0;

        let phi0 = if ccw { -phi0 } else { phi0 };
        let phi1 = if ccw { -phi1 } else { phi1 };
        let (s0, c0) = phi0.sin_cos();
        let (s1, c1) = (phi0 + phi1).sin_cos();
        let vc = (0.5 * phi0.abs()).tan();
        let ab = (vc + xi) * radius;

        let sp0 = kurbo::Vec2 {
            x: (c0 * -n.x + s0 * n.y),
            y: (s0 * -n.x - c0 * n.y),
        };
        let sp1 = kurbo::Vec2 {
            x: (c1 * -n.x + s1 * n.y),
            y: (s1 * -n.x - c1 * n.y),
        };

        let c00 = p1 + n0 * smoothing;
        let c01 = p1 + n0 * (smoothing - 2.0 * ab / 3.0);
        let c02 = p1 + n0 * ((t - vc) * radius);
        let c03 = center + radius * sp0;

        let c10 = p1 + n1 * smoothing;
        let c11 = p1 + n1 * (smoothing - 2.0 * ab / 3.0);
        let c12 = p1 + n1 * ((t - vc) * radius);
        let c13 = center + radius * sp1;

        let phi2 = (phi1) / 2.0;
        let (s2, c2) = (phi0 + phi2).sin_cos();
        let sp2 = kurbo::Vec2 {
            x: (c2 * -n.x + s2 * n.y),
            y: (s2 * -n.x - c2 * n.y),
        };

        let alp = 4.0 / 3.0 * (phi2 / 4.0).tan();
        let x02 = center + radius * sp2;
        let x00 = c03
            + radius
                * kurbo::Vec2 {
                    x: -alp * sp0.y,
                    y: alp * sp0.x,
                };
        let x01 = x02
            + radius
                * kurbo::Vec2 {
                    x: alp * sp2.y,
                    y: -alp * sp2.x,
                };
        let x03 = x02
            + radius
                * kurbo::Vec2 {
                    x: -alp * sp2.y,
                    y: alp * sp2.x,
                };
        let x04 = c13
            + radius
                * kurbo::Vec2 {
                    x: alp * sp1.y,
                    y: -alp * sp1.x,
                };

        vec![
            c00, c01, c02, c03, x00, x01, x02, x03, x04, c13, c12, c11, c10,
        ]
    }
}
