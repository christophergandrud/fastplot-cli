# Comprehensive Code Review: fastplot-cli

## Overall Assessment

The fastplot-cli project demonstrates mixed adherence to deep module design principles. While it shows good separation of concerns in some areas, there are significant opportunities to reduce complexity through better abstraction and interface design. The codebase exhibits symptoms of **temporal decomposition** and **information leakage** that compromise long-term maintainability.

## Strengths

**1. Good Domain Separation**
- `/Users/christophergandrud/git_repos/fastplot-cli/src/color.rs` provides an excellent example of a **deep module** - simple interface (`apply_color`) hiding complex color handling logic
- The coordinate transformation logic in `/Users/christophergandrud/git_repos/fastplot-cli/src/coordinates.rs` properly encapsulates mathematical complexity
- Function evaluation in `/Users/christophergandrud/git_repos/fastplot-cli/src/function.rs` successfully abstracts the `evalexpr` library complexity

**2. Effective Use of Types**
- The `DataPoint` enum in `/Users/christophergandrud/git_repos/fastplot-cli/src/data.rs` (lines 7-10) cleanly represents the categorical vs numeric distinction
- `RenderPriority` enum in `/Users/christophergandrud/git_repos/fastplot-cli/src/layered_canvas.rs` (lines 4-10) provides clear rendering order semantics

## Critical Complexity Issues

### 1. Shallow Command Interface (High Severity)

**Location:** `/Users/christophergandrud/git_repos/fastplot-cli/src/main.rs` lines 25-102

The CLI command definitions suffer from **interface explosion**. Each command duplicates parameters with inconsistent defaults:

```rust
Commands::Scatter { source, title, point_char, color, range, points }
Commands::Line { source, title, style, points_only, lines_only, point_char, line_char, color, range, points }
Commands::Bar { source, title, bar_char, bar_width, color, range, points, category_order }
```

**Problems:**
- **Change amplification:** Adding a new parameter requires modifying multiple command variants
- **Cognitive load:** Users must remember different parameter names across commands
- **Information leakage:** Implementation details (points count, ranges) exposed at the top level

**Recommendation:** Create a unified `PlotConfig` struct that encapsulates common parameters:

```rust
struct PlotConfig {
    source: String,
    title: String,
    color: Option<String>,
    range: Option<String>,
    points: usize,
}

enum Commands {
    Scatter { config: PlotConfig, point_char: char },
    Line { config: PlotConfig, style: LineStyle },
    Bar { config: PlotConfig, bar_char: char, bar_width: usize },
}
```

### 2. Data Type Confusion (High Severity)

**Location:** Multiple files show type conversion chaos

The codebase maintains **two different `DataPoint` types**:
- `/Users/christophergandrud/git_repos/fastplot-cli/src/data.rs` lines 7-10: Modern enum-based type
- `/Users/christophergandrud/git_repos/fastplot-cli/src/coordinates.rs` lines 4-8: Legacy struct type

**Problems:**
- **Unknown unknowns:** Developers must understand which DataPoint type to use where
- **Information leakage:** The conversion at line 17 in `scatter.rs` loses categorical information
- **Change amplification:** Type changes require updates across multiple conversion points

**Recommendation:** Eliminate the legacy type entirely and create proper coordinate transformation interfaces that preserve type information.

### 3. Canvas Rendering Inconsistency (Medium Severity)

**Location:** Comparison between `/Users/christophergandrud/git_repos/fastplot-cli/src/scatter.rs` and `/Users/christophergandrud/git_repos/fastplot-cli/src/line_plot.rs`

The scatter plot implements its own `CharCanvas` (lines 101-187) while line plots use `LayeredCanvas`. This creates:

**Problems:**
- **Code duplication:** Similar rendering logic implemented differently
- **Cognitive load:** Developers must understand two different canvas abstractions
- **Interface inconsistency:** Different plot types have different rendering capabilities

