package com.example.demo;

public class RustGreetings {
    static {
        System.loadLibrary("android_rust_sample");
    }

    private static native String greeting(final String pattern);

    public String sayHello(String to){
        return greeting(to);
    }
}
