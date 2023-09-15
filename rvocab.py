import re
import os

if not os.path.exists('text.txt'):
    print("ОШИБКА: text.txt не найден.")
    exit()

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

resulting_words.sort(key=lambda x: x[0])
resulting_words.reverse()

resulting_list = []

for word in resulting_words:
    resulting_list.append(str(word[0]) + "  " + word[1])

about_unique_words = "Уникальных слов: " + str(len(resulting_words))

rational_vocabularu = "\n".join(resulting_list)
rational_vocabularu = about_unique_words + "\n\n" + rational_vocabularu

file = open("rational_vocabularu.txt", "w")
file.write(rational_vocabularu)

if os.path.exists('rational_vocabularu.txt'):
    print("СДЕЛАНО: rational_vocabularu.txt успешно создан.")
    print(about_unique_words)
else:
    print("ОШИБКА: rational_vocabularu.txt не был создан.")
    exit()
