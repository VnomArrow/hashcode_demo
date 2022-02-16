pizza = open("pizza.txt")
pizza = pizza.read()
points = 0
ingredients = pizza.split()[1:]

preferences = open("input.txt")
preferences = preferences.read()
people_likes_and_dislikes = preferences.split("\n")
print(people_likes_and_dislikes)
length = len(people_likes_and_dislikes)
line_iter = iter(people_likes_and_dislikes)

customers = int(next(line_iter))

for _ in range(customers):
    pizza_ok = True
    likes = next(line_iter).split()[1:]
    dislikes = next(line_iter).split()[1:]
    for topping in likes:
        if not topping in ingredients:
            pizza_ok = False
    for topping in dislikes:
        if topping in ingredients:
            pizza_ok = False
    if pizza_ok:
        points += 1


print(points)