package com.minyung.android.mylibrary

class RustLib {
    companion object {
        init {
            System.loadLibrary("rustlib")
        }

        external fun printTestLog()
        external fun getHelloString(name: String, number: Int): String
    }

    fun getTestString(): String = "Test String"
}
