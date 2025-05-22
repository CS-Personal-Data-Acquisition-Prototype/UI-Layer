# UI-Layer

## egui with eframe

### Setup
-  Open terminal in package directory
    - `cd .\data-display`

- Install web target
    - `rustup target add wasm32-unknown-unknown`

- Install wasm-bindgen
    - `cargo install wasm-bindgen-cli`

- Install Trunk
    - `cargo install --locked trunk`

<details>
<summary>If trunk installation fails</summary> 
<ul>
    <li>Open powershell in admin mode</li>
    <li>Install chocolatey by executing the following line
        <pre>Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))</pre>
    <li>Install perl</li>
        <pre>choco install strawberryperl</pre>
    <li>Install make</li>
        <pre>choco install make</pre>
    <li>Install specific openssl version</li>
        <pre>choco install openssl --version=1.1.1.2100</pre>
    <li>Set environment variables</li>
        <pre>OPENSSL_DIR="C:\Program Files\OpenSSL-Win64"</pre>
        <pre>OPENSSL_INCLUDE_DIR="C:\Program Files\OpenSSL-Win64\include"</pre>
        <pre>OPENSSL_LIB_DIR="C:\Program Files\OpenSSL-Win64\lib"</pre>
        <pre>OPENSSL_STATIC=1</pre>
    <li>Verify environment variables</li>
        <pre>ls env:</pre>
    <li>Restart IDE and/or terminal to refresh environment variables as needed</li>
    <li>Reattempt trunk installation</li>
        <pre>cargo install --locked trunk</pre>
</ul>
</details>

---
### Building
- Download and build [client-api-lib](https://github.com/CS-Personal-Data-Acquisition-Prototype/client-api-lib) following relevant instructions

- Return to the data-display folder, open Cargo.toml and replace `"path_to_tcp_clent"` with the path to your tcp-client folder

- Run `cargo build`

---
### Local Testing
- Download and build [Rust-Tcp](https://github.com/CS-Personal-Data-Acquisition-Prototype/Rust-Tcp), following relevant instructions

- In a separate window with the TCP server open, run `cargo run --features sql`

- In the data-display window, run `trunk serve` 
    - This will build the project and host a local server that automatically rebuilds, allowing changes to be seen in realtime. 

- To build the project without hosting simply run `cargo build` as normal

---
### Production
Running `trunk build --release` will generate files in a `dist` directory that can be served as static html.

---
# License Notice
To apply the Apache License to your work, attach the following boilerplate notice. The text should be enclosed in the appropriate comment syntax for the file format. We also recommend that a file or class name and description of purpose be included on the same "printed page" as the copyright notice for easier identification within third-party archives.

    Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
    Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
    
    http://www.apache.org/licenses/LICENSE-2.0
    Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.


