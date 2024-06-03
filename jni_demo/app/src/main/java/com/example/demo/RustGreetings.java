package com.example.demo;

public class RustGreetings {
    static {
        System.loadLibrary("android_rust_sample");
    }

    private static native String greeting(final String pattern,
                                         RustResponse rust_response);

    public static class RustResponse{
        public int code;
        public String message;
        public byte[] result;
    }

    public String sayHello(String to, RustResponse rust_response){
        return greeting(to, rust_response);
    }
}
