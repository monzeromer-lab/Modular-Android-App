package com.example.modularandroidapp

import android.os.Bundle
import android.util.Log
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import com.example.modularandroidapp.databinding.ActivityMainBinding
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding
    private lateinit var libraryManager: LibraryManager
    private lateinit var rustBridge: RustBridge
    private lateinit var notificationService: NotificationService

    companion object {
        private const val TAG = "MainActivity"
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setupUI()
        initializeComponents()
    }

    private fun setupUI() {
        binding.btnTestRust.setOnClickListener {
            testRustFunction()
        }

        binding.btnUpdateLibrary.setOnClickListener {
            updateLibrary()
        }

        binding.btnTestAsync.setOnClickListener {
            testAsyncCallback()
        }

        binding.btnCheckStatus.setOnClickListener {
            checkLibraryStatus()
        }
        
        // New buttons for native activity
        binding.btnInitializeNative.setOnClickListener {
            initializeNativeActivity()
        }
        
        binding.btnSendNotification.setOnClickListener {
            sendTestNotification()
        }
        
        binding.btnUpdateStatus.setOnClickListener {
            updateStatus()
        }
        
        binding.btnGetSensorData.setOnClickListener {
            getSensorData()
        }
    }

    private fun initializeComponents() {
        libraryManager = LibraryManager(this)
        rustBridge = RustBridge()
        notificationService = NotificationService(this)

        lifecycleScope.launch {
            try {
                val success = libraryManager.initializeLibrary()
                if (success) {
                    Log.d(TAG, "Library initialized successfully")
                    updateStatus("Library loaded successfully")
                } else {
                    Log.e(TAG, "Failed to initialize library")
                    updateStatus("Failed to load library")
                }
            } catch (e: Exception) {
                Log.e(TAG, "Error initializing library", e)
                updateStatus("Error: ${e.message}")
            }
        }
    }

    private fun testRustFunction() {
        lifecycleScope.launch {
            try {
                val result = withContext(Dispatchers.IO) {
                    rustBridge.testSum(10, 20)
                }
                Log.d(TAG, "Rust sum result: $result")
                updateStatus("Rust sum: 10 + 20 = $result")
            } catch (e: Exception) {
                Log.e(TAG, "Error calling Rust function", e)
                updateStatus("Error calling Rust: ${e.message}")
            }
        }
    }

    private fun updateLibrary() {
        lifecycleScope.launch {
            try {
                updateStatus("Starting library update...")
                val success = libraryManager.updateLibrary()
                if (success) {
                    updateStatus("Library updated successfully")
                    // Reinitialize after update
                    initializeComponents()
                } else {
                    updateStatus("Library update failed")
                }
            } catch (e: Exception) {
                Log.e(TAG, "Error updating library", e)
                updateStatus("Update error: ${e.message}")
            }
        }
    }

    private fun testAsyncCallback() {
        lifecycleScope.launch {
            try {
                updateStatus("Testing async callback...")
                withContext(Dispatchers.IO) {
                    rustBridge.testAsyncCallback()
                }
            } catch (e: Exception) {
                Log.e(TAG, "Error testing async callback", e)
                updateStatus("Async test error: ${e.message}")
            }
        }
    }

    private fun checkLibraryStatus() {
        lifecycleScope.launch {
            try {
                val status = libraryManager.getLibraryStatus()
                updateStatus("Library status: $status")
            } catch (e: Exception) {
                Log.e(TAG, "Error checking library status", e)
                updateStatus("Status check error: ${e.message}")
            }
        }
    }
    
    // New methods for native activity
    private fun initializeNativeActivity() {
        lifecycleScope.launch {
            try {
                updateStatus("Initializing native activity...")
                val success = withContext(Dispatchers.IO) {
                    rustBridge.initializeNativeActivityWrapper()
                }
                if (success) {
                    updateStatus("Native activity initialized successfully")
                    Toast.makeText(this@MainActivity, "Native activity ready!", Toast.LENGTH_SHORT).show()
                } else {
                    updateStatus("Failed to initialize native activity")
                }
            } catch (e: Exception) {
                Log.e(TAG, "Error initializing native activity", e)
                updateStatus("Native activity error: ${e.message}")
            }
        }
    }
    
    private fun sendTestNotification() {
        lifecycleScope.launch {
            try {
                updateStatus("Sending test notification...")
                withContext(Dispatchers.IO) {
                    rustBridge.sendTestNotificationWrapper()
                }
                updateStatus("Test notification sent")
            } catch (e: Exception) {
                Log.e(TAG, "Error sending test notification", e)
                updateStatus("Notification error: ${e.message}")
            }
        }
    }
    
    private fun updateStatus() {
        lifecycleScope.launch {
            try {
                val message = "Status updated at ${System.currentTimeMillis()}"
                withContext(Dispatchers.IO) {
                    rustBridge.updateStatusWrapper(message)
                }
                updateStatus("Status updated via Rust")
            } catch (e: Exception) {
                Log.e(TAG, "Error updating status", e)
                updateStatus("Status update error: ${e.message}")
            }
        }
    }
    
    private fun getSensorData() {
        lifecycleScope.launch {
            try {
                updateStatus("Getting sensor data...")
                val sensorData = withContext(Dispatchers.IO) {
                    rustBridge.getSensorDataWrapper()
                }
                updateStatus("Sensor data: $sensorData")
            } catch (e: Exception) {
                Log.e(TAG, "Error getting sensor data", e)
                updateStatus("Sensor data error: ${e.message}")
            }
        }
    }

    private fun updateStatus(message: String) {
        runOnUiThread {
            binding.tvStatus.text = message
            Log.d(TAG, message)
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        libraryManager.cleanup()
    }
} 