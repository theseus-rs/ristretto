#!/usr/bin/env sh
javac -source 8 -target 8 *.java
jar --create --verbose --file classes.jar --main-class HelloWorld *.class
