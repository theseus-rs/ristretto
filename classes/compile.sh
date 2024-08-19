#!/usr/bin/env sh
javac -source 21 -target 21 *.java
jar --create --verbose --file classes.jar --main-class HelloWorld *.class
