#!/usr/bin/env sh
rm *.class
javac -source 8 -target 8 *.java
jar --create --verbose --file classes.jar --main-class HelloWorld *.class
