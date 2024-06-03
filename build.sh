
cd android_rust_sample
cargo ndk -t armeabi-v7a -t arm64-v8a -o ../libs/android/jniLibs build --release

cd ..

cp -r ./libs/android/jniLibs ./jni_demo/app/src/main/