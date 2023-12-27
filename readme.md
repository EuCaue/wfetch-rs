# wfetch

<!--toc:start-->

- [wfetch](#wfetch)
  - [Usage](#usage)
  - [Contribution](#contribution)
  - [License](#license)
  <!--toc:end-->

wfetch is a minimalistic command-line tool written in Rust for give your information about the current weather.

## Usage

<small>you need to have **Cargo** installed.</small>

1. Clone the wfetch repository:

   ```bash
   git clone https://github.com/EuCaue/wfetch.git
   ```

2. Navigate to the project directory:

   ```bash
   cd wfetch
   ```

3. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

4. Run wfetch --api-key, to setup the api key:

   ```bash
   ./target/release/wfetch --api-key=<API_KEY>
   ```

5. Run wfetch --setup, to setup the location:

   ```bash
   ./target/release/wfetch --setup
   ```

6. Run wfetch to get the current weather:

   ```bash
   ./target/release/wfetch
   ```

## Contribution

Contributions are welcome! _(since I'm learning rust, probably has a lot of things to improve)_ Fork this repository, make your changes, and submit a pull request.

## License

wfetch is licensed under the GPL3 License.

Thank you for using wfetch for your terminal theme customization needs!
