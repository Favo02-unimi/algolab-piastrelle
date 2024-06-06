import random
import math

def generaPiastrelle(numPiastrelle):
  for _ in range(numPiastrelle):
    x = random.randint(0, bound)
    y = random.randint(0, bound)
    c = colors[random.randint(0, len(colors)-1)]
    i = random.randint(1, 1_000)
    print(f"C {x} {y} {c} {i}")

def spegni(numPiastrelle):
  for _ in range(numPiastrelle):
    x = random.randint(0, bound)
    y = random.randint(0, bound)
    print(f"S {x} {y}")

def stampaTutti():
  for y in range(0, bound+1):
    for x in range(0, bound+1):
      print(f"? {x} {y}")

def generaRegole(numRegole):
  for _ in range(numRegole):
    maxx = random.randint(1, 8)
    finalColor = colors[random.randint(0, len(colors)-1)]
    count = 0
    print(f"r {finalColor}", end="")
    availableColors = colors.copy()
    while count < maxx:
      coeff = random.randint(1, maxx-count)
      colorIndex = random.randint(0, len(availableColors)-1)
      print(f" {coeff} {availableColors[colorIndex]}", end="")
      availableColors = availableColors[:colorIndex] + availableColors[colorIndex+1:]
      count += coeff
    print()
  print("s")

def blocco(numBlocchi):
  for _ in range(numBlocchi):
    x = random.randint(0, bound)
    y = random.randint(0, bound)
    print(f"b {x} {y}") # blocco
    print(f"B {x} {y}") # omogeneo

def propaga(numPropagazioni, forseOrdina=False):
  for i in range(numPropagazioni):
    x = random.randint(0, bound)
    y = random.randint(0, bound)
    if i % 2 == 0:
      print(f"p {x} {y}")
    else:
      print(f"P {x} {y}")
    if forseOrdina and random.randint(1, 10) == 1:
      print("o")
      print("s")

def pista(numPiste):
  for _ in range(numPiste):
    x = random.randint(0, bound)
    y = random.randint(0, bound)
    leng = random.randint(1, int(math.sqrt(bound)))
    print(f"t {x} {y}", end="")
    for _ in range(leng):
      d = dirs[random.randint(0, len(dirs)-1)]
      print(f" {d}", end="")
    print()

def lung(numLung):
  for i in range(numLung):
    x1 = random.randint(0, bound)
    y1 = random.randint(0, bound)
    if i % 2 == 0:
      x2 = random.randint(max(0, x1-100), x1+100)
      y2 = random.randint(max(0, y1-100), y1+100)
    else:
      x2 = random.randint(0, bound)
      y2 = random.randint(0, bound)
    print(f"L {x1} {y1} {x2} {y2}")

colors = ["red", "yellow", "green", "blue", "black", "white", "purple"]
dirs = ["NN", "SS", "WW", "EE", "NE", "NO", "SE", "SO"]
bound = 1000

# genera piastrelle
generaPiastrelle(500_000)
stampaTutti()

# spegni un po' di piastrelle
spegni(500)
stampaTutti()

# blocchi
blocco(1_000)

# regole e propagazione
generaRegole(1_000)
propaga(1_000, forseOrdina=True)
stampaTutti()

# pista(1_000)
# lung(10_000)
