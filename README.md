## Quaternion

Numeric Quaternion type - 4D hyper complex numbers [1-6].

References
----------

[1] [The rotation problem and Hamilton's discovery of quaternions I | Famous Math Problems 13a](https://youtu.be/uRKZnFAR7yw)

[2] [The rotation problem and Hamilton's discovery of quaternions (II) | Famous Math Problems 13b](https://youtu.be/0_XoZc-A1HU)

[3] [The rotation problem and Hamilton's discovery of quaternions III | Famous Math Problems 13c](https://youtu.be/g22jAtg3QAk)

[4] [The rotation problem and Hamilton's discovery of quaternions IV | Famous Math Problems 13d](https://youtu.be/MkNfQtINEjo)

[5] [Quaternions - UC Davis](https://youtu.be/mHVwd8gYLnI)

[6] "Hypercomplex Numbers An Elementary Introduction to Algebras" by I.L. Kantor A.S. Solodovnikov


Run
-----

```
% cargo run

q1 = 3.06 +1i +1j +2k
q2 = 0.7 +3i -1j +2k
q1 + q2 = 3.76 +4i +0j +4k
q1 - q2 = 2.36 -2i +2j +0k
q1 * q2 = -3.858 +13.88i +1.64j +3.52k
q2 * q1 = -3.858 +5.88i -6.36j +11.52k
q1 / q2 = 0.5619 -0.8613i -0.0166j -0.0497k
q1 * q1.inverse() = 1 +0i +0j +0k
q2 * q2.inverse() = 1 +0i +0j +0k
Magnitude of q1 = 3.919642840872112
Conjugate of q1 = 3.06 -1i -1j -2k
```

