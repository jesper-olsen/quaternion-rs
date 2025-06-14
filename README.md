## Quaternion

Numeric Quaternion type - 4D hyper complex numbers [1,2,3,4].

References
----------

[1] [The rotation problem and Hamilton's discovery of quaternions I | Famous Math Problems 13a](https://youtu.be/uRKZnFAR7yw)

[2] [The rotation problem and Hamilton's discovery of quaternions (II) | Famous Math Problems 13b](https://youtu.be/0_XoZc-A1HU)

[3] [The rotation problem and Hamilton's discovery of quaternions III | Famous Math Problems 13c](https://youtu.be/g22jAtg3QAk)

[4] [The rotation problem and Hamilton's discovery of quaternions IV | Famous Math Problems 13d](https://youtu.be/MkNfQtINEjo)

[5] [Quaternions - UC Davis](https://youtu.be/mHVwd8gYLnI)


RUN
-----

```
% cargo run

q1 = Quaternion([3.06, 1.0, 1.0, 2.0])
q2 = Quaternion([0.7, 3.0, -1.0, 2.0])
q1 + q2 = Quaternion([3.76, 4.0, 0.0, 4.0])
q1 - q2 = Quaternion([2.3600000000000003, -2.0, 2.0, 0.0])
q1 * q2 = Quaternion([-3.858, 13.879999999999999, 1.6399999999999997, 3.52])
q2 * q1 = Quaternion([-3.858, 5.879999999999999, -6.359999999999999, 11.52])
q1 / q2 = Quaternion([0.5619047619047619, -0.8612836438923396, -0.0165631469979296, -0.049689440993788844])
Magnitude of q1 = 3.919642840872112
Conjugate of q1 = Quaternion([3.06, -1.0, -1.0, -2.0])
```

