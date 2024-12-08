# Overview

As a software engineer looking to deepen my understanding of Rust, I set out to create an inventory management system that tracks different types of items in a game. The goal was to implement key Rust concepts such as memory management, ownership, and borrowing, while also exploring its powerful data structures and functionality. This software serves as a practical demonstration of these concepts, focusing on how to manage a categorized inventory using `HashMap` and `Vec` as the primary data structures.

The program allows users to add, remove, view, and search for items within specified categories like "Worn Items," "Held Items," and "Consumables." By building this system, I aimed to not only reinforce my understanding of Rust syntax but also improve my ability to manage memory safely in an application.

[Software Demo Video](https://youtu.be/nOGqZH-RsAE)

# Development Environment

For this project, I used the following tools and technologies:

- **Programming Language**: Rust
  - Rust is a systems programming language designed for safety, performance, and concurrency. It was ideal for this project due to its strong memory management features and powerful built-in data structures.
  
- **Development Tools**:
  - **VS Code**: I used Visual Studio Code as my primary code editor, with the Rust extension for syntax highlighting, code completion, and other useful features.
  - **Cargo**: Cargo is Rust's build system and package manager. It was used to compile the code and manage dependencies.

- **Libraries**:
  - **std::collections::HashMap**: Used for storing items in categories, allowing efficient lookups and modifications.

# Useful Websites

The following websites were particularly helpful during my project:

- [Rust Official Documentation](https://doc.rust-lang.org)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Rust Tutorials on TutorialsPoint](https://www.tutorialspoint.com/rust/index.htm)

# Future Work

While the program is functional, there are still some improvements and additional features that could be implemented in future versions:

- **Add more detailed item attributes**: Allow for multiple attributes per item, such as durability, rarity, etc.
- **Improve error handling**: Add more robust error handling for user input and edge cases.
- **Persist inventory data**: Implement file I/O to save and load the inventory, so it isn't lost when the program is closed.
- **Extend the item categories**: Add more categories or allow the user to create custom categories dynamically.
- **Enhance search functionality**: Allow searching by attributes or other item properties, not just by name.
