import re

file = open("text.txt")
text = file.read()
file.close()

text = re.sub("[\n\r\t]", " ", text)
text = re.sub(" {2,}", " ", text)
text = re.sub("[.,?!] ", " ", text)
text = text.strip()

print(text)
