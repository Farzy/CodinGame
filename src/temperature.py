n = int(input())  # the number of temperatures to analyse
if n != 0:
    temp_min = -9999
    for i in input().split():
        # t: a temperature expressed as an integer ranging from -273 to 5526
        t = int(i)
        if abs(t) < abs(temp_min) or (t > 0 and t + temp_min == 0):
            temp_min = t
else:
    temp_min = 0

# Write an answer using print
# To debug: print("Debug messages...", file=sys.stderr, flush=True)

print(temp_min)
