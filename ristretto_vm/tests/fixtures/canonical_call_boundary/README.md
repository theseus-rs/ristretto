# Scalar-only called-target rejection

`generate.py` creates genuine javac Java 17 classfiles. An 11,000-increment
conditional body forces javac to emit `goto_w` while remaining within the JVM
method-size limit. The source generator is kept instead of 11,000 repeated lines.

The scalar entry dispatcher supports that instruction. The object/initializer
frame dispatcher at this stage does not. The test keeps the standalone scalar
entry admitted and rejects a closed program whose initializer calls it, before
heap creation. Before the admission guard, the same real Java initializer was
admitted but failed during execution with `Unsupported(Goto_w)`.

Run `python3 generate.py --check` with JDK 17+ `javac` and `java` on PATH to compare
both classfiles and the three-row HotSpot oracle. The generated peer driver is
never part of the admitted program.
