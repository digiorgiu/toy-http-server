# Toy HTTP Server

A simple HTTP server implementation built in Rust as part of my Rust learning journey. This project demonstrates fundamental concepts of network programming, HTTP protocol handling, and safe file serving.

## 🚀 Features

- **Basic HTTP Server**: Handles incoming TCP connections and serves HTTP responses
- **Static File Serving**: Serves HTML, CSS, and other static files from a public directory
- **Security**: Includes protection against directory traversal attacks
- **Request Parsing**: Custom HTTP request parser implementation
- **Modular Design**: Clean separation of concerns with dedicated modules for HTTP components

## 📁 Project Structure

```
toy-http-server/
├── src/
│   ├── http/                 # HTTP protocol implementation
│   │   ├── method.rs         # HTTP methods (GET, POST, etc.)
│   │   ├── query_string.rs   # Query string parsing
│   │   ├── request.rs        # HTTP request parsing
│   │   ├── response.rs       # HTTP response generation
│   │   └── status_code.rs    # HTTP status codes
│   ├── main.rs              # Application entry point
│   ├── server.rs            # TCP server implementation
│   └── website_handler.rs   # Request handler for static files
├── public/                  # Static files directory
│   ├── index.html          # Homepage
│   ├── about.html          # About page
│   └── style.css           # Stylesheet
└── Cargo.toml              # Project configuration
```

## 🛠 How It Works

1. **Server Setup**: The server binds to `127.0.0.1:8080` and listens for incoming connections
2. **Request Handling**: Each incoming TCP connection is read and parsed as an HTTP request
3. **Routing**: The `WebsiteHandler` routes requests to appropriate static files:
   - `/` → `index.html`
   - `/about` → `about.html`
   - Other paths → Direct file lookup in public directory
4. **Security**: Path canonicalization prevents directory traversal attacks
5. **Response**: Appropriate HTTP responses are sent back to the client

## 🏃‍♂️ Running the Server

1. **Clone and navigate to the project**:
   ```bash
   cd toy-http-server
   ```

2. **Run the server**:
   ```bash
   cargo run
   ```

3. **Access the server**:
   Open your browser and visit `http://127.0.0.1:8080`

## 🎯 Learning Objectives

This project helped me learn:

- **Network Programming**: TCP socket handling and HTTP protocol basics
- **Rust Ownership**: Safe memory management without garbage collection
- **Error Handling**: Rust's `Result` type and error propagation
- **Module System**: Organizing code with Rust's module system
- **Pattern Matching**: Extensive use of `match` expressions
- **Trait System**: Implementing the `Handler` trait for request processing
- **Security Considerations**: Path traversal prevention and input validation

## 🔧 Configuration

The server can be configured through environment variables:

- `PUBLIC_PATH`: Custom path to the public directory (defaults to `./public`)

Example:
```bash
PUBLIC_PATH=/custom/path cargo run
```

## 🚦 Supported HTTP Methods

Currently supports:
- **GET**: For retrieving static files

## ⚠️ Limitations

This is a toy implementation for learning purposes and should **not** be used in production:

- No HTTPS support
- Limited HTTP method support
- Basic error handling
- No concurrent connection handling
- No caching mechanisms
- Simple routing system

## 🎓 Next Steps

Potential improvements for continued learning:
- Add support for more HTTP methods (POST, PUT, DELETE)
- Implement proper logging
- Add configuration file support
- Implement basic templating
- Add unit tests
- Support for concurrent connections
- Add middleware support

---

Built with ❤️ and Rust 🦀
