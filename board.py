class Board:
    def __init__(self, width, height):
        self.cars = []
        self.width = width
        self.height = height
        self.exit_y = 2

    # Finds and returns the car at (x, y). If there isn't any, returns None
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

    # Hash function
    def __hash__(self):
        res = hash((self.width, self.height))
        for car in self.cars:
            # I don't know how to rotate `res`, so this shall do
            res = res ^ hash(car)
        return res

    # Returns true if the position is solved
    def solved(self):
        return len(self.cars) > 0 and self.cars[0][0] + self.cars[0][2] == self.width

    # Returns an instance of Board where the `index`-th car was moved to `(x, y)`
    def move(self, index, x, y):
        res = Board(self.width, self.height) # On crée une nouvelle instance de Board
        res.exit_y = self.exit_y
        for n in range(0, len(self.cars)): # Les voitures sont clonées,
            if n == index: # à l'exception de la voiture `index`, qui reçoit une nouvelle position
                res.cars.append((x, y, self.cars[n][2], self.cars[n][3]))
            else:
                res.cars.append(self.cars[n])

        return res

    # Returns a list of child states and the movements to get to these
    def next_states(self):
        res = [] # La liste à retourner, contenant des paires de positions et de mouvements
        for n in range(0, len(self.cars)): # Pour chaque voiture dans self.cars
            car = self.cars[n]
            if car[3]: # Si la voiture est horizontale
                for x in range(0, car[0]): # Pour x dans [0; car.x[
                    if self.get_car(car[0] - x - 1, car[1]) == None: # Si l'espace est vide:
                        # Ajouter la nouvelle position ainsi que le mouvement fait à res
                        res.append((self.move(n, car[0] - x - 1, car[1]), (n, car[0] - x - 1, car[1])))
                    else: # Sinon, s'arrêter
                        break
                for x in range(car[0], self.width - car[2]): # Pour x dans [car.x; width - car.length[
                    if self.get_car(x + car[2], car[1]) == None:
                        res.append((self.move(n, x + 1, car[1]), (n, x + 1, car[1])))
                    else:
                        break
            else: # Si la voiture est verticale
                for y in range(0, car[1]): # Pour y dans [0; car.y[
                    if self.get_car(car[0], car[1] - y - 1) == None:
                        res.append((self.move(n, car[0], car[1] - y - 1), (n, car[0], car[1] - y - 1)))
                    else:
                        break
                for y in range(car[1], self.height - car[2]): # Pour y dans [car.y; height - car.length[
                    if self.get_car(car[0], y + car[2]) == None:
                        res.append((self.move(n, car[0], y + 1), (n, car[0], y + 1)))
                    else:
                        break
        return res
