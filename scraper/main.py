import requests
import re
from bs4 import BeautifulSoup

URL = "https://www.berndeutsch.ch/browse?stack=%5B%5D&page="

with open("../berndeutsch.csv", 'wb') as file:
    for i in range(0, 848):
        page = requests.get(URL + str(i))

        soup = BeautifulSoup(page.content, "html.parser")

        h3s = soup.findAll("h3")

        for h3_tag in h3s:
            a_tags = h3_tag.find_all('a')

            # loop through each a tag and do something with it
            for a_tag in a_tags:
                word = re.split(r'[ ,;!?]+', a_tag.text)[0]                # do something with the a tag
                print(word)
                file.write((word + "\n").encode("UTF-8"))