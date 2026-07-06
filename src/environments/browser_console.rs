use crate::environments::Environment;

/// The browser console environment renders a complete console.log statement, ready to paste or eval
pub struct BrowserConsoleEnv;

impl Environment for BrowserConsoleEnv {}
