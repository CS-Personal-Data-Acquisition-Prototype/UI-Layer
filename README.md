To apply the Apache License to your work, attach the following boilerplate notice, with the fields enclosed by brackets "[]" replaced with your own identifying information. (Don't include the brackets!) The text should be enclosed in the appropriate comment syntax for the file format. We also recommend that a file or class name and description of purpose be included on the same "printed page" as the copyright notice for easier identification within third-party archives.

# License Notice
Copyright [2025] [name of copyright owner]

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

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
- If trunk installation fails
    - Open powershell in admin mode
    - Install chocolatey by executing the following line
        - `Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))`
    - Install perl
        - `choco install strawberryperl`
    - Install make
        - `choco install make`
    - Install specific openssl version
        - `choco install openssl --version=1.1.1.2100`
    - Set environment variables
        - `OPENSSL_DIR="C:\Program Files\OpenSSL-Win64"`
        - `OPENSSL_INCLUDE_DIR="C:\Program Files\OpenSSL-Win64\include"`
        - `OPENSSL_LIB_DIR="C:\Program Files\OpenSSL-Win64\lib"`
        - `OPENSSL_STATIC=1`
    - Verify environment variables
        - `ls env:`
    - Restart IDE and/or terminal to refresh environment variables as needed
    - Reattempt trunk installation
        - `cargo install --locked trunk`
- Download packages and dependencies
    - `cargo build`

---
### Local Testing
Running `trunk serve` will build the project and host a local server that automatically rebuilds, allowing changes to be seen in realtime. 

To build the project without hosting simply run `cargo build` as normal

---
### Production
Running `trunk build --release` will generate files in a `dist` directory that can be served as static html.
