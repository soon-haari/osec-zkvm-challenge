p = 2^31 - 1
Fp = GF(p)

# i^2 = -1
R.<X> = PolynomialRing(Fp)
CM31.<i> = Fp.extension(X^2 + 1)

# u^2 = 2 + i
S.<Y> = PolynomialRing(CM31)
QM31.<u> = CM31.extension(Y^2 - (CM31(2) + i))

# element format: (a + b*i) + (c + d*i)*u
def qm31(a,b,c,d):
    return CM31(a) + CM31(b)*i + (CM31(c) + CM31(d)*i)*u


tar = (197670581 + 917995853*i) + (1371741767 + 406819140*i)*u
a = (1281752765 + 1564335693*i) + (711495991 + 1476685420*i)*u

b = (1717841229 + 1996471878*i) + (1827963874 + 1588780450*i)*u

d = b - a

ans = (tar - a) / d

print(ans)
exit()

# examples (your two values)
x = qm31(625001828, 119159153, 240333459, 848697795)
y = qm31(1138767216, 39349376, 2047427983, 874150208)

print("x =", x)
print("y =", y)
print("x+y =", x + y)
print("x*y =", x * y)
print("x^-1 =", x^-1)
