# Types

* Destructing means breaking part a complex type and assigning values of its members to new variables in signle line.
```rs
let Point { x: left_edge, y: top_edge } = point;
```
This line create two new variable `left_edge` and `top_edge` and assign these values of `point.x` and `point.y`.
It works in function arguments as well.
```rs
fn print_coords((x, y): (i32, i32)) {
    println!("x = {}, y = {}", x, y);
}

```
This functions take one argument a `tuple` and immidiately destructs it into two variables `x` and `y`.

## Structs
In first example we are calculating area of `Rectangle` using function `react_area` using nested destruct.
In 2nd example we write a function `square` which takes `point` and `f32` argument and returns a rectangle with its top left left at `point` and height and width corrosponds to `f32`.