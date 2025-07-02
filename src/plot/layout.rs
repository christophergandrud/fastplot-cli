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
    
    /// Calculate layout for histogram bins
    pub fn for_bins(chart_width: usize, num_bins: usize) -> Self {
        // Similar to bars but optimize for bins
        Self::for_bars(chart_width, num_bins)
    }
    
    /// Calculate layout for axis ticks
    pub fn for_ticks(chart_width: usize, num_ticks: usize) -> Self {
        // Optimize for ticks (minimal width, consistent spacing)
        ElementLayout {
            element_width: 1,
            spacing: if num_ticks > 1 { 
                chart_width.saturating_sub(num_ticks) / num_ticks.saturating_sub(1).max(1)
            } else { 
                0 
            },
            offset: 0,
        }
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

/// Tick mark styles
#[derive(Debug, Clone, Copy)]
pub enum TickStyle {
    Standard, // ┴
}

impl TickStyle {
    pub fn get_symbol(&self) -> char {
        match self {
            TickStyle::Standard => '┴',
        }
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