# Mini Excel

A CLI tool build with Rust to parse excel like files

## Usage

```console
$ cargo build --release
$ ./target/release/mini-excel <input>
```

## Simple Examples

### Example 1 (Evaluating Different Expressions)

input:
```
123|456
=-1
=1 + 2 + 3
=((10 * 5) + (20 / 4)) - ((8 + 2) * 3)
=random()+8+2+2
=randbetween(20, 30) 
=sum(1, 2, 3, 4)
=average(15, 18, 2, 36, 12, 78, 5, 6, 9)
=max(15, 18, 2, 36, 12, 78, 5, 6, 9)
=min(15, 18, 2, 36, 12, 78, 5, 6, 9)
=if(1, 10, 20)
=10+2
```

output:
```
123       |456       |
-1        |
6         |
25        |
625802100 |
20.124908 |
10        |
20.11111  |
78        |
2         |
10        |
12        |
```

### Example 2 (Referencing Cells)

input:
```
1
=A0
=B0
=B0
=D0+C0
```

output:
```
1         |
1         |
1         |
1         |
2         |
```

### Example 3 (Loop Detection)

input:
```
1
=D0 + 1
=B0 + 1
=C0 + 1
```

output:
```console
$ Cycle detected, "A0 -> C0 -> B0 -> A0"
```
