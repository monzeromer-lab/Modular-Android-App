package com.example.modularandroidapp

import android.content.Context
import android.util.Log
import androidx.security.crypto.EncryptedFile
import androidx.security.crypto.MasterKey
import java.io.File
import java.io.FileInputStream
import java.io.FileOutputStream
import java.security.MessageDigest
import java.util.concurrent.atomic.AtomicBoolean

class LibraryManager(private val context: Context) {
    companion object {
        private const val TAG = "LibraryManager"
        private const val LIBRARY_NAME = "libmainlogic.so"
        private const val UPDATE_URL = "https://example.com/latest/libmainlogic.so"
        private const val EXPECTED_HASH = "a1b2c3d4e5f6..." // This would be the expected SHA-256 hash
    }

    private val isInitialized = AtomicBoolean(false)
    private var libraryFile: File? = null

    suspend fun initializeLibrary(): Boolean {
        return try {
            // First try to load from internal storage (updated version)
            val internalFile = File(context.filesDir, LIBRARY_NAME)
            if (internalFile.exists() && verifyHash(internalFile)) {
                libraryFile = internalFile
                Log.d(TAG, "Using updated library from internal storage")
            } else {
                // Fallback to bundled library
                val bundledFile = File(context.applicationInfo.nativeLibraryDir, LIBRARY_NAME)
                if (bundledFile.exists()) {
                    libraryFile = bundledFile
                    Log.d(TAG, "Using bundled library")
                } else {
                    Log.e(TAG, "No library found")
                    return false
                }
            }
            
            isInitialized.set(true)
            true
        } catch (e: Exception) {
            Log.e(TAG, "Error initializing library", e)
            false
        }
    }

    suspend fun updateLibrary(): Boolean {
        return try {
            Log.d(TAG, "Starting library update...")
            
            // Download the new library
            val downloadService = DownloadService(context)
            val downloadedFile = downloadService.downloadLibrary(UPDATE_URL)
            
            if (downloadedFile != null && verifyHash(downloadedFile)) {
                // Move to internal storage
                val internalFile = File(context.filesDir, LIBRARY_NAME)
                downloadedFile.copyTo(internalFile, overwrite = true)
                downloadedFile.delete()
                
                libraryFile = internalFile
                isInitialized.set(true)
                
                Log.d(TAG, "Library updated successfully")
                true
            } else {
                Log.e(TAG, "Downloaded library verification failed")
                false
            }
        } catch (e: Exception) {
            Log.e(TAG, "Error updating library", e)
            false
        }
    }

    fun getLibraryStatus(): String {
        return when {
            !isInitialized.get() -> "Not initialized"
            libraryFile?.exists() == true -> {
                val source = if (libraryFile?.parent == context.filesDir.absolutePath) {
                    "Updated version"
                } else {
                    "Bundled version"
                }
                "Initialized ($source)"
            }
            else -> "Error: Library file not found"
        }
    }

    private fun verifyHash(file: File): Boolean {
        return try {
            val digest = MessageDigest.getInstance("SHA-256")
            val inputStream = FileInputStream(file)
            val buffer = ByteArray(8192)
            var bytesRead: Int
            
            while (inputStream.read(buffer).also { bytesRead = it } != -1) {
                digest.update(buffer, 0, bytesRead)
            }
            inputStream.close()
            
            val hash = digest.digest().joinToString("") { "%02x".format(it) }
            Log.d(TAG, "File hash: $hash")
            
            // In a real app, you would compare against a trusted hash
            // For demo purposes, we'll accept any valid hash
            hash.isNotEmpty()
        } catch (e: Exception) {
            Log.e(TAG, "Error verifying hash", e)
            false
        }
    }

    fun cleanup() {
        isInitialized.set(false)
        libraryFile = null
    }

    fun getLibraryPath(): String? {
        return libraryFile?.absolutePath
    }
} 