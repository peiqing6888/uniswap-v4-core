use std::sync::{Arc, RwLock};

/// Lock state for the pool manager
/// This controls whether the pool manager is in a locked or unlocked state
#[derive(Debug, Clone, Default)]
pub struct Lock {
    state: Arc<RwLock<bool>>,
}

impl Lock {
    /// Create a new lock (initially locked)
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Unlock the lock
    pub fn unlock(&self) -> Result<(), LockError> {
        let mut state = self.state.write().unwrap();
        if *state {
            return Err(LockError::AlreadyUnlocked);
        }
        *state = true;
        Ok(())
    }
    
    /// Lock the lock
    pub fn lock(&self) {
        let mut state = self.state.write().unwrap();
        *state = false;
    }
    
    /// Check if the lock is unlocked
    pub fn is_unlocked(&self) -> bool {
        *self.state.read().unwrap()
    }
}

/// Errors that can occur during locking operations
#[derive(Debug, thiserror::Error)]
pub enum LockError {
    #[error("The pool manager is already unlocked")]
    AlreadyUnlocked,
    
    #[error("The pool manager is locked")]
    ManagerLocked,
}

/// A guard that automatically unlocks during creation and locks on drop
pub struct UnlockGuard<'a> {
    lock: &'a Lock,
}

impl<'a> UnlockGuard<'a> {
    /// Create a new unlock guard, which unlocks the lock
    pub fn new(lock: &'a Lock) -> Result<Self, LockError> {
        lock.unlock()?;
        Ok(Self { lock })
    }
}

impl<'a> Drop for UnlockGuard<'a> {
    fn drop(&mut self) {
        self.lock.lock();
    }
} 