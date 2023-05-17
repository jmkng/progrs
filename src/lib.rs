/// Represents a progress bar.
pub struct Bar {
    /// Value to represent the start of the bar.
    start: String,
    /// Value to represent the end of the bar.
    end: String,
    /// Value to represent the filled part of the bar.
    filled: char,
    /// Value to represent the empty part of the bar.
    empty: char,
    /// The maximum bar value.
    max: f64,
    /// The bar width.
    width: i32,
    /// The bar's current value.
    value: f64,
    /// Flavor text.
    text: String,
}

impl std::fmt::Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pct = self.value / self.max;
        let filled = (self.width as f64 * pct).ceil();
        let empty = (self.width as f64 - filled).floor();
        write!(f, "{}", self.start)?;
        write!(f, "{}", self.filled.to_string().repeat(filled as usize))?;
        write!(f, "{}", self.empty.to_string().repeat(empty as usize))?;
        write!(f, "{}", self.end)?;

        if !self.text.is_empty() {
            write!(f, " {}", self.text)?;
        }

        Ok(())
    }
}

impl Bar {
    /// Return a new instance of Bar.
    pub fn new() -> Self {
        Bar {
            start: "|".to_string(),
            end: "|".to_string(),
            filled: '█',
            empty: '░',
            text: "".to_string(),
            width: 10,
            max: 100.0,
            value: 0.0,
        }
    }

    /// Set the start token.
    pub fn start<S>(mut self, token: S) -> Self
    where
        S: Into<String>,
    {
        self.start = token.into();

        self
    }

    /// Set the end token.
    pub fn end<S>(mut self, token: S) -> Self
    where
        S: Into<String>,
    {
        self.start = token.into();

        self
    }

    /// Set the filled token.
    pub fn filled(mut self, token: char) -> Self {
        self.filled = token;

        self
    }

    /// Set the empty token.
    pub fn empty(mut self, token: char) -> Self {
        self.empty = token;

        self
    }

    /// Set the maximum value.
    pub fn max(mut self, token: f64) -> Self {
        self.max = token.into();

        self
    }

    /// Set the width of the bar.
    pub fn width(mut self, token: i32) -> Self {
        self.width = token;

        self
    }

    /// Set the progress bar's text.
    pub fn text<S>(mut self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.text = text.into();

        self
    }

    /// Set the value of the bar.
    pub fn value<T: Into<f64>>(mut self, val: T) -> Result<Self, String> {
        let val_f64 = val.into();
        if val_f64 > self.max {
            return Err("value exceeds maximum".to_string());
        }

        self.value = val_f64;

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{stdout, Write},
        thread::sleep,
        time::Duration,
    };

    #[test]
    fn test_bar_display() -> Result<(), String> {
        let one = Bar::new().value(50)?.text("updating");
        assert_eq!(format!("{}", one), "|█████░░░░░| updating");

        let two = Bar::new().value(75)?;
        assert_eq!(format!("{}", two), "|████████░░|");

        Ok(())
    }

    #[test]
    fn test_bar_visual_update() -> Result<(), String> {
        // run test with --nocapture to see bar updating.
        let mut bar = Bar::new().text("testing").value(100)?;

        for i in 1..=10 {
            bar = bar.value(i as f64 * 10.0)?;
            print!("{:<width$}\r", bar, width = bar.width as usize);
            stdout().flush().map_err(|e| e.to_string())?;
            sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}
