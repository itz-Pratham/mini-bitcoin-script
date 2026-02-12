use crate::error::ScriptError;

/// Determines whether a byte slice is "true" under Bitcoin Script semantics.
///
/// Bitcoin defines false as any representation of zero:
/// - Empty byte vector
/// - All bytes `0x00`, except the last byte may be `0x80` (negative zero)
///
/// Everything else is true.
pub(crate) fn is_true(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }
    for byte in &bytes[..bytes.len() - 1] {
        if *byte != 0x00 {
            return true;
        }
    }
    let last = bytes[bytes.len() - 1];
    last != 0x00 && last != 0x80
}

/// Internal execution stack for the Bitcoin Script engine.
///
/// Elements are arbitrary byte vectors (`Vec<u8>`). The stack grows
/// upward: `push` appends to the end, `pop` removes from the end.
pub(crate) struct Stack {
    items: Vec<Vec<u8>>,
}

impl Stack {
    /// Creates an empty stack.
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Pushes a byte vector onto the top of the stack.
    pub(crate) fn push(&mut self, item: Vec<u8>) {
        self.items.push(item);
    }

    /// Removes and returns the top element.
    ///
    /// Returns `ScriptError::StackUnderflow` if the stack is empty.
    pub(crate) fn pop(&mut self) -> Result<Vec<u8>, ScriptError> {
        self.items.pop().ok_or(ScriptError::StackUnderflow)
    }

    /// Returns a reference to the top element without removing it.
    ///
    /// Returns `ScriptError::StackUnderflow` if the stack is empty.
    pub(crate) fn peek(&self) -> Result<&[u8], ScriptError> {
        self.items
            .last()
            .map(|v| v.as_slice())
            .ok_or(ScriptError::StackUnderflow)
    }

    /// Returns the number of elements on the stack.
    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if the stack contains no elements.
    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Pushes a boolean value using Bitcoin Script encoding.
    ///
    /// `true` is encoded as `[0x01]`, `false` as `[]` (empty vector).
    pub(crate) fn push_bool(&mut self, val: bool) {
        if val {
            self.items.push(vec![0x01]);
        } else {
            self.items.push(vec![]);
        }
    }

    /// Removes and returns the element at the given index (0 = bottom).
    ///
    /// Returns `ScriptError::StackUnderflow` if the index is out of bounds.
    /// Used by OP_NIP to remove the second-from-top element.
    pub(crate) fn remove(&mut self, idx: usize) -> Result<Vec<u8>, ScriptError> {
        if idx >= self.items.len() {
            return Err(ScriptError::StackUnderflow);
        }
        Ok(self.items.remove(idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_true truth table ──────────────────────────────────────────

    #[test]
    fn is_true_empty() {
        assert!(!is_true(&[]));
    }

    #[test]
    fn is_true_zero() {
        assert!(!is_true(&[0x00]));
    }

    #[test]
    fn is_true_negative_zero() {
        assert!(!is_true(&[0x80]));
    }

    #[test]
    fn is_true_multi_byte_zero() {
        assert!(!is_true(&[0x00, 0x00]));
    }

    #[test]
    fn is_true_multi_byte_negative_zero() {
        assert!(!is_true(&[0x00, 0x80]));
    }

    #[test]
    fn is_true_three_byte_negative_zero() {
        assert!(!is_true(&[0x00, 0x00, 0x80]));
    }

    #[test]
    fn is_true_one() {
        assert!(is_true(&[0x01]));
    }

    #[test]
    fn is_true_negative_one() {
        assert!(is_true(&[0x81]));
    }

    #[test]
    fn is_true_nonzero_low_byte() {
        assert!(is_true(&[0x00, 0x01]));
    }

    #[test]
    fn is_true_0x80_not_last() {
        // 0x80 in a non-last position is non-zero
        assert!(is_true(&[0x80, 0x00]));
    }

    // ── Stack operations ─────────────────────────────────────────────

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(vec![0x01, 0x02]);
        assert_eq!(stack.len(), 1);
        let item = stack.pop().unwrap();
        assert_eq!(item, vec![0x01, 0x02]);
        assert!(stack.is_empty());
    }

    #[test]
    fn pop_empty_stack() {
        let mut stack = Stack::new();
        let err = stack.pop().unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow));
    }

    #[test]
    fn peek_returns_top() {
        let mut stack = Stack::new();
        stack.push(vec![0xaa]);
        stack.push(vec![0xbb]);
        assert_eq!(stack.peek().unwrap(), &[0xbb]);
        assert_eq!(stack.len(), 2); // peek doesn't remove
    }

    #[test]
    fn peek_empty_stack() {
        let stack = Stack::new();
        let err = stack.peek().unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow));
    }

    #[test]
    fn push_bool_true() {
        let mut stack = Stack::new();
        stack.push_bool(true);
        assert_eq!(stack.pop().unwrap(), vec![0x01]);
    }

    #[test]
    fn push_bool_false() {
        let mut stack = Stack::new();
        stack.push_bool(false);
        let val = stack.pop().unwrap();
        assert!(val.is_empty());
    }

    #[test]
    fn remove_second_from_top() {
        let mut stack = Stack::new();
        stack.push(vec![0x01]); // index 0 (bottom)
        stack.push(vec![0x02]); // index 1 (top)
                                // Remove second-from-top (index 0)
        let removed = stack.remove(0).unwrap();
        assert_eq!(removed, vec![0x01]);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek().unwrap(), &[0x02]);
    }

    #[test]
    fn remove_out_of_bounds() {
        let mut stack = Stack::new();
        stack.push(vec![0x01]);
        let err = stack.remove(5).unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow));
    }

    #[test]
    fn lifo_order() {
        let mut stack = Stack::new();
        stack.push(vec![0x01]);
        stack.push(vec![0x02]);
        stack.push(vec![0x03]);
        assert_eq!(stack.pop().unwrap(), vec![0x03]);
        assert_eq!(stack.pop().unwrap(), vec![0x02]);
        assert_eq!(stack.pop().unwrap(), vec![0x01]);
    }
}
