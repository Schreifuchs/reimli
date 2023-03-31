import requests
import re
from bs4 import BeautifulSoup

URL = "https://www.berndeutsch.ch/browse?stack=%5B%5D&page="

with open("../berndeutsch.csv", 'wb') as file:
    for i in range(0, 848):
        page = requests.get(URL + str(i))

        soup = BeautifulSoup(page.content, "html.parser")

        last = ""

        for h3_tag in soup.findAll("h3"):
            for a_tag in h3_tag.find_all('a'):
                word = re.split(r'[ \/,;!?]+', a_tag.text)[0]
                if word != last:
                    print(word)
                    file.write((word + "\n").encode("UTF-8"))
                    last = word
                else:
                    print("** "+word+" **")