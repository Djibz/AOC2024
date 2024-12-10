import re

with open('./1/input.txt', 'r') as input:

  print("Part 1")
  total = 0
  for line in input:
    first = re.findall(r'^[^0-9]*?([0-9])', line)[0]
    last = re.findall(r'([0-9])[^0-9]*?$', line)[0]
    total += int(first+last)

  print(total)

def converts_str(literal):
  match literal:
    case 'one': return '1'
    case 'two': return '2'
    case 'three': return '3'
    case 'four': return '4'
    case 'five': return '5'
    case 'six': return '6'
    case 'seven': return '7'
    case 'eight': return '8'
    case 'nine': return '9'
    case _: return literal

with open('./1/input.txt', 'r') as input:
  print("Part 2")
  total = 0
  numbers = r'one|two|three|four|five|six|seven|eight|nine|[0-9]'
  for line in input:
    print(line)
    first = converts_str(re.findall(fr'^.*?({numbers})', line)[0])
    last = converts_str(re.findall(fr'^.*({numbers}).*?$', line)[0])
    print(first)
    print(last)
    total += int(first+last)
  
  print(total)