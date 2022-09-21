package com.minyung.android.sample

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import com.minyung.android.mylibrary.RustLib

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        RustLib.printTestLog()

        val textView: TextView = findViewById(R.id.text)
        textView.text = RustLib.getHelloString("minyung", 2022)
    }
}
