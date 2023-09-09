import re

file = open("text.txt")
words = file.read().lower()
file.close()

words = re.sub("[\\W\\d_]", " ", words)
words = re.sub(" {2,}", " ", words)
words = words.strip()

words = words.split(" ")
cleared_words = []

for word in words:
    if len(word) > 1:
        cleared_words.append(word)

del words
