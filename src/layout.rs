use ratatui::layout::Rect;

/// Splits an area into two responsive columns.
///
/// The returned rectangles sit side by side with a one-cell gap when both can
/// meet `minimum_width`. On narrower terminals they stack vertically. This
/// helper performs no drawing and works with any ratatui widget.
#[must_use]
pub fn responsive_columns(area: Rect, minimum_width: u16) -> [Rect; 2] {
    if area.width >= minimum_width.saturating_mul(2).saturating_add(1) {
        let left_width = area.width.saturating_sub(1) / 2;
        let right_x = area.x.saturating_add(left_width).saturating_add(1);

        [
            Rect::new(area.x, area.y, left_width, area.height),
            Rect::new(
                right_x,
                area.y,
                area.width.saturating_sub(left_width).saturating_sub(1),
                area.height,
            ),
        ]
    } else {
        let top_height = area.height / 2;

        [
            Rect::new(area.x, area.y, area.width, top_height),
            Rect::new(
                area.x,
                area.y.saturating_add(top_height),
                area.width,
                area.height.saturating_sub(top_height),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_areas_split_horizontally_with_a_gap() {
        let [left, right] = responsive_columns(Rect::new(2, 3, 81, 20), 40);

        assert_eq!(left, Rect::new(2, 3, 40, 20));
        assert_eq!(right, Rect::new(43, 3, 40, 20));
    }

    #[test]
    fn narrow_areas_stack_without_losing_rows() {
        let [top, bottom] = responsive_columns(Rect::new(2, 3, 79, 21), 40);

        assert_eq!(top, Rect::new(2, 3, 79, 10));
        assert_eq!(bottom, Rect::new(2, 13, 79, 11));
    }
}
