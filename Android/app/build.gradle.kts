plugins {
    id("com.android.application")
    id("org.mozilla.rust-android-gradle.rust-android")
}

android {
    namespace = "com.hk416.android_lab_project_rs_00.sample_application"
    compileSdk = 34
    ndkVersion = "26.2.11394342"

    defaultConfig {
        applicationId = "com.hk416.android_lab_project_rs_00.sample_application"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    buildFeatures {
        viewBinding = true
    }
    sourceSets {
        getByName("androidTest") {
            jniLibs.srcDir("$buildDir/rustJniLibs/android")
        }
        getByName("debug") {
            jniLibs.srcDir("$buildDir/rustJniLibs/android")
        }
    }
}

cargo {
    pythonCommand = "python3"
    verbose = true
    module  = "../../lib"
    libname = "wgpu_app"
    targets = listOf("arm64")
}

tasks.whenTaskAdded {
    if (name == "javaPreCompileDebug" || name == "javaPreCompileRelease") {
        dependsOn("cargoBuild")
    }
}

dependencies {
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")

    implementation("androidx.appcompat:appcompat:1.6.1")

    // To use the Games Activity library
    implementation("androidx.games:games-activity:2.0.2")
}