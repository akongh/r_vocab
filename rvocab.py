import re

file = open("text.txt")
text = file.read().lower()
file.close()

text = re.sub("[\\W\\d_]", " ", text)
text = re.sub(" {2,}", " ", text)
text = text.strip()

text = text.split(" ")
clear_text = []

for word in text:
    if len(word) > 1:
        clear_text.append(word)
        print(word)
