use crate::canvas::Canvas;
use crate::data::Dataset;

pub fn render_scatter_plot(dataset: &Dataset, title: &str, symbol: char) -> String {
    let mut canvas = Canvas::new(dataset, title);
    
    // Plot all points
    for point in &dataset.points {
        canvas.plot_point(point, symbol);
    }
    
    canvas.render(title, &dataset.x_label, &dataset.y_label)
}