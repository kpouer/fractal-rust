use crate::point::Point;

enum Interaction {
    MouseLeftClick(Point),
    MouseRightClick(Point),
    MouseWheel { delta: f64 },
}
