# Savvy Examples - Matrix Handling


This repository is an example to show how to deal with matrices with
[the savvy framework](https://yutannihilation.github.io/savvy/guide/).

## ndarray

[code](https://github.com/yutannihilation/savvy-matrix-examples/blob/master/src/rust/src/ndarray.rs)

``` r
m <- rbind(
  c(1.0, 3.0, 5.0),
  c(2.0, 4.0, 6.0)
)

ndarray_input(m)
```

    Ok([[1.0, 3.0, 5.0],
     [2.0, 4.0, 6.0]], shape=[2, 3], strides=[1, 2], layout=Ff (0xa), dynamic ndim=2)

``` r
ndarray_output()
```

         [,1] [,2] [,3]
    [1,]    1    3    5
    [2,]    2    4    6

## nalgebra

[code](https://github.com/yutannihilation/savvy-matrix-examples/blob/master/src/rust/src/nalgebra.rs)

``` r
m <- rbind(
  c(1.0, 3.0, 5.0),
  c(2.0, 4.0, 6.0)
)

nalgebra_input(m)
```

    VecStorage { data: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0], nrows: Dyn(2), ncols: Dyn(3) }

``` r
nalgebra_output()
```

         [,1] [,2] [,3]
    [1,]    1    3    5
    [2,]    2    4    6

## glam-rs

[code](https://github.com/yutannihilation/savvy-matrix-examples/blob/master/src/rust/src/glam.rs)

``` r
m <- as.numeric(1:9)
dim(m) <- c(3, 3)

glam_input(m)
```

    DMat3 { x_axis: DVec3(1.0, 2.0, 3.0), y_axis: DVec3(4.0, 5.0, 6.0), z_axis: DVec3(7.0, 8.0, 9.0) }

``` r
glam_output()
```

         [,1] [,2] [,3]
    [1,]    1    4    7
    [2,]    2    5    8
    [3,]    3    6    9
