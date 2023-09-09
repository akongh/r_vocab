file = open("text.txt")
text = file.read()
file.close()

text = text.lower().replace("\n", " ").replace("\r", "")

print(text)
