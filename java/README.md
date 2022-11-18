# Java generator

This directory contains the necessary build config for java code generation. The gradle
build takes the protos defined in `../protos` and using the grade protobug plugin will
generate a single jar.

To generate a jar from the protobuf spec run
```
./gradlew assemble
```
A jar file will be created at `./build/libs/protobuf-specs-<version>.jar`

## Releasing

The generate jar can be released to maven central by TODO

## Why is the gradle wrapper jar checked in?

The file `gradle-wrapper.jar` is usually checked into java projects that are built with gradle.
This file is validated by the gradle/wrapper-validation-action in the java-build.yml workflow.
More info at: https://github.com/gradle/wrapper-validation-action
