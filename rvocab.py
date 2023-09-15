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
    if len(word) > 3:
        cleared_words.append(word)

del words

cleared_words.sort()
cnt = 0
resulting_words = []
i_word = cleared_words[0]

for word in cleared_words:
    if i_word == word:
        cnt += 1
    else:
        resulting_words.append(str(cnt) + " - " + i_word)
        i_word = word
        cnt = 1

resulting_words.sort()

for word in resulting_words:
    print(word)
