import json
from selenium import webdriver
from bs4 import BeautifulSoup
import time
from urllib.parse import urlparse

def initialize_driver():
    options = webdriver.ChromeOptions()
    options.add_argument('--headless')
    options.add_argument('--disable-gpu')
    driver = webdriver.Chrome(options=options)
    return driver

def scrape_content(url):
    driver = initialize_driver()
    driver.get(url)
    time.sleep(3)

    soup = BeautifulSoup(driver.page_source, 'html.parser')
    texts = soup.find_all(text=True)
    visible_texts = filter(tag_visible, texts)
    content = ' '.join(t.strip() for t in visible_texts)

    driver.quit()
    return content

def tag_visible(element):
    from bs4.element import Comment
    if element.parent.name in ['style', 'script', 'head', 'title', 'meta', '[document]']:
        return False
    if isinstance(element, Comment):
        return False
    return True

def export_to_json(data, filename):
    with open(filename, 'w', encoding='utf-8') as file:
        json.dump(data, file, ensure_ascii=False, indent=4)

def main_menu():
    print("\nWeb Scraper Menu")
    print("1. Enter URL to scrape")
    print("2. Exit")
    choice = input("Enter your choice: ")
    return choice

def main():
    while True:
        choice = main_menu()
        if choice == '1':
            url = input("Enter the URL: ")
            try:
                content = scrape_content(url)
                if content:
                    domain = urlparse(url).netloc
                    filename = f"{domain}_content.json"
                    export_to_json(content, filename)
                    print(f"Content exported successfully to '{filename}'.")
                else:
                    print("No content found on the page.")
            except Exception as e:
                print(f"An error occurred: {e}")
        elif choice == '2':
            print("Exiting the program.")
            break
        else:
            print("Invalid choice. Please enter 1 or 2.")

if __name__ == "__main__":
    main()
