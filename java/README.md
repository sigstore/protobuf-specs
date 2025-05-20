# Java generator

This directory contains the necessary build config for java code generation. The gradle
build takes the protos defined in `../protos` and using the grade protobug plugin will
generate a single jar.

To generate a jar from the protobuf spec run
```
./gradlew assemble
```
A jar file will be created at `./build/libs/protobuf-specs-SNAPSHOT.jar`

## Releasing

### Generate Release artifacts
1. On creation of a tag in the style `release/java/v1.2.3`, new artifacts will be built signed
   and published to maven central (in staging, but no released).

### Complete Publish Flow Maven Central
1. Log into https://s01.oss.sonatype.org with credentials that have permissions to upload to `dev.sigstore`
1. Find the release in `Staging Repositories`, "close" it and once it passes validation "release" it.

## How do I get permissions to upload to Maven Central

- Create an account: https://central.sonatype.org/publish/publish-guide/
- Request permissions to publish to dev.sigstore on JIRA ([example](https://issues.sonatype.org/browse/OSSRH-83556)) and get [Bob](https://github.com/bobcallaway) (or [Appu](https://github.com/loosebazooka) to signoff on it.

## Why is the gradle wrapper jar checked in?

The file `gradle-wrapper.jar` is usually checked into java projects that are built with gradle.
This file is validated by the gradle/wrapper-validation-action in the gradle-wrapper-validation.yml workflow.
More info at: https://github.com/gradle/wrapper-validation-action
