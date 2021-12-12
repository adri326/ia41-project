class Board:
    def __init__(self, width, height):
        self.cars = []
        self.width = width
        self.height = height
        self.exit_y = 2

    def get_car(self, x, y):
        for n in range(0, len(self.cars)):
            if self.cars[n][3]:
                if x >= self.cars[n][0] and \
                    x < self.cars[n][0] + self.cars[n][2] and \
                    y == self.cars[n][1]:
                    return n
            else:
                if y >= self.cars[n][1] and \
                    y < self.cars[n][1] + self.cars[n][2] and \
                    x == self.cars[n][0]:
                    return n

        return None

    def __hash__(self):
        res = hash((self.width, self.height))
        for car in self.cars:
            # I don't know how to rotate `res`, so this shall do
            res = res ^ hash(car)
        return res

    def solved(self):
        return len(self.cars) > 0 and self.cars[0][0] + self.cars[0][2] == self.width

    def move(self, index, x, y):
        res = Board(self.width, self.height)
        res.exit_y = self.exit_y
        for n in range(0, len(self.cars)):
            if n == index:
                res.cars.append((x, y, self.cars[n][2], self.cars[n][3]))
            else:
                res.cars.append(self.cars[n])

        return res

    def next_states(self):
        res = []
        for n in range(0, len(self.cars)):
            car = self.cars[n]
            if car[3]:
                for x in range(0, car[0]):
                    if self.get_car(car[0] - x - 1, car[1]) == None:
                        res.append((self.move(n, car[0] - x - 1, car[1]), (n, car[0] - x - 1, car[1])))
                    else:
                        break
                for x in range(car[0], self.width - car[2]):
                    if self.get_car(x + car[2], car[1]) == None:
                        res.append((self.move(n, x + 1, car[1]), (n, x + 1, car[1])))
                    else:
                        break
            else:
                for y in range(0, car[1]):
                    if self.get_car(car[0], car[1] - y - 1) == None:
                        res.append((self.move(n, car[0], car[1] - y - 1), (n, car[0], car[1] - y - 1)))
                    else:
                        break
                for y in range(car[1], self.height - car[2]):
                    if self.get_car(car[0], y + car[2]) == None:
                        res.append((self.move(n, car[0], y + 1), (n, car[0], y + 1)))
                    else:
                        break
        return res
