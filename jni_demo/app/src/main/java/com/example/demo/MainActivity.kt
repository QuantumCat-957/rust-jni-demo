package com.example.demo

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.example.demo.ui.theme.DemoTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            DemoTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    val rustGreetings = RustGreetings()
                    val response = RustGreetings.RustResponse()
                    val name = rustGreetings.sayHello("Android", response);
                    println("Response Code: ${response.code}")
                    println("Response Message: ${response.message}")
                    println("Response Result: ${response.result?.contentToString()}")
                    Greeting(
                            name = name,
                            modifier = Modifier.padding(innerPadding)
                    )
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
            text = "$name!",
            modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    DemoTheme {
        Greeting("Android")
    }
}