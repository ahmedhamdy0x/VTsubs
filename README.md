# VTsubs - VirusTotal Subdomains Enumeration

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)



<p align="center">
    <img src="https://github.com/user-attachments/assets/4f87b8af-c244-49ae-86e6-34aa84dbace3" alt="subExtreme Screenshot" width="80%"/>
</p>

**VTsubs** is a command-line tool designed to enumerate subdomains for a given domain using the **VirusTotal API**. This tool is useful for penetration testers, security researchers, and anyone interested in identifying subdomains associated with a domain, leveraging the powerful VirusTotal API.

---

## Features

- **Subdomain Enumeration**: Discover subdomains for a given domain by querying the VirusTotal API.
- **VirusTotal API Key Integration**: Authenticate requests using your personal VirusTotal API key.
- **Output to File**: Export the discovered subdomains into a text file.
- **User-Friendly Output**: Easy-to-read, colored output showing the found subdomains.

---

## Requirements

To use **VTsubs**, you need the following:

- **Rust**: The programming language used for development. Install it from [rust-lang.org](https://www.rust-lang.org/).
- **VirusTotal API Key**: Obtain an API key from [VirusTotal](https://www.virustotal.com/) to authenticate API requests.

---

## Install Rust

If you don't have Rust installed, you can install it using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, reload the environment:

```bash
source $HOME/.cargo/env
```

---

## VTsubs Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/ahmedhamdy0x/VTsubs.git
   cd VTsubs
   ```

2. **Build the project**:

   Ensure you have Rust installed, then run:

   ```bash
   cargo build --release
   ```

3. **Install Dependencies**:
   
   Ensure that all dependencies are correctly installed by running:

    ```bash
    cargo build
    ```

   This will create the executable in the `target/release/` directory.

4. **Add VTsubs to Your System PATH**:

   Move it to `/usr/local/bin/` to make the tool globally accessible:

   ```bash
   sudo cp target/release/VTsubs /usr/local/bin/
   ```
   
5. **Make subExtreme Executable**:

   Ensure that the tool is executable:

   ```bash
   sudo chmod +x /usr/local/bin/VTsubs
   ```


---

## Usage

To run the tool and enumerate subdomains for a domain, use the following syntax:

```bash
VTsubs -d <DOMAIN> -a <API_KEY> -o <OUTPUT_FILE>
```

### Arguments

- `-d` or `--domain` (required): The target domain (e.g., `example.com`).
- `-a` or `--api` (required): Your VirusTotal API key.
- `-o` or `--output` (optional): The output file where the subdomains will be saved. Default is `output.txt`.

### Example

```bash
VTsubs -d example.com -a 4f7be9aaa6ad6421cd64b66d3e30f1b7ad651cbebe4d450b293509e21401e548 -o subdomains.txt
```

This command will retrieve all the subdomains for `example.com` using your VirusTotal API key and save them to `subdomains.txt`.

---

## Example Output

Upon execution, the output will display the discovered subdomains in the terminal:

```bash
 __      _________        _         
 \ \    / |__   __|      | |        
  \ \  / /   | |___ _   _| |__  ___ 
   \ \/ /    | / __| | | | '_ \/ __|
    \  /     | \__ | |_| | |_) \__ \
     \/      |_|___/\__,_|_.__/|___/
                                  
    Author:  @ahmedhamdy0x
    YouTube: Gentil Security
    Version: 0.1.0

Domain: example.com
API Key: YOUR_API_KEY
Output File: subdomains.txt

[+] Found Subdomain: https://community.example.com
[+] Found Subdomain: https://admin.example.com
[+] Found Subdomain: https://shop.example.com
...
```

The discovered subdomains will also be written to the specified output file (`subdomains.txt`).

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgements

- **VirusTotal API**: Used for querying domain and subdomain information.
- **Rust**: The programming language used to develop this tool.

---

## Author

- **Ahmed Hamdy**
  - [GitHub](https://github.com/ahmedhamdy0x)
  - [YouTube: Gentil Security](https://www.youtube.com/channel/UCX2bbhX3fF6w8_hOqwpJpCw)