**Recommendation:** Standardize on `LayeredCanvas` for all plot types to provide consistent rendering capabilities.

### 4. Data Source Parsing Complexity (Medium Severity)

**Location:** `/Users/christophergandrud/git_repos/fastplot-cli/src/data.rs` lines 134-195

The `parse_csv` function exhibits **temporal decomposition** with complex state management:

```rust
let mut is_categorical = false;
let mut categories = Vec::new();
// ... complex state transitions ...
if !is_categorical {
    // First categorical value found - convert previous numeric points
    is_categorical = true;
    let old_points = std::mem::take(&mut points);
    // ... more complex logic ...
}
```

**Problems:**
- **High cognitive load:** Requires understanding state transitions across loop iterations
- **Change amplification:** Adding new data types requires modifying this complex state machine
- **Error prone:** Easy to introduce bugs in the state transition logic

**Recommendation:** Use a two-pass approach: first pass determines data type, second pass parses with known type.

## Interface Design Problems

### 1. Plot Construction Inconsistency

Each plot type has different construction patterns:
- **Scatter:** Direct function call `render_scatter_plot()` at line 190
- **Line:** Builder pattern with `LinePlot::new().with_style()` at lines 19, 33
- **Bar:** Builder pattern with multiple `with_*` methods at lines 36, 41

**Recommendation:** Standardize on a consistent construction interface across all plot types.

### 2. Configuration Parameter Explosion

**Location:** Main command handling in `/Users/christophergandrud/git_repos/fastplot-cli/src/main.rs` lines 107-167

The command handlers manually thread through many parameters, creating **shallow interfaces** that expose too much detail.

**Recommendation:** Create plot-specific configuration structs that encapsulate related parameters.

## Error Handling Patterns

### Inconsistent Error Propagation

**Location:** Throughout the codebase

Some modules use `Result<T>` consistently (`function.rs`), while others mix `Option` and `Result` without clear rationale. The coordinate transformation at line 115 in `coordinates.rs` returns `Option<ScreenPoint>` but doesn't indicate why transformation failed.

**Recommendation:** Establish consistent error handling patterns with descriptive error types.

## Recommendations for Improvement

### Priority 1: Unify Command Interface

Create a deep command processing module that hides parameter complexity:

```rust
pub struct PlotCommand {
    config: PlotConfig,
    plot_type: PlotType,
}

impl PlotCommand {
    pub fn execute(&self) -> Result<String> {
        // Single point of execution logic
    }
}
```

### Priority 2: Eliminate Type Confusion

Remove the legacy `DataPoint` type and create proper abstractions for coordinate transformation that preserve type information throughout the pipeline.

### Priority 3: Standardize Rendering

Adopt `LayeredCanvas` universally and create a consistent plot rendering interface:

```rust
trait PlotRenderer {
    fn render(&self, canvas: &mut LayeredCanvas, transformer: &dyn CoordinateTransformer);
}
```

### Priority 4: Simplify Data Parsing

Replace the complex state machine in CSV parsing with a cleaner two-pass approach or use a more sophisticated parser combinator approach.

## Future Considerations

The current architecture will face challenges when:
1. **Adding new plot types:** Requires changes to main.rs command handling
2. **Supporting multiple data sources:** Current parsing logic is tightly coupled to CSV
3. **Adding interactive features:** No clear place to add UI state management
4. **Supporting different output formats:** Rendering logic is mixed with business logic

The system would benefit from a plugin-style architecture where plot types register themselves and handle their own configuration and rendering logic.

## Conclusion

While the fastplot-cli project demonstrates good intentions with domain separation, it suffers from several **shallow module** antipatterns that will create maintenance burden as the system grows. The primary issues stem from exposing too much complexity at interface boundaries and maintaining redundant abstractions. Addressing the command interface unification and type system cleanup should be the immediate priorities to establish a foundation for sustainable growth.