//plugins {
//  id("com.google.gms.google-services") version "4.4.0" apply false
//}

buildscript {
  repositories {
    google()
    mavenCentral()
  }
  dependencies {
    classpath("com.android.tools.build:gradle:8.2.1")
    classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.22")
    classpath("com.google.gms:google-services:4.4.0")
  }
}

allprojects {
  repositories {
    google()
    mavenCentral()
  }
}

tasks.register("clean").configure {
  delete("build")
}

