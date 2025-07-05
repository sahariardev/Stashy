# Stashy

  

**Stashy** is a simple, persistent key-value store server built with Rust. It's designed to be a lightweight and easy-to-use database for your projects.

  

---

  

## Features

  

-  **Simple Command Interface**

Interact with the server using straightforward `GET`, `SET`, and `DELETE` commands.

  

-  **Data Persistence**

Data is saved to a `data.json` file to ensure nothing is lost when the server shuts down.

  

-  **Asynchronous I/O**

Built with `tokio` for efficient, non-blocking network communication.

  

---

  

## Getting Started

  

### Prerequisites

  

- Rust programming language and Cargo package manager

[Install Rust](https://www.rust-lang.org/tools/install)

  

### Building and Running

  

```bash
# Clone the repository
git  clone  https://github.com/sahariardev/stashy.git 

# Navigate to the project directory
cd  stashy

# Build and run the server
cargo  run
```

The server will start and listen on 127.0.0.1:6379

---
## Usage

  

You can interact with the server using a TCP client like `netcat` or `telnet`.

  

### Commands

**SET**

-   `SET <key> <value>`: Stores a value for a given key.
    
-   Example: `SET name Stashy`
    
-   Returns: `OK`

   **GET**
    
 -   `GET <key>`: Retrieves the value for a given key.
        
 -   Example: `GET name`
        
 -   Returns: The value if the key exists, otherwise "Error Key not found".
        
**DELETE**
    
  -   `DELETE <key>`: Deletes a key-value pair.
        
  -   Example: `DELETE name`
        
  -   Returns: `Ok` if the key was deleted, "Key not found" if the key didn't exist.

## Dependencies

-   [tokio](https://tokio.rs/): An asynchronous runtime for the Rust programming language.
    
-   [serde](https://serde.rs/): A framework for serializing and deserializing Rust data structures efficiently and generically.
    
-   [serde_json](https://github.com/serde-rs/json): A JSON serialization library for Serde.
    
-   [anyhow](https://github.com/dtolnay/anyhow): A library for flexible error handling.

## License

This project is not licensed. Consider adding a `LICENSE` file to let others know how they can use your code. The [MIT License](https://opensource.org/licenses/MIT) is a popular choice for open-source projects.