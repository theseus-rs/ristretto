#!/usr/bin/env sh
javac -source 21 -target 21 *.java
jar cvf classes.jar *.class