## Quaternion

Numeric Quaternion type - 4D hyper complex numbers [1,2,3,4].

References
----------

[1] [The rotation problem and Hamilton's discovery of quaternions I | Famous Math Problems 13a](https://youtu.be/uRKZnFAR7yw)

[2] [The rotation problem and Hamilton's discovery of quaternions (II) | Famous Math Problems 13b](https://youtu.be/0_XoZc-A1HU)

[3] [The rotation problem and Hamilton's discovery of quaternions III | Famous Math Problems 13c](https://youtu.be/g22jAtg3QAk)

[4] [The rotation problem and Hamilton's discovery of quaternions IV | Famous Math Problems 13d](https://youtu.be/MkNfQtINEjo)


RUN
-----

```
% cargo run

q1 = Quaternion { w: 3.06, x: 1.0, y: 1.0, z: 2.0 }
q2 = Quaternion { w: 0.7, x: 3.0, y: -1.0, z: 2.0 }
q1 + q2 = Quaternion { w: 3.76, x: 4.0, y: 0.0, z: 4.0 }
q1 - q2 = Quaternion { w: 2.3600000000000003, x: -2.0, y: 2.0, z: 0.0 }
q1 * q2 = Quaternion { w: -3.858, x: 13.879999999999999, y: 1.6399999999999997, z: 3.52 }
q2 * q1 = Quaternion { w: -3.858, x: 5.879999999999999, y: -6.359999999999999, z: 11.52 }
q1 / q2 = Quaternion { w: 0.5619047619047619, x: -0.8612836438923396, y: -0.0165631469979296, z: -0.049689440993788844 }
Magnitude of q1 = 3.919642840872112
Conjugate of q1 = Quaternion { w: 3.06, x: -1.0, y: -1.0, z: -2.0 }
```

