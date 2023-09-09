import re

file = open("text.txt")
text = file.read().lower()
file.close()

text = re.sub("[\n\r\t]", " ", text)
text = re.sub(" {2,}", " ", text)
text = re.sub("[.,?!] ", " ", text)
text = text.strip()

text = text.split(" ")

for word in text:
    print(word)

# print(text)
