import random

def generaPiastrelle(numPiastrelle):
  for _ in range(numPiastrelle):
    x = random.randint(-bound, bound)
    y = random.randint(-bound, bound)
    c = colors[random.randint(0, len(colors)-1)]
    print(f"C {x} {y} {c}")

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

def blocco(numBlocchi):
  for _ in range(numBlocchi):
    x = random.randint(-bound, bound)
    y = random.randint(-bound, bound)
    print(f"b {x} {y}") # blocco
    print(f"B {x} {y}") # omogeneo

def propaga(numPropagazioni):
  print("s")
  for i in range(numPropagazioni):
    x = random.randint(-bound, bound)
    y = random.randint(-bound, bound)
    if i % 2 == 0:
      print(f"p {x} {y}")
    else:
      print(f"P {x} {y}")
    # stampa e ordina le regole per vedere utilizzo
    print("o")
    print("s")

colors = ["q", "w", "e", "r"]
bound = 500

generaPiastrelle(500_000)
generaRegole(1_000)
blocco(1_000)
propaga(1_000)
