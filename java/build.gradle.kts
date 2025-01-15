plugins {
    `java-library`
    `maven-publish`
    id("dev.sigstore.sign") version "1.2.0"
    id("com.diffplug.spotless") version "7.0.2"
    `signing`
}

description = "Sigstore protobuf spec protos bundled into a jar"

repositories {
    mavenCentral()
}

sourceSets {
    main {
        resources {
            srcDirs("../protos")
        }
    }
}

// gradle reproducible jar builds
tasks.withType<AbstractArchiveTask>().configureEach {
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}

java {
    withJavadocJar()
    withSourcesJar()
}

spotless {
    kotlinGradle {
        target("*.gradle.kts") // default target for kotlinGradle
        ktlint()
    }
    format("misc") {
        target("*.md", ".gitignore", "**/*.yaml")

        trimTrailingWhitespace()
        indentWithSpaces()
        endWithNewline()
    }
    // we have no non-generated java code
}

val repoUrl = "https://github.com/sigstore/protobuf-specs"

publishing {
    publications {
        create<MavenPublication>("proto") {

            artifactId = project.name
            from(components["java"])

            pom {
                name.set(
                    (project.findProperty("artifact.name") as? String)
                        ?: project.name,
                )
                description.set(
                    project.provider { project.description },
                )
                inceptionYear.set("2022")
                url.set(repoUrl)
                organization {
                    name.set("Sigstore")
                    url.set("https://sigstore.dev")
                }
                developers {
                    developer {
                        organization.set("Sigstore authors")
                        organizationUrl.set("https://sigstore.dev")
                    }
                }
                issueManagement {
                    system.set("GitHub Issues")
                    url.set("$repoUrl/issues")
                }
                licenses {
                    license {
                        name.set("Apache-2.0")
                        url.set("https://www.apache.org/licenses/LICENSE-2.0.txt")
                    }
                }
                scm {
                    connection.set("scm:git:$repoUrl.git")
                    developerConnection.set("scm:git:$repoUrl.git")
                    url.set(repoUrl)
                    tag.set("HEAD")
                }
            }
        }
    }
    repositories {
        maven {
            name = "releaseStaging"
            url = uri(layout.buildDirectory.dir("releaseStaging"))
        }
    }
}

tasks.register<Jar>("createReleaseBundle") {
    dependsOn("publishProtoPublicationToReleaseStagingRepository")
    from(layout.buildDirectory.dir("releaseStaging/dev/sigstore/protobuf-specs/$version")) {
        include("*.jar")
        include("*.pom")
        include("*.module")
        include("*.sigstore")
        include("*.sigstore.json")
        include("*.asc")
    }
    archiveFileName = "${project.name}-${project.version}-bundle.jar"
}

signing {
    val signingKey: String? by project
    val signingPassword: String? by project
    useInMemoryPgpKeys(signingKey, signingPassword)
    sign(publishing.publications["proto"])
}

tasks.withType<Sign>().configureEach {
    onlyIf("Is a release") {
        project.hasProperty("release")
    }
    onlyIf("PGP Signing is not skipped") {
        !project.hasProperty("skipPgpSigning")
    }
}

tasks.withType<dev.sigstore.sign.tasks.SigstoreSignFilesTask>().configureEach {
    onlyIf("Is a release") {
        project.hasProperty("release")
    }
    onlyIf("Sigstore Signing is not skipped") {
        !project.hasProperty("skipSigstoreSigning")
    }
}
