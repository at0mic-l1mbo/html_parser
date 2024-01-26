Web Crawler and DNS Resolver

Overview

This Rust-based web crawler and DNS resolver provide a powerful and flexible tool for extracting information from web pages and resolving associated IP addresses. With a sleek command-line interface, it's designed for simplicity and efficiency.
Features

    Randomized IP Spoofing: Utilizes random IP addresses for web requests to enhance anonymity.
    User-Agent Rotation: Employs fake User-Agents for each request, adding an extra layer of disguise.
    DNS Resolution: Resolves domain names to IP addresses using the Trust-DNS Resolver library.
    HTML Parsing: Extracts hyperlinks from HTML documents, enabling efficient link analysis.
    Output to File: Saves resolved IP addresses with associated URLs to a customizable output file.

Usage

    Clone the repository: git clone https://github.com/yourusername/your-repo.git
    Navigate to the project directory: cd your-repo
    Build and run: cargo run -- https://www.example.com

Requirements

    Rust
    Cargo
    Trust-DNS Resolver
    Reqwest

Getting Started

    Install Rust and Cargo by following the official Rust installation guide.
    Add dependencies to your Cargo.toml.
    Customize the code and configuration as needed for your specific use case.
    Run the application and explore the results in the index.txt file.

Contributions

Contributions are welcome! Feel free to open issues, submit pull requests, or provide feedback. Together, we can make this project even better.
License

This project is licensed under the MIT License - see the LICENSE file for details.
