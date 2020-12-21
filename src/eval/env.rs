//! Handles the scoping of functions and various contexts.
use crate::eval::eval::Object;
use std::collections::HashMap;

/// Scopes contain bound variables and expressions.
#[derive(Clone)]
pub struct Env {
    pub space: HashMap<String, Object>,
    pub enclosing: Option<Box<Env>>, // Pointer to a scope or nothing.
}

impl Env {
    /// Instantiates a new environment object with no outer scope.
    pub fn new() -> Self {
        Env {
            space: HashMap::new(),
            enclosing: None,
        }
    }

    /// Defines a variable/function in the current scope.
    pub fn set(&mut self, key: String, value: Object) {
        self.space.insert(key, value);
    }

    /// Recursively attempts to get a variable/functions value.
    pub fn get(&self, key: &str) -> Option<Object> {
        match (self.space.get(key), &self.enclosing) {
            (Some(val), _) => Some(val.clone()),       //  Variable found
            (None, Some(enclose)) => enclose.get(key), // Query outer scope
            (None, _) => None,                         // Variable not found at outermost scope
        }
    }

    // Enclosing = outer or global typically.
    pub fn new_enclosing(outer: Self) -> Self {
        let mut env = Self::new();
        env.enclosing = Some(Box::new(outer)); // Pointer to original object
        env // Return the inner environment
    }
}
