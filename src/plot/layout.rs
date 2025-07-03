/// Generic layout calculator for any regularly-spaced elements
#[derive(Debug, Clone)]
pub struct ElementLayout {
    /// Width of each element (bars, ticks, points, etc.)
    pub element_width: usize,
    /// Spacing between elements
    pub spacing: usize,
    /// Offset from left edge
    pub offset: usize,
}

impl ElementLayout {
    /// Calculate layout for bar charts (migrated from BarLayout logic)
    pub fn for_bars(chart_width: usize, num_bars: usize) -> Self {
        const MIN_BAR_WIDTH: usize = 1;
        const MIN_SPACING: usize = 0;
        const PREFERRED_BAR_WIDTH: usize = 2;
        const PREFERRED_SPACING: usize = 1;
        
        if num_bars == 0 {
            return ElementLayout {
                element_width: PREFERRED_BAR_WIDTH,
                spacing: PREFERRED_SPACING,
                offset: 1,
            };
        }
        
        // Start with preferred layout
        let mut layout = ElementLayout {
            element_width: PREFERRED_BAR_WIDTH,
            spacing: PREFERRED_SPACING,
            offset: 1, // Initial offset for alignment
        };
        
        // Check if preferred layout fits
        let required = layout.total_width(num_bars);
        
        if required > chart_width {
            // Try with minimum spacing first
            layout.spacing = MIN_SPACING;
            let required = layout.total_width(num_bars);
            
            if required > chart_width {
                // Fall back to minimum bar width
                layout.element_width = MIN_BAR_WIDTH;
                
                // Calculate remaining space for gaps
                let bars_width = num_bars * MIN_BAR_WIDTH;
                if bars_width + layout.offset < chart_width {
                    let available_for_gaps = chart_width - bars_width - layout.offset;
                    let num_gaps = num_bars.saturating_sub(1).max(1);
                    layout.spacing = available_for_gaps / num_gaps;
                }
            }
        } else {
            // We have extra space, but keep spacing modest
            let extra_space = chart_width - required;
            let num_gaps = num_bars.saturating_sub(1).max(1);
            
            // Only add minimal extra spacing, don't go overboard
            let max_extra_spacing = 1; // Limit extra spacing to 1 additional character
            let extra_spacing = (extra_space / num_gaps).min(max_extra_spacing);
            layout.spacing += extra_spacing;
            
            // Center the chart with remaining space, but don't go overboard
            let remaining_space = extra_space - (extra_spacing * num_gaps);
            let max_additional_offset = 3; // Limit how much we center
            let additional_offset = (remaining_space / 2).min(max_additional_offset);
            layout.offset += additional_offset;
        }
        
        layout
    }
    
    
    /// Calculate layout for axis ticks - matches Canvas tick positioning
    pub fn for_ticks(_chart_width: usize, _num_ticks: usize) -> Self {
        // For Canvas-based plots, match Canvas's tick algorithm: x_pos = (i * (width - 1)) / (num_ticks + 1)
        // This is different from evenly spaced ticks - it uses Canvas's internal spacing
        ElementLayout {
            element_width: 1,
            spacing: 0, // Spacing handled by custom positioning
            offset: 0,  // Will be calculated per-tick to match Canvas
        }
    }
    
    /// Get tick position using Canvas algorithm (for Canvas-based plots)
    pub fn canvas_tick_position(&self, tick_index: usize, chart_width: usize, num_ticks: usize) -> usize {
        // Match Canvas's data positioning which reserves column 0 for Y-axis
        // Data maps to columns 1 through width-1 using (width-2) as data width
        // So ticks should be positioned in the same data space
        let i = tick_index;
        let data_width = chart_width.saturating_sub(1); // Available data width (excluding Y-axis column)
        
        if num_ticks <= 1 {
            return 1 + data_width / 2; // Center position
        }
        
        // Position ticks evenly across the data width, starting from column 1
        1 + (i * (data_width - 1)) / (num_ticks - 1)
    }
    
    /// Calculate total width needed for this layout
    pub fn total_width(&self, num_elements: usize) -> usize {
        if num_elements == 0 {
            return 0;
        }
        self.offset + (num_elements * self.element_width) + 
        (num_elements.saturating_sub(1) * self.spacing)
    }
    
