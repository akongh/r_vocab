import os
import re

# Geting content.
if not os.path.exists('text.txt'):
    print("\n    ERROR\n"
          "    File \"text.txt\" not found.\n")
    exit()
file = open("text.txt")
words = file.read().lower()
file.close()

# Clearing content.
words = re.sub("[^a-zA-Z]", " ", words)
words = re.sub(" {2,}", " ", words)
words = words.strip()

# Making words list from content.
words = words.split()

# Removing short words.
cleared_words = []
for word in words:
    if len(word) > 3:
        cleared_words.append(word)

# Sorting words list.
cleared_words.sort()

# Counting repetitions number for each word and removing duplicates.
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

# Sorting words by repetitions number.
resulting_words.sort(key=lambda x: x[0], reverse=True)

# Tuples into strings.
resulting_list = []
for word in resulting_words:
    resulting_list.append(str(word[0]) + "  " + word[1])

# Making resulting content.
about_unique_words = "    Unique words: " + str(len(resulting_words)) + ".\n"
rvocab = "\n".join(resulting_list)
rvocab = about_unique_words.strip() + "\n\n" + rvocab

# Writing resulting content to output file.
file = open("rvocab.txt", "w")
file.write(rvocab)
if os.path.exists('rvocab.txt'):
    print("\n    DONE\n"
          "    File \"rvocab.txt\" successfully created.\n" +
          about_unique_words)
else:
    print("\n    ERROR\n"
          "    File \"rvocab.txt\" was not created.\n")
    exit()
