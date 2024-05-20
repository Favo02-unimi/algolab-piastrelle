import random
import math

def generaPiastrelle(numPiastrelle):
  for _ in range(numPiastrelle):
    x = random.randint(-bound, bound)
    y = random.randint(-bound, bound)
    c = colors[random.randint(0, len(colors)-1)]
    i = random.randint(1, 1_000)
    print(f"C {x} {y} {c} {i}")

def spegni(numPiastrelle):
  for _ in range(numPiastrelle):
    x = random.randint(-bound, bound)
    y = random.randint(-bound, bound)
    print(f"S {x} {y}")

def stampaTutti():
  for y in range(-bound, bound+1):
    for x in range(-bound, bound+1):
      print(f"? {x} {y}")

def generaRegole(numRegole):
  for _ in range(numRegole):
    maxx = random.randint(1, 8)
    finalColor = colors[random.randint(0, len(colors)-1)]
    count = 0
    print(f"r {finalColor}", end="")
    while count < maxx:
      coeff = random.randint(0, maxx-count)
      c = colors[random.randint(0, len(colors)-1)]
      print(f" {coeff} {c}", end="")
      count += coeff
    print()
  print("s")

def blocco(numBlocchi):
  for _ in range(numBlocchi):
    x = random.randint(-bound, bound)
    y = random.randint(-bound, bound)
    print(f"b {x} {y}") # blocco
    print(f"B {x} {y}") # omogeneo

def propaga(numPropagazioni, forseOrdina=False):
  for i in range(numPropagazioni):
    x = random.randint(-bound, bound)
    y = random.randint(-bound, bound)
    if i % 2 == 0:
      print(f"p {x} {y}")
    else:
      print(f"P {x} {y}")
    if forseOrdina and random.randint(1, 10) == 1:
      print("o")
      print("s")

def pista(numPiste):
  for _ in range(numPiste):
    x = random.randint(-bound, bound)
    y = random.randint(-bound, bound)
    leng = random.randint(1, int(math.sqrt(bound)))
    print(f"t {x} {y}", end="")
    for _ in range(leng):
      d = dirs[random.randint(0, len(dirs)-1)]
      print(f" {d}", end="")
    print()

def lung(numLung):
  for i in range(numLung):
    x1 = random.randint(-bound, bound)
    y1 = random.randint(-bound, bound)
    if i % 2 == 0:
      x2 = random.randint(x1-100, x1+100)
      y2 = random.randint(y1-100, y1+100)
    else:
      x2 = random.randint(-bound, bound)
      y2 = random.randint(-bound, bound)
    print(f"L {x1} {y1} {x2} {y2}")

colors = ["q", "w", "e", "r"]
dirs = ["NN", "SS", "WW", "EE", "NE", "NO", "SE", "SO"]
bound = 500

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
