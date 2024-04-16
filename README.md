# Running the Rust Web Scraping Project

To help and guide you through setting up and running the Rust web scraping project on different operating systems: macOS, Windows, and Linux.

## Prerequisites

Before you begin, ensure you have Rust installed. If not, you can install it using `rustup`, which manages Rust versions and associated tools.

In the other hand, if you are using the Python version, simply excute setup.py before excecuting the python version to intall every necessary library.

### Install Rust

Open your terminal or command prompt and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts to complete the installation. After installation, restart your terminal or run

```bash
source $HOME/.cargo/env
```

### Install WebDriver

The project requires a WebDriver for scraping dynamic content. (In case of doubt check the setup.rs file, but it's important to take into account that the setup.rs doesn't work as a setup.py, setup.rs is just to check that all extensions are installed).

#### macOS

Install ChromeDriver for Google Chrome:

```bash
brew install --cask chromedriver
```

Or install geckodriver for Firefox:

```bash
brew install geckodriver
```

### Windows

Download ChromeDriver for Google Chrome from [ChromeDriver - WebDriver for Chrome](https://sites.google.com/a/chromium.org/chromedriver/) or download `geckodriver` for Firefox from [GitHub - mozilla/geckodriver](https://github.com/mozilla/geckodriver/releases). Extract the downloaded file and place it in a directory that is in your system's PATH.

#### Linux

Install ChromeDriver for Google Chrome:

```bash
sudo apt-get install chromium-chromedriver
```

Or install geckodriver for Firefox:

```bash
sudo apt-get install firefox-geckodriver
```

### Excecution

With these steps done, simply run the following command as a usual Rust excecution:

```bash
cargo run
```

Note this project is still being developed, if you want to contribute with this project, feel free to do so :)
