use crate::coordinates::ScreenPoint;

pub struct LineRenderer;

impl LineRenderer {
    /// Generate all points along a line between two screen points
    /// using Bresenham's line algorithm
    pub fn bresenham_line(start: ScreenPoint, end: ScreenPoint) -> Vec<ScreenPoint> {
        let mut points = Vec::new();
        
        let dx = (end.col as i32 - start.col as i32).abs();
        let dy = (end.row as i32 - start.row as i32).abs();
        
        let sx = if start.col < end.col { 1 } else { -1 };
        let sy = if start.row < end.row { 1 } else { -1 };
        
        let mut err = dx - dy;
        let mut x = start.col as i32;
        let mut y = start.row as i32;
        
        loop {
            points.push(ScreenPoint {
                col: x as usize,
                row: y as usize,
            });
            
            if x == end.col as i32 && y == end.row as i32 {
                break;
            }
            
            let e2 = 2 * err;
            
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        
        points
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bresenham_horizontal_line() {
        let start = ScreenPoint { col: 1, row: 5 };
        let end = ScreenPoint { col: 5, row: 5 };
        let points = LineRenderer::bresenham_line(start, end);
        
        assert_eq!(points.len(), 5);
        assert_eq!(points[0], start);
        assert_eq!(points[4], end);
        
        // All points should have the same row
        for point in &points {
            assert_eq!(point.row, 5);
        }
    }

    #[test]
    fn test_bresenham_vertical_line() {
        let start = ScreenPoint { col: 3, row: 1 };
        let end = ScreenPoint { col: 3, row: 4 };
        let points = LineRenderer::bresenham_line(start, end);
        
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], start);
        assert_eq!(points[3], end);
        
        // All points should have the same column
        for point in &points {
            assert_eq!(point.col, 3);
        }
    }

    #[test]
    fn test_bresenham_diagonal_line() {
        let start = ScreenPoint { col: 0, row: 0 };
        let end = ScreenPoint { col: 3, row: 3 };
        let points = LineRenderer::bresenham_line(start, end);
        
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], start);
        assert_eq!(points[3], end);
    }

}