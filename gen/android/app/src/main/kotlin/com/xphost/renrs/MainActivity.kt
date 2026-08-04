package com.xphost.renrs

import android.os.Bundle

class MainActivity : WryActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        initSysdirs(filesDir.absolutePath)
    }
    external fun initSysdirs(filesDir: String)
}