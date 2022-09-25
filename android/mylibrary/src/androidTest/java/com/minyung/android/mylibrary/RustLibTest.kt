package com.minyung.android.mylibrary

import org.junit.Assert.assertEquals
import org.junit.Test

class RustLibTest {
    @Test
    fun getTestString() {
        assertEquals(
            "Hello, minyung 1234",
            RustLib.getHelloString("minyung", 1234)
        )
        assertEquals(
            "Hello, my 99999",
            RustLib.getHelloString("my", 99999)
        )
    }
}
