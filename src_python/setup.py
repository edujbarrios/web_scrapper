import subprocess
import sys

def install_packages():
    # List of necessary packages
    packages = ['selenium', 'beautifulsoup4']
    # Command to install packages
    for package in packages:
        subprocess.check_call([sys.executable, "-m", "pip", "install", package])

if __name__ == "__main__":
    print("Installing required packages...")
    install_packages()
    print("Installation completed successfully.")
