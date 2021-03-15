 # Hypervisor
 * **Event:** UTCTF
 * **Problem Type:** Pwn
 * **Point Value / Difficulty:** ???
 * **(Optional) Tools Required / Used:** 
 ## Solution

This problem invovles analysing cache probe vectors and extracting the key. The algorithm's control flow is determined by the key. By training a random forest classifier on the lab cache vectors and then running the classifier on the victim cache list We can get the list of function calls in the algorithm. Using our knowledge of the algorithm we can then decode it get the key

```
import pandas as pd
from sklearn.ensemble import RandomForestRegressor
from sklearn.preprocessing import LabelEncoder
from sklearn.preprocessing import OneHotEncoder


df = pd.read_csv("lab.csv",header=0)
forest_model = RandomForestRegressor()
y = LabelEncoder().fit_transform(df.iloc[:,-1:])
x = df.iloc[:,0:64]
forest_model.fit(x, y)


keyx = pd.read_csv("target.csv",header=None)


pred = [ int(round(i)) for i in forest_model.predict(keyx)]


i = 0
key = []
while i+1 < len(pred):
    if pred[i] == 2 and pred[i+1] == 0:
        key += [1]
        i+= 2
    else:
        key += [0]
        i+= 3

print(sol)


```