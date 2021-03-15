import pandas as pd
from sklearn.ensemble import RandomForestRegressor
from sklearn.preprocessing import LabelEncoder
from sklearn.preprocessing import OneHotEncoder


df = pd.read_csv("lab.csv",header=None)
forest_model = RandomForestRegressor()
y = LabelEncoder().fit_transform(df.iloc[:,-1:])

x = df.iloc[:,0:64]
forest_model.fit(x, y)


keyx = pd.read_csv("target.csv",header=None)


pred = [ int(round(i)) for i in forest_model.predict(keyx)]


i = 0
sol = []
while i < len(pred):
    if pred[i] == 2 and pred[i+1] == 0:
        sol += [1]
        i+= 2
    else:
        sol += [0]
        i+= 3


key = [1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1]

sol.reverse()
for i in sol:
    print(i,end="")
print()

"1010101100010111011101000010110110011110010110101101110100011111"
"101010011101000100010111101001001100001101001010010001011011111"
"101010011101000100010111101001001100001101001010010001011011111"
"101010011101000100010111101001001100001101001010010001011011111"

z = [2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 2, 0, 1, 2, 0, 2, 0, 2, 0, 2, 0, 1, 2, 0, 2, 0, 2, 0, 1, 2, 0, 2, 0, 1, 2, 0, 2, 0, 2, 0, 1, 2, 0, 2, 0, 1, 2, 0, 1, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 1, 2, 0, 1, 2, 0, 2, 0, 2, 0, 1, 2, 0, 2, 0, 2, 0, 1, 2, 0, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 2, 0, 1, 2, 0, 2, 0, 2, 0, 2, 0, 1, 2, 0, 2, 0, 2, 0, 2, 0, 1, 2, 0, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 2, 0, 2, 0, 1, 2, 0, 2, 0, 1, 2, 0, 2, 0, 1, 2, 0, 2, 0]


s = 'pdpdpdpdpdmpdmpdmpdpdmpdpdpdpdmpdpdpdmpdpdmpdpdpdmpdpdmpdmpdpdpdpdpdmpdmpdpdpdmpdpdpdmpdpdmpdmpdmpdmpdpdmpdpdpdpdmpdpdpdpdmpdpdmpdmpdmpdpdpdmpdpdmpdpdmpdpd'

res = []
for q in s:
    if q == 'p':
        res.append(2)
    elif q == 'm':
        res.append(1)
    else:
        res.append(0)


print(res == pred)

z = []

def solve(loc):
    global s
    global z
    if loc == len(s):
        return 1
    if s[loc] == 'p' and s[loc+1] == 'd': # pd
        z += [1]
        return 2 * solve(loc + 2) + 1 
    else: # mpd
        z += [0]
        return 2 * solve(loc + 3) + 2


q = solve(0)


print(sol)
z.reverse()
print(z)
print(sol == z)
print(q)
print(int("".join(str(i) for i in sol),2))
sol.reverse()

print(int("".join(str(i) for i in sol),2))