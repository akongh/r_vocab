import re

file = open("text.txt")
words = file.read().lower()
file.close()

words = re.sub("[\\W\\d_]", " ", words)
words = re.sub(" {2,}", " ", words)
words = words.strip()

words = words.split(" ")
cleared_words = []
count = 0

for word in words:
    if len(word) > 1:
        count += 1
        print(f"{count} - {word}")
        cleared_words.append(word)
