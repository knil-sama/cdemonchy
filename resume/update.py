from selenium import webdriver
from selenium.webdriver.common.by import By
import tomllib
from time import sleep
if __name__ == "__main__":
    data = None
    with open("src/.secrets.toml", "rb") as f:
        config = tomllib.load(f)
    print(f"{config['malt']['url']}")
    # Chrome not working as I got chromium instead 
    # https://stackoverflow.com/questions/76846675/selenium-ubuntu22-bug-message-binary-is-not-a-firefox-executable
    geckodriver_path = "/snap/bin/geckodriver"
    driver_service = webdriver.FirefoxService(executable_path=geckodriver_path)
    driver = webdriver.Firefox(service=driver_service)

    driver.get(f"{config['malt']['url']}/signin")

    title = driver.title
    print(title)
    sleep(15)
    print("waited")
    #cookie_button = driver.find_element(by=By.ID, value="axeptio_btn_acceptAll")
    #print(f"found {cookie_button}")
    #cookie_button.click()
    #sleep(2)
    email_input = driver.find_element(by=By.ID, value="email")
    print(config['malt']['username'])
    email_input.send_keys(config['malt']['username'])
    password_input = driver.find_element(by=By.ID, value="password")
    password_input.send_keys(config['malt']['password'])

    submit_button = driver.find_element(by=By.CSS_SELECTOR, value="button[type='submit']")
    submit_button.click()
    code_pin = input("pin : ")
    for index, character in enumerate(code_pin):
        input_pin = driver.find_element(by=By.CSS_SELECTOR,value=f"input[name='otp-input-{index+1}']")
        input_pin.send_keys(character)
    submit_button_pin = driver.find_element(by=By.CLASS_NAME, value="joy-dialog--confirm")
    submit_button_pin.click()
    code_pin = input("pin : ")
    print(title)
    #driver.quit()

