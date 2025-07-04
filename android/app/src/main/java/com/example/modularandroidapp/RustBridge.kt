package com.example.modularandroidapp

import android.util.Log
import java.io.File

class RustBridge {
    companion object {
        private const val TAG = "RustBridge"
        private const val LIBRARY_NAME = "mainlogic"
        
        init {
            try {
                System.loadLibrary(LIBRARY_NAME)
                Log.d(TAG, "Rust library loaded successfully")
            } catch (e: UnsatisfiedLinkError) {
                Log.e(TAG, "Failed to load Rust library", e)
                throw e
            }
        }
    }

    // Native method declarations
    external fun rustSum(a: Int, b: Int): Int
    external fun rustNotifyJava(message: String)
    external fun rustAsyncCallback(delayMs: Long)
    external fun rustGetVersion(): String
    external fun rustProcessData(input: String): String
    
    // New native activity methods
    external fun initializeNativeActivity(): Boolean
    external fun sendTestNotification()
    external fun updateStatus(message: String)
    external fun getSensorData(): String

    // Kotlin wrapper methods
    fun testSum(a: Int, b: Int): Int {
        return try {
            rustSum(a, b)
        } catch (e: Exception) {
            Log.e(TAG, "Error calling rustSum", e)
            -1
        }
    }

    fun testAsyncCallback() {
        try {
            rustAsyncCallback(2000) // 2 second delay
        } catch (e: Exception) {
            Log.e(TAG, "Error calling rustAsyncCallback", e)
        }
    }

    fun getVersion(): String {
        return try {
            rustGetVersion()
        } catch (e: Exception) {
            Log.e(TAG, "Error getting version", e)
            "Unknown"
        }
    }

    fun processData(input: String): String {
        return try {
            rustProcessData(input)
        } catch (e: Exception) {
            Log.e(TAG, "Error processing data", e)
            "Error processing data"
        }
    }
    
    // New wrapper methods for native activity
    fun initializeNativeActivityWrapper(): Boolean {
        return try {
            initializeNativeActivity()
        } catch (e: Exception) {
            Log.e(TAG, "Error initializing native activity", e)
            false
        }
    }
    
    fun sendTestNotificationWrapper() {
        try {
            sendTestNotification()
        } catch (e: Exception) {
            Log.e(TAG, "Error sending test notification", e)
        }
    }
    
    fun updateStatusWrapper(message: String) {
        try {
            updateStatus(message)
        } catch (e: Exception) {
            Log.e(TAG, "Error updating status", e)
        }
    }
    
    fun getSensorDataWrapper(): String {
        return try {
            getSensorData()
        } catch (e: Exception) {
            Log.e(TAG, "Error getting sensor data", e)
            "Error getting sensor data"
        }
    }

    // Callback methods that Rust can call
    fun onRustEvent(eventType: String, data: String) {
        Log.d(TAG, "Rust event received: $eventType - $data")
        // This method can be called from Rust via JNI
    }

    fun onRustAsyncResult(result: String) {
        Log.d(TAG, "Rust async result: $result")
        // This method can be called from Rust background threads
    }
    
    // New callback methods for native activity
    fun onSensorData(sensorData: String) {
        Log.d(TAG, "Sensor data received: $sensorData")
        // Handle sensor data from Rust
    }
    
    fun onNotification(id: Int, title: String, message: String) {
        Log.d(TAG, "Notification received: [$id] $title - $message")
        // Handle notification from Rust
    }
    
    fun onStatusUpdate(status: String) {
        Log.d(TAG, "Status update: $status")
        // Handle status update from Rust
    }
} 