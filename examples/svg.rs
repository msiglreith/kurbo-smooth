use piet::RenderContext;
use kurbo_smooth::Squircle;

fn draw_squircle<P: piet::RenderContext>(rt: &mut P, brush: &P::Brush, squircle: Squircle) {
    let points = squircle.to_curve();
    let mut path = Vec::new();
    let mut i = 0;
    while i + 4 <= points.len() {
        path.push(kurbo::PathSeg::Cubic(kurbo::CubicBez {
            p0: points[i],
            p1: points[i + 1],
            p2: points[i + 2],
            p3: points[i + 3],
        }));
        i += 3;
    }
    rt.stroke(
        kurbo::BezPath::from_path_segments(path.into_iter()),
        brush,
        2.0,
    );
}

fn main() -> anyhow::Result<()> {
    let mut rt = piet_svg::RenderContext::new();
    let brush = rt.solid_brush(piet::Color::rgba(0.8, 0.2, 0.1, 0.7));
    draw_squircle(
        &mut rt,
        &brush,
        Squircle {
            p0: kurbo::Point { x: 0.0, y: 100.0 },
            p1: kurbo::Point { x: 100.0, y: 100.0 },
            p2: kurbo::Point { x: 100.0, y: 0.0 },
            radius: 50.0,
            smoothness: 0.6,
        },
    );
    draw_squircle(
        &mut rt,
        &brush,
        Squircle {
            p0: kurbo::Point { x: 90.0, y: 50.0 },
            p1: kurbo::Point { x: 50.0, y: 10.0 },
            p2: kurbo::Point { x: 100.0, y: 10.0 },
            radius: 20.0,
            smoothness: 0.6,
        },
    );
    draw_squircle(
        &mut rt,
        &brush,
        Squircle {
            p0: kurbo::Point { x: 160.0, y: 50.0 },
            p1: kurbo::Point { x: 120.0, y: 10.0 },
            p2: kurbo::Point { x: 200.0, y: 10.0 },
            radius: 20.0,
            smoothness: 0.9,
        },
    );
    draw_squircle(
        &mut rt,
        &brush,
        Squircle {
            p0: kurbo::Point { x: 90.0, y: 150.0 },
            p1: kurbo::Point { x: 150.0, y: 100.0 },
            p2: kurbo::Point { x: 100.0, y: 100.0 },
            radius: 16.0,
            smoothness: 0.5,
        },
    );

    let mut out = std::fs::File::create("squircle.svg")?;
    rt.write(&mut out)?;
    Ok(())
}
