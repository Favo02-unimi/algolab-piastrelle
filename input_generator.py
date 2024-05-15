import random
import math

piastrelle = 500_000
bound = 500

colors = 'qwertyuiopasdfghjklzxcvbnm'

for i in range(piastrelle):
  x = random.randint(-bound, bound)
  y = random.randint(-bound, bound)
  c = colors[random.randint(0, 25)]
  print(f"C {x} {y} {c}")

for i in range(int(math.sqrt(piastrelle))):
  x = random.randint(-bound, bound)
  y = random.randint(-bound, bound)
  print(f"b {x} {y} {c}")
