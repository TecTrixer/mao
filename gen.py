color = ["Heart", "Diamond", "Club", "Spade"]
number = ["Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King", "Ace"]
i = 0
for col in color:
    for num in number:
        i += 1
        lower = col[0].lower() + col[1:]
        print(col + num + " " + lower + num + " = " + str(i) + ";")
