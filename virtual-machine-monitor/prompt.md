You've managed to get your attack vm co-resident with a victims vm in a data center. As the crafty attacker that you are, you have decided to prime and probe the instruction set cache to determine the victim vm's secrets. You now have a collection of cache vectors representing the cache usage of the target vm. Your goal is to determine the key being used by the vm. Luckily, your hosting provider is antiquated and the victim VM doesn't alternate between cores so you were able to continously monitor it. You also know that the victim VM uses the exponent by squaring algorithm in cryptographic operations with the key.


```
int mult(int x, int y) { return x * y; }

int div(int x, int y) { return x / y; }

int exp_by_squaring(int x, int n) {
  if (n == 0) {
    return 1;
  }
  int y = 1;
  while (n > 1) {
    if (n % 2 == 1) {
      x = pow(x, 2);
      n = div(n, 2);
    } else {
      y = mult(x, y);
      x = pow(x, 2);
      n = div((n - 1), 2);
    }
    n++;
  }
  return x * y;
}
```

In your lab you ran the same hardware as the target vm with a similar software stack. You used the same prime and probe technique to label cache vectors P,D,M referring to when the functions pow, div, mult where running. Below are the datasets from the machines.

[Lab.csv]()

[Victim.csv]()

The flag is the key used by the victim VM in binary.


Algorithm Pseudocode courtesy of Wikipedia.