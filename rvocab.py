file = open("text.txt")
text = file.read()
file.close()

text = text.lower()

print(text)
