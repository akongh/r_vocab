import os
import re

if not os.path.exists('text.txt'):
    print("ERROR\n"
          "File \"text.txt\" not found.")
    exit()

file = open("text.txt")
words = file.read().lower()
file.close()

words = re.sub("[^a-zA-Z]", " ", words)
words = re.sub(" {2,}", " ", words)
words = words.strip()

words = words.split(" ")
cleared_words = []

for word in words:
    if len(word) > 3:
        cleared_words.append(word)

cleared_words.sort()
cnt = 0
resulting_words = []
i_word = cleared_words[0]

for word in cleared_words:
    if i_word == word:
        cnt += 1
    else:
        resulting_words.append((cnt, i_word))
        i_word = word
        cnt = 1

resulting_words.sort(key=lambda x: x[0], reverse=True)

resulting_list = []

for word in resulting_words:
    resulting_list.append(str(word[0]) + "  " + word[1])

about_unique_words = "Unique words: " + str(len(resulting_words)) + "."

rvocab = "\n".join(resulting_list)
rvocab = about_unique_words + "\n\n" + rvocab

file = open("rvocab.txt", "w")
file.write(rvocab)

if os.path.exists('rvocab.txt'):
    print("DONE\n"
          "File \"rvocab.txt\" successfully created.\n" +
          about_unique_words)
else:
    print("ERROR\n"
          "File \"rvocab.txt\" was not created.")
    exit()