    /// Get x-position for a specific element
    pub fn element_position(&self, index: usize) -> usize {
        self.offset + (index * (self.element_width + self.spacing))
    }
    
    /// Maximum elements that can fit with this layout
    pub fn max_elements_for_width(&self, width: usize) -> usize {
        if self.element_width + self.spacing == 0 {
            return 0;
        }
        
        let available = width.saturating_sub(self.offset);
        // Account for the fact that the last element doesn't need spacing after it
        (available + self.spacing) / (self.element_width + self.spacing)
    }
}

/// Different bar representations based on available width
#[derive(Debug, Clone, Copy)]
pub enum BarStyle {
    Wide,     // ██ (2 chars)
    Normal,   // █ (1 char)  
    #[allow(dead_code)]
    Thin,     // ▌ (half block) - reserved for future use
    #[allow(dead_code)]
    Minimal,  // | (pipe) - reserved for future use
}

impl BarStyle {
    pub fn from_width(width: usize) -> Self {
        match width {
            0 => BarStyle::Minimal,
            1 => BarStyle::Normal,
            2 => BarStyle::Wide,
            _ => BarStyle::Wide,
        }
    }
    
    pub fn get_symbol(&self) -> char {
        match self {
            BarStyle::Wide => '█',
            BarStyle::Normal => '█',
            BarStyle::Thin => '▌',
            BarStyle::Minimal => '|',
        }
    }
    
    #[allow(dead_code)]
    pub fn char_count(&self) -> usize {
        match self {
            BarStyle::Wide => 2,
            BarStyle::Normal => 1,
            BarStyle::Thin => 1,
            BarStyle::Minimal => 1,
        }
    }
}


/// Unified axis renderer for consistent axis, tick, and label handling
pub struct AxisRenderer {
    chart_width: usize,
    pub y_axis_offset: usize,
}

impl AxisRenderer {
    pub fn new(chart_width: usize) -> Self {
        use crate::plot::RenderUtils;
        Self {
            chart_width,
            y_axis_offset: RenderUtils::Y_AXIS_LABEL_WIDTH,
        }
    }

    /// Render X-axis line with consistent positioning
    pub fn render_x_axis_line(&self) -> String {
        let mut result = String::new();
        
        // Use consistent Y-axis offset
        result.push_str(&format!("{:>width$}└", "", width = self.y_axis_offset));
        
        // Draw horizontal line
        for _ in 0..self.chart_width {
            result.push('─');
        }
        result.push('\n');
        
        result
    }


    /// Render X-axis labels with consistent positioning and layout
    pub fn render_x_axis_labels(&self, labels: &[String]) -> String {
        if labels.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        
        // Use consistent Y-axis offset
        result.push_str(&format!("{:>width$} ", "", width = self.y_axis_offset));
        
        // Create label layout and use Canvas tick positioning for alignment
        let label_layout = ElementLayout::for_ticks(self.chart_width, labels.len());
        let mut label_line = vec![' '; self.chart_width];
        
        // Place labels at Canvas tick positions for proper alignment
        for (i, label) in labels.iter().enumerate() {
            let position = label_layout.canvas_tick_position(i, self.chart_width, labels.len());
            let label_start = position.saturating_sub(label.len() / 2);
            
            // Place the label if there's space
            for (j, ch) in label.chars().enumerate() {
                if label_start + j < self.chart_width {
                    label_line[label_start + j] = ch;
                }
            }
        }
        
        let label_str: String = label_line.iter().collect();
        result.push_str(&label_str);
        result.push('\n');
        
        result
    }

    /// Generate evenly spaced numeric labels for a range
    pub fn generate_numeric_labels(&self, data_len: usize, num_labels: usize) -> Vec<String> {
        let effective_labels = num_labels.min(data_len).max(1);
        let mut labels = Vec::new();
        
        for i in 0..effective_labels {
            let x_index = if data_len > 1 {
                i * (data_len - 1) / (effective_labels - 1)
            } else {
                0
            };
            labels.push(x_index.to_string());
        }
        
        labels
    }

