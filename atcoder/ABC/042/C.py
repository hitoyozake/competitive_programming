s = input().split(" ")
n = int(s[0])
numbers = [ i for i in input() if i != " "]
 
for i in range(n, 10001):
  found = False
  snum = str(i)
  for num in numbers:
    if num in snum:
      found = True
      break
  
  if found == False:
     print(i)
     break