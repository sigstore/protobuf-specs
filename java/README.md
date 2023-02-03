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
1. On creation of a tag in the style `release/java/v1.2.3`, new artifacts will be built and
uploaded to a github release `release/java/v1.2.3`
1. Once a release is created, check it and remove the draft label on the github release page.
1. On a machine with your pgp key, `gpg`, `bash` and `cosign`, go to `protobuf-specs/java/scripts`
1. Run `./sign_and_bundle_release.sh v1.2.3` to generate a release bundle for `release/java/v1.2.3`

### Publish on Maven Central
1. Log into https://s01.oss.sonatype.org with credentials that have permissions to upload to `dev.sigstore`
1. Take the release bundle, `release_java_v1.2.3/protobuf-specs-1.2.3-bundle.jar` and upload via the `Staging Upload -> (Upload Mode) Artifact Bundle`
1. Once the bundle is validated and checked, release it via `Staging Repositories`, if any issues occur, drop it and fix the issues before restarting the release process.

## How do I get permissions to upload to Maven Central

- Create an account: https://central.sonatype.org/publish/publish-guide/
- Request permissions to publish to dev.sigstore on JIRA ([example](https://issues.sonatype.org/browse/OSSRH-83556)) and get [Bob](https://github.com/bobcallaway) (or [Appu](https://github.com/loosebazooka) to signoff on it.

## Why is the gradle wrapper jar checked in?

The file `gradle-wrapper.jar` is usually checked into java projects that are built with gradle.
This file is validated by the gradle/wrapper-validation-action in the java-build.yml workflow.
More info at: https://github.com/gradle/wrapper-validation-action
