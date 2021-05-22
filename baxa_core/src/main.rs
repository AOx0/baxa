pub mod error;

/// baxa - means trash in otomí. 
/// 
/// Interpreted "trash" lang.
pub mod baxa {
    /// File manager. Used to read baxa code of files (.bx)
    pub mod file_manager;
}

/// Baxa virtual machine
pub mod bxvm {
    /// Definition of "asm"  bytecodes for bxvm
    pub mod bytecode;
    /// Main interface of bxvm
    pub mod vm;
    /// Definition of the heap and all its methods
    pub mod heap;
    
    /// Definition of the heap and all its methods
    pub mod stack;
}