    /// Generate numeric labels for a specific value range (for scatter plots)
    pub fn generate_range_labels(&self, min_val: f64, max_val: f64, num_labels: usize) -> Vec<String> {
        let effective_labels = num_labels.max(1);
        let mut labels = Vec::new();
        
        if effective_labels == 1 {
            labels.push(format!("{:.1}", (min_val + max_val) / 2.0));
        } else {
            for i in 0..effective_labels {
                let ratio = i as f64 / (effective_labels - 1) as f64;
                let value = min_val + ratio * (max_val - min_val);
                labels.push(format!("{:.1}", value));
            }
        }
        
        labels
    }

    /// Render Y-axis labels for scatter plots and other Canvas-based plots
    pub fn render_y_axis_labels(&self, canvas_output: &str, y_min: f64, y_max: f64, _chart_height: usize) -> String {
        let lines: Vec<&str> = canvas_output.lines().collect();
        let mut result = String::new();
        
        // Separate title lines from chart content
        let mut title_lines = Vec::new();
        let mut chart_lines = Vec::new();
        let mut in_chart = false;
        
        for line in lines.iter() {
            if line.contains('│') || line.contains('├') || line.contains('└') {
                in_chart = true;
            }
            
            if in_chart {
                chart_lines.push(*line);
            } else if !line.trim().is_empty() {
                title_lines.push(*line);
            }
        }
        
        // Add title lines as-is
        for title_line in &title_lines {
            result.push_str(title_line);
            result.push('\n');
        }
        if !title_lines.is_empty() {
            result.push('\n'); // Add spacing after title
        }
        
        // Process chart lines with Y-axis labels
        let effective_height = chart_lines.len();
        
        for (i, line) in chart_lines.iter().enumerate() {
            // Calculate Y value for this row (excluding the bottom axis line)
            let chart_rows = effective_height.saturating_sub(1).max(1); // Exclude bottom axis
            let y_ratio = if chart_rows > 1 && i < chart_rows {
                1.0 - (i as f64 / (chart_rows - 1) as f64)
            } else {
                0.0 // Bottom axis line
            };
            let y_value = y_min + y_ratio * (y_max - y_min);
            
            // Add Y-axis label every few rows, but not on the bottom axis line
            if line.contains('└') {
                // Bottom axis line - use consistent spacing
                result.push_str(&format!("{:>6} ", ""));
            } else if i % 3 == 0 {
                result.push_str(&format!("{:>6.1} ", y_value));
            } else {
                result.push_str("       ");
            }
            
            result.push_str(line);
            result.push('\n');
        }
        
        result
    }

    /// Generate bin center labels for histograms
    pub fn generate_bin_labels(&self, bin_edges: &[f64]) -> Vec<String> {
        let mut labels = Vec::new();
        
        for i in 0..bin_edges.len().saturating_sub(1) {
            let bin_center = (bin_edges[i] + bin_edges[i + 1]) / 2.0;
            labels.push(format!("{:.0}", bin_center));
        }
        
        labels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_layout_for_bars() {
        let layout = ElementLayout::for_bars(50, 5);
        assert_eq!(layout.element_width, 2); // Preferred width
        assert!(layout.spacing >= 1); // Should have some spacing
        assert!(layout.total_width(5) <= 50); // Should fit in available space
    }

    #[test]
    fn test_element_layout_narrow_space() {
        let layout = ElementLayout::for_bars(10, 8);
        assert!(layout.element_width >= 1); // Should be at least minimum width
        assert!(layout.total_width(8) <= 10); // Should fit in available space
    }

    #[test]
    fn test_element_position() {
        let layout = ElementLayout::for_bars(50, 5);
        let pos1 = layout.element_position(0);
        let pos2 = layout.element_position(1);
        assert!(pos2 > pos1); // Second position should be after first
        assert_eq!(pos2 - pos1, layout.element_width + layout.spacing);
    }

    #[test]
    fn test_max_elements_for_width() {
        let layout = ElementLayout::for_bars(20, 5);
        let max_elements = layout.max_elements_for_width(20);
        assert!(max_elements >= 5); // Should fit at least the original count
    }

    #[test]
    fn test_bar_style() {
        let wide = BarStyle::from_width(2);
        let normal = BarStyle::from_width(1);
        assert_eq!(wide.get_symbol(), '█');
        assert_eq!(normal.get_symbol(), '█');
    }

    #[test]
    fn test_tick_style() {
        let tick = TickStyle::Standard;
        assert_eq!(tick.get_symbol(), '┴');
    }
